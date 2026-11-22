# Smart Home MicroPython starter

This directory contains a **MicroPython implementation for BBC micro:bit v2**. It is intentionally independent of the Rust Cargo workspace.

## Files

| File | Purpose |
| --- | --- |
| `smart_home.py` | `Pir`, `Fan`, `Servo`, and `fan_should_run` helpers |
| `example_pir_fan.py` | Demonstration: run the fan when PIR motion is detected |

## Running on a micro:bit v2

1. Use a micro:bit MicroPython-compatible editor or deployment tool.
2. Copy `smart_home.py` onto the board.
3. Copy `example_pir_fan.py` as `main.py`.
4. Connect the relevant modules to the specified pins and power them appropriately.
5. Run/reset the board to execute `main.py`.

The module imports from `microbit`, so it is **not intended to execute using desktop CPython**. The Python hardware drivers have **not yet been validated on the physical kit**.

## Current API

```python
from smart_home import Pir, Fan, Servo, fan_should_run

pir = Pir()       # Default: P15
fan = Fan()       # Default: P13 / P16
servo = Servo()   # Default: P1

motion = pir.motion_detected()
should_run = fan_should_run(temperature_c=29, currently_running=False)
```

The `Servo` class uses micro:bit analogue PWM output and a 20 ms period. Check the actual servo's pulse range before connecting a mechanical linkage.

## Limitations

DHT11, ultrasonic, addressable RGB LEDs and I²C LCD support are planned. MicroPython's blocking style differs from the Rust firmware's Embassy-based async execution; both implementations aim to provide a clear interface rather than identical internal scheduling.

See the [root README](../README.md) for the kit pin map and electrical cautions.
