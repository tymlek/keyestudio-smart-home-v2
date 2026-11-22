# Keyestudio Smart Home V2: Rust and MicroPython starter libraries

Based on the Keyestudio KS4048/KS4049 Smart Home Projects documentation:
https://docs.keyestudio.com/projects/KS4048-KS4049/en/latest/OLD/3.microbit%20smart%20home%20projects.html

## What's implemented

| Capability | Rust | MicroPython |
|---|---|---|
| PIR motion sensor | `Pir` | `Pir` |
| Two-input fan motor driver | `Fan` | `Fan` |
| Servo positioning | Pulse-width calculation | `Servo` |
| Temperature fan hysteresis | `fan_should_run` | `fan_should_run` |

This is a **starter**, not a complete hardware library. The Rust servo helper
calculates pulse width only; a board-specific PWM adapter must generate the
signal. Hardware drivers have not been tested on a physical board.

## Pins reported in the Keyestudio guide

| Module | Pins |
|---|---|
| RGB 4-pixel module | P14 |
| Ultrasonic | TRIG P12, ECHO P8 |
| Motor | IN+ P13, IN- P16 |
| PIR | P15 |
| DHT11 | P2 in standalone example, P0 in the environment-monitor example |
| Door servo | P1 |
| Bin servo | P9 |
| LCD | SDA P20, SCL P19 |

Confirm actual wiring and electrical supply with the expansion-board markings.
In particular, never drive a DC motor from a micro:bit GPIO pin directly.
Ensure the ultrasonic sensor's ECHO voltage is compatible with micro:bit 3.3 V GPIO.
The Keyestudio assembly notes require servo calibration before attaching the linkage.

## Rust

`rust/` is a portable `no_std` library crate using `embedded-hal` 1.0.
Add it as a path dependency to the existing firmware workspace, then provide
micro:bit HAL pins configured in input/output modes. No hardware firmware
entrypoint is included because it must match your existing micro:bit HAL version.
Run `cargo test --manifest-path rust/Cargo.toml` on a host to test pure logic.

## MicroPython

Transfer `python/smart_home.py` to the micro:bit, and rename
`python/example_pir_fan.py` to `main.py`. The example turns on the fan
when motion is detected. Use the kit's motor driver and intended battery
supply. Use a powered micro:bit v2 (not desktop Python).

## Next drivers

DHT11 single-wire timing, 4-pixel WS2812/6812 RGB, ultrasonic trigger/echo,
I2C 1602 LCD and application demos (password door, monitoring, bin, alarms).
These require validating the module's exact signalling and board-level peripherals.
