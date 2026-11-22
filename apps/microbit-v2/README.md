# micro:bit v2 Embassy firmware

This binary package (`smart-home-firmware`) runs the Smart Home V2 application on the **BBC micro:bit v2 / Nordic nRF52833** using Embassy.

## Current application: event-driven PIR monitor

- Connect the PIR signal to **micro:bit P15**, which maps to **nRF52833 P0.13**.
- Initialise the input with `embassy_nrf::gpio::Input`.
- Create the reusable library's `Pir` driver.
- Wait asynchronously for HIGH (`Motion detected`) and LOW (`No motion`).
- Log state transitions and timestamps through `defmt` over RTT.
- On a GPIO wait error, log a warning and pause 100 ms before retrying.

The PIR module's HIGH state has commonly lasted roughly 2–3 seconds in our hardware experiments; this is an **observed output duration**, not a measurement of detection latency. Longer HIGH intervals can occur during repeated triggering.

## Build and run

Install the target and probe tools if necessary:

```bash
rustup target add thumbv7em-none-eabihf
cargo install probe-rs-tools --locked
probe-rs list
```

**From the repository root**, change into this directory before invoking Cargo:

```bash
cd apps/microbit-v2
cargo build --release
DEFMT_LOG=info cargo run --release
```

Its local `.cargo/config.toml` configures:

```toml
[build]
target = "thumbv7em-none-eabihf"

[target.thumbv7em-none-eabihf]
runner = "probe-rs run --chip nRF52833_xxAA"
```

The existing configuration also passes `-Tlink.x` and `-Tdefmt.x` to the linker. `memory.x` and `build.rs` belong with this firmware package. **Do not assume a `cargo build` from the workspace root will read this local config.**

## Viewing logs

Use:

```bash
DEFMT_LOG=info cargo run --release
```

RTT logs appear in the terminal running `probe-rs`, not on the board's 5×5 LED matrix or a serial terminal. If the binary builds but prints nothing, check that `defmt-rtt` is linked, logging is enabled and the configured runner is `probe-rs run` rather than a download-only operation.

## Electrical notes

Use the kit's intended supply and wiring. **Never apply 5 V directly to an nRF52833 GPIO input**. Confirm the module's output voltage and the expansion-board routing before connecting it.

## Next steps

Add mock-based async PIR tests in the library, then extend the firmware with separate Embassy tasks for additional hardware modules. Keep board-specific pin and peripheral setup here, not in the portable library.

See [root README](../../README.md) and [library README](../../crates/smart-home/README.md).
