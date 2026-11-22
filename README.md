# Keyestudio Smart Home V2

Reusable **Rust (`no_std`)** and **MicroPython** drivers and examples for the Keyestudio KS4048/KS4049 Smart Home V2 kit using the **BBC micro:bit v2**.

This is an educational, work-in-progress project. The Rust firmware uses **Embassy** for asynchronous GPIO events; the reusable drivers use the `embedded-hal` / `embedded-hal-async` traits instead of depending directly on a particular microcontroller.

## Repository layout

```text
.
├── Cargo.toml                 # Rust workspace
├── Cargo.lock                 # Workspace dependency lockfile
├── crates/
│   └── smart-home/            # Reusable no_std Rust library
│       └── src/
│           ├── lib.rs
│           ├── pir.rs
│           ├── fan.rs
│           └── servo.rs
├── apps/
│   └── microbit-v2/           # Embassy firmware for nRF52833
│       ├── .cargo/config.toml
│       ├── build.rs
│       ├── memory.x
│       └── src/main.rs
└── python/                    # MicroPython starter library and example
    ├── smart_home.py
    └── example_pir_fan.py
```

Each component has its own README: [Rust library](crates/smart-home/README.md), [Embassy firmware](apps/microbit-v2/README.md), [MicroPython](python/README.md).

## Feature status

| Component / capability | Rust | MicroPython | Hardware verified |
| --- | --- | --- | --- |
| PIR motion sensor | Immediate reads and asynchronous state/edge waits | Digital read | **Rust on micro:bit v2: yes** |
| Two-input fan motor driver | Forward, reverse, stop | Forward, reverse, stop | Not yet |
| Fan temperature hysteresis | Pure control helper; unit tested | Pure control helper | Logic tested in Rust; no hardware test |
| Servo positioning | Pulse-width calculation only; unit tested | PWM positioning | Not yet |
| DHT11 temperature and humidity | Planned | Planned | No |
| Ultrasonic ranging | Planned | Planned | No |
| 4-pixel addressable RGB LEDs | Planned | Planned | No |
| I²C LCD1602 | Planned | Planned | No |

The PIR has been tested on a real micro:bit v2 using Embassy GPIO waits and RTT logs. Other components should not be considered hardware-validated yet. The Rust servo utility **does not generate PWM**; it calculates a pulse width for a future hardware-specific PWM driver.

## Prerequisites (Rust)

- Rust toolchain with Cargo and `rustup`.
- `thumbv7em-none-eabihf` compilation target for the nRF52833 Cortex-M4F.
- `probe-rs` tools and a USB data cable for flashing and RTT logs.
- BBC micro:bit v2 and, for the PIR demo, the Keyestudio Smart Home V2 kit.

```bash
rustup target add thumbv7em-none-eabihf
cargo install probe-rs-tools --locked
probe-rs list
```

On Ubuntu, installing `probe-rs-tools` from source may require system packages such as `pkg-config` and `libudev-dev`. Ensure `$HOME/.cargo/bin` is on your `PATH`.

## Build and test the Rust library

Run these commands from the **repository root** (the workspace defaults to the library):

```bash
cargo test
cargo fmt --all --check
cargo clippy -p keyestudio-smart-home-v2 --all-targets -- -D warnings
```

These are **host-side** tests of portable library code. They do not flash the board or test real peripherals.

## Build, flash and monitor the Embassy application

**Run Cargo from the firmware directory** to load its local `.cargo/config.toml`, which specifies the ARM target, linker arguments and `probe-rs` runner:

```bash
cd apps/microbit-v2
cargo build --release
DEFMT_LOG=info cargo run --release
```

The current application monitors the PIR connected to micro:bit edge connector **P15** (nRF52833 **P0.13**). `probe-rs` displays `defmt` messages via **RTT** in your terminal; the code does not write to USB serial or the micro:bit LED matrix.

For troubleshooting and the hardware connection, see the [firmware README](apps/microbit-v2/README.md).

## Hardware pin reference

Pins below follow the Keyestudio example projects. Confirm the wiring and supply requirements of your particular expansion board before connecting modules.

| Module | micro:bit edge pins |
| --- | --- |
| PIR | P15 |
| Motor driver | IN+ P13, IN− P16 |
| Door servo | P1 |
| Bin servo | P9 |
| RGB LED module (4 pixels) | P14 |
| Ultrasonic sensor | TRIG P12, ECHO P8 |
| DHT11 | P2 (standalone example), P0 (environment-monitoring example) |
| LCD1602 (I²C) | SDA P20, SCL P19 |

**Electrical safety:** Never power a DC motor directly from a micro:bit GPIO. Check voltage compatibility of all digital outputs, especially ultrasonic ECHO; micro:bit GPIO is **3.3 V logic**. Calibrate servos before attaching mechanisms that could hit end stops.

## Roadmap

1. Add host-based unit tests for asynchronous PIR behaviour using mock GPIO.
2. Validate the fan and servo implementations on the kit and improve their error/configuration handling.
3. Add ultrasonic, DHT11, RGB and LCD drivers, with focused examples.
4. Combine components into event-driven smart-home applications.
5. Expand the MicroPython equivalents and document any platform differences.

## References

- [Keyestudio KS4048/KS4049 Smart Home V2 project guide](https://docs.keyestudio.com/projects/KS4048-KS4049/en/latest/OLD/3.microbit%20smart%20home%20projects.html)
- [Embassy documentation](https://embassy.dev/)
- [embedded-hal](https://docs.rs/embedded-hal/)
- [embedded-hal-async](https://docs.rs/embedded-hal-async/)
- [probe-rs](https://probe.rs/)
