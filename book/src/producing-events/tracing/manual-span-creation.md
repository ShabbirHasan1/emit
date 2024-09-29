# Manual span creation

Span events may also be created manually:

```rust
# extern crate emit;
// Create a new span context that is a child of the current one
// This context can be freely copied or stored elsewhere
let ctxt = emit::SpanCtxt::current(emit::ctxt())
    .new_child(emit::rng());

// Push the span onto the current context when you're about to execute
// some code within it
ctxt.push(emit::ctxt())
    .call(move || {
        let timer = emit::Timer::start(emit::clock());

        // Your code goes here
        let sleep_ms = 1200;
        std::thread::sleep(std::time::Duration::from_millis(sleep_ms));

        // Make sure you complete the span in the frame.
        // This is especially important for futures, otherwise the span may
        // complete before the future does
        emit::emit!(
            extent: timer,
            "wait a bit",
            evt_kind: "span",
            sleep_ms,
        );
    });
```

Trace and span ids don't need to be managed by `emit` if you have another scheme in mind. In these cases, they can be attached as regular properties to the span event:

```rust
# extern crate emit;
let timer = emit::Timer::start(emit::clock());

// Your code goes here
let sleep_ms = 1200;
std::thread::sleep(std::time::Duration::from_millis(sleep_ms));

emit::emit! {
    extent: timer,
    "wait a bit",
    evt_kind: "span",
    trace_id: "4bf92f3577b34da6a3ce929d0e0e4736",
    span_id: "00f067aa0ba902b7",
    sleep_ms,
}
```

Keep in mind when emitting spans as regular events that you still thread the trace context around somehow, otherwise other events emitted within its execution won't be correlated with it.
