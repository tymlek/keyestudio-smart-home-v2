#![no_std]

//! Keyestudio Smart Home V2 building blocks.
//! This is a starter library: PIR, DC motor, servo pulse calculation,
//! and temperature-based fan control. It intentionally does not claim
//! support for the LCD, DHT11, ultrasonic sensor or RGB LEDs yet.

pub mod pir;

pub use pir::Pir;

use embedded_hal::digital::OutputPin;

/// KS6038 130 motor control with separate IN+ and IN- pins.
/// Supply the motor through the Keyestudio motor driver, not directly
/// from micro:bit GPIO pins.
pub struct Fan<PLUS, MINUS> { plus: PLUS, minus: MINUS }
impl<PLUS: OutputPin, MINUS: OutputPin> Fan<PLUS, MINUS> {
    pub fn new(mut plus: PLUS, mut minus: MINUS)
        -> Result<Self, FanError<PLUS::Error, MINUS::Error>> {
        plus.set_low().map_err(FanError::Plus)?;
        minus.set_low().map_err(FanError::Minus)?;
        Ok(Self { plus, minus })
    }
    pub fn stop(&mut self) -> Result<(), FanError<PLUS::Error, MINUS::Error>> {
        self.plus.set_low().map_err(FanError::Plus)?;
        self.minus.set_low().map_err(FanError::Minus)?;
        Ok(())
    }
    pub fn forward(&mut self) -> Result<(), FanError<PLUS::Error, MINUS::Error>> {
        self.stop()?;
        self.plus.set_high().map_err(FanError::Plus)
    }
    pub fn reverse(&mut self) -> Result<(), FanError<PLUS::Error, MINUS::Error>> {
        self.stop()?;
        self.minus.set_high().map_err(FanError::Minus)
    }
    pub fn release(self) -> (PLUS, MINUS) { (self.plus, self.minus) }
}
#[derive(Debug, PartialEq, Eq)]
pub enum FanError<P, M> { Plus(P), Minus(M) }

/// Compute a standard positional-servo pulse width for 0..=180 degrees.
/// Typical calibration endpoints are 1000 and 2000 us; the actual servo
/// may require different values. Avoid forcing the linkage into end stops.
pub fn servo_pulse_us(angle: u8, min_us: u16, max_us: u16) -> u16 {
    let angle = angle.min(180) as u32;
    let min = min_us as u32;
    let span = max_us.saturating_sub(min_us) as u32;
    (min + span * angle / 180) as u16
}

/// Return the desired fan state using two thresholds to prevent chatter.
/// `on_at` must be greater than `off_below`.
pub fn fan_should_run(temperature_c: i16, currently_running: bool,
                      on_at: i16, off_below: i16) -> bool {
    if currently_running { temperature_c >= off_below }
    else { temperature_c >= on_at }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn servo_endpoints() {
        assert_eq!(servo_pulse_us(0, 1000, 2000), 1000);
        assert_eq!(servo_pulse_us(90, 1000, 2000), 1500);
        assert_eq!(servo_pulse_us(180, 1000, 2000), 2000);
    }
    #[test]
    fn fan_hysteresis() {
        assert!(!fan_should_run(27, false, 28, 26));
        assert!(fan_should_run(28, false, 28, 26));
        assert!(fan_should_run(27, true, 28, 26));
        assert!(!fan_should_run(25, true, 28, 26));
    }
}
