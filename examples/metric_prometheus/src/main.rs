/*!
An example of emitting metrics to Prometheus.

Prometheus has direct support for OTLP: https://prometheus.io/docs/guides/opentelemetry/
*/

use rand::Rng;

fn main() {
    let rt = emit::setup()
        .emit_to(
            emit_otlp::new()
                .resource(emit::props! {
                    #[emit::key("service.name")]
                    service_name: "emit-sample",
                })
                .metrics(emit_otlp::metrics_http_proto(
                    "http://localhost:9090/api/v1/otlp/v1/metrics",
                ))
                .spawn(),
        )
        .init();

    let mut bytes_written = 0usize;
    for _ in 0..60 {
        bytes_written += rand::rng().random_range(0..750);

        emit::emit!(evt: emit::Metric::new(emit::pkg!(), "bytes_written", "count", emit::clock().now(), bytes_written, emit::Empty));

        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    rt.blocking_flush(std::time::Duration::from_secs(5));
}
