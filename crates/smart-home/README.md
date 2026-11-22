# Smart Home Rust library

This crate (`keyestudio-smart-home-v2`) contains **portable `#![no_std]` drivers and control helpers** for the Keyestudio Smart Home V2 kit. It does not initialise micro:bit hardware or run an Embassy executor; the application in [`../../apps/microbit-v2`](../../apps/microbit-v2/README.md) does that.

## Modules

| Module | Public API | Status |
| --- | --- | --- |
| `pir.rs` | `Pir<P>`: `motion_detected()`, async state waits and edge waits | Hardware tested on micro:bit v2 |
| `fan.rs` | `Fan<PLUS, MINUS>`, `FanError`, `fan_should_run()` | Hysteresis unit tested; motor not hardware tested |
| `servo.rs` | `servo_pulse_us()` | Unit tested; PWM output not implemented |

`lib.rs` declares these modules and may re-export their main types/functions for convenience.

## Architecture

- The PIR driver's `motion_detected()` requires `embedded_hal::digital::InputPin`.
- Its asynchronous wait methods require `embedded_hal_async::digital::Wait`.
- Separate trait-bounded `impl` blocks let hardware types provide only the operations they support.
- The fan uses `embedded_hal::digital::OutputPin`.
- Servo calculation and fan hysteresis are pure functions and can be tested on a development PC.

Use **synchronous operations for immediate GPIO reads and writes**; use **asynchronous waits for events**. An Embassy application can use both without making the library itself depend on Embassy.

## Development commands

From the repository root:

```bash
cargo test
cargo fmt --all --check
cargo clippy -p keyestudio-smart-home-v2 --all-targets -- -D warnings
```

To explicitly select this package:

```bash
cargo test -p keyestudio-smart-home-v2
```

## Limitations and planned improvements

- The current PIR hardware integration has been tested, but automated tests for its async waits are still planned.
- The fan's output ordering assumes an appropriate external motor driver; GPIO errors require deliberate safety handling.
- `fan_should_run` expects `on_at > off_below` but does not yet validate those thresholds.
- `servo_pulse_us` clamps angles to 0–180° and computes a pulse width; it does not validate calibration ranges or drive PWM hardware.

See the [root README](../../README.md) for the kit pin map and feature roadmap.
