/// Asynchronous PIR motion sensor driver.

use embedded_hal_async::digital::Wait;

/// An asynchronous PIR motion sensor driver.
///
/// P is any GPIO implementation supporting
/// the embedded-hal-async Wait trait.
pub struct AsyncPir<P> {
    pin: P,
}

impl<P: Wait> AsyncPir<P> {
    /// Create a new asynchronous PIR driver.
    pub fn new(pin: P) -> Self {
        Self { pin }
    }

    /// Wait until motion is detected.
    pub async fn wait_for_motion(
        &mut self,
    ) -> Result<(), P::Error> {
        self.pin.wait_for_high().await
    }

    /// Wait until the sensor stops reporting motion.
    pub async fn wait_for_no_motion(
        &mut self,
    ) -> Result<(), P::Error> {
        self.pin.wait_for_low().await
    }

    /// Wait for a new rising edge.
    pub async fn wait_for_motion_edge(
        &mut self,
    ) -> Result<(), P::Error> {
        self.pin.wait_for_rising_edge().await
    }

    /// Wait for a new falling edge.
    pub async fn wait_for_no_motion_edge(
        &mut self,
    ) -> Result<(), P::Error> {
        self.pin.wait_for_falling_edge().await
    }
}