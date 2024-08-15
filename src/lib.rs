wit_bindgen::generate!({
    world: "test",
    with: {
        "wasi:io/poll@0.2.0": generate,
        "wasi:io/poll@0.2.1": generate,
        "wasi:clocks/monotonic-clock@0.2.0": generate,
        "wasi:clocks/monotonic-clock@0.2.1": generate,
    }
});

struct MyTest;

impl Guest for MyTest {
    fn test() {
        wasi::io0_2_0::poll::poll(&[&wasi::clocks0_2_0::monotonic_clock::subscribe_duration(
            10_000_000,
        )]);
        wasi::io0_2_1::poll::poll(&[&wasi::clocks0_2_1::monotonic_clock::subscribe_duration(
            10_000_000,
        )]);
    }
}

export!(MyTest);
