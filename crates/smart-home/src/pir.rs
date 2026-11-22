// KS6018 PIR motion sensor driver.
// HIGH indicates that motion has been detected.

use embedded_hal::digital::InputPin;
use embedded_hal_async::digital::Wait;

/// PIR motion sensor driver.
pub struct Pir<P> {
    pin: P,
}

// Available for every pin type.
impl<P> Pir<P> {
    pub fn new(pin: P) -> Self {
        Self { pin }
    }

    pub fn release(self) -> P {
        self.pin
    }
}

// Available when P supports synchronous GPIO reads.
impl<P: InputPin> Pir<P> {
    pub fn motion_detected(&mut self) -> Result<bool, P::Error> {
        self.pin.is_high()
    }
}

// Available when P supports asynchronous GPIO waiting.
impl<P: Wait> Pir<P> {
    pub async fn wait_for_motion(&mut self) -> Result<(), P::Error> {
        self.pin.wait_for_high().await
    }

    pub async fn wait_for_no_motion(&mut self) -> Result<(), P::Error> {
        self.pin.wait_for_low().await
    }

    pub async fn wait_for_motion_edge(&mut self) -> Result<(), P::Error> {
        self.pin.wait_for_rising_edge().await
    }

    pub async fn wait_for_no_motion_edge(&mut self) -> Result<(), P::Error> {
        self.pin.wait_for_falling_edge().await
    }
}
