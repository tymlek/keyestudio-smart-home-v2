"""Keyestudio Smart Home V2 MicroPython starter library (micro:bit v2).

Included: PIR, DC motor fan, servo, temperature fan control.
Not yet included: DHT11, ultrasonic ranging, NeoPixel, I2C LCD.
"""
from microbit import pin1, pin13, pin15, pin16


class Pir:
    """Motion sensor: HIGH (1) means movement detected."""
    def __init__(self, pin=pin15):
        self.pin = pin

    def motion_detected(self):
        return self.pin.read_digital() == 1


class Fan:
    """Two-input motor module; never connect a DC motor to GPIO directly."""
    def __init__(self, plus=pin13, minus=pin16):
        self.plus = plus
        self.minus = minus
        self.stop()

    def stop(self):
        self.plus.write_digital(0)
        self.minus.write_digital(0)

    def forward(self):
        self.stop()
        self.plus.write_digital(1)

    def reverse(self):
        self.stop()
        self.minus.write_digital(1)


class Servo:
    """Positional servo, 20 ms frame, calibrated pulse widths in microseconds."""
    def __init__(self, pin=pin1, min_us=1000, max_us=2000):
        if not (0 < min_us < max_us < 20000):
            raise ValueError("invalid servo pulse range")
        self.pin = pin
        self.min_us = min_us
        self.max_us = max_us
        self.pin.set_analog_period(20)

    def set_angle(self, angle):
        angle = max(0, min(180, angle))
        pulse_us = self.min_us + (self.max_us - self.min_us) * angle // 180
        duty = (pulse_us * 1023 + 10000) // 20000
        self.pin.write_analog(duty)


def fan_should_run(temperature_c, currently_running, on_at=28, off_below=26):
    """Hysteresis: switch on at 28 C, off below 26 C by default."""
    if on_at <= off_below:
        raise ValueError("on_at must be greater than off_below")
    if currently_running:
        return temperature_c >= off_below
    return temperature_c >= on_at
