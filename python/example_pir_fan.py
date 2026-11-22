"""Copy smart_home.py and this file onto the micro:bit as main.py."""
from microbit import sleep
from smart_home import Pir, Fan

pir = Pir()          # Keyestudio PIR: P15
fan = Fan()          # Motor IN+: P13, IN-: P16

while True:
    if pir.motion_detected():
        fan.forward()
    else:
        fan.stop()
    sleep(100)
