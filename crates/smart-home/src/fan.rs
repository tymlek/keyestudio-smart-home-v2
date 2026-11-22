//! DC motor fan driver and temperature control logic.

use embedded_hal::digital::OutputPin;

/// KS6038 DC motor controller.
///
/// Controls the motor using separate IN+ and IN- pins.
///
/// The motor must be powered through the appropriate
/// motor driver, not directly from micro:bit GPIO pins.
pub struct Fan<PLUS, MINUS> {
    plus: PLUS,
    minus: MINUS,
}

/// Errors returned by the fan driver.
#[derive(Debug, PartialEq, Eq)]
pub enum FanError<P, M> {
    Plus(P),
    Minus(M),
}

impl<PLUS: OutputPin, MINUS: OutputPin> Fan<PLUS, MINUS> {
    /// Creates a fan driver with both outputs LOW.
    pub fn new(
        mut plus: PLUS,
        mut minus: MINUS,
    ) -> Result<Self, FanError<PLUS::Error, MINUS::Error>> {
        plus.set_low().map_err(FanError::Plus)?;
        minus.set_low().map_err(FanError::Minus)?;

        Ok(Self { plus, minus })
    }

    /// Stops the motor.
    pub fn stop(&mut self) -> Result<(), FanError<PLUS::Error, MINUS::Error>> {
        self.plus.set_low().map_err(FanError::Plus)?;
        self.minus.set_low().map_err(FanError::Minus)?;

        Ok(())
    }

    /// Runs the motor forwards.
    pub fn forward(&mut self) -> Result<(), FanError<PLUS::Error, MINUS::Error>> {
        self.stop()?;

        self.plus.set_high().map_err(FanError::Plus)
    }

    /// Runs the motor backwards.
    pub fn reverse(&mut self) -> Result<(), FanError<PLUS::Error, MINUS::Error>> {
        self.stop()?;

        self.minus.set_high().map_err(FanError::Minus)
    }

    /// Releases ownership of both GPIO pins.
    pub fn release(self) -> (PLUS, MINUS) {
        (self.plus, self.minus)
    }
}

/// Determines whether the fan should run.
///
/// Uses separate switching thresholds to prevent
/// rapid switching around a single temperature.
///
/// `on_at` must be greater than `off_below`.
pub fn fan_should_run(
    temperature_c: i16,
    currently_running: bool,
    on_at: i16,
    off_below: i16,
) -> bool {
    if currently_running {
        temperature_c >= off_below
    } else {
        temperature_c >= on_at
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fan_hysteresis() {
        assert!(!fan_should_run(27, false, 28, 26));
        assert!(fan_should_run(28, false, 28, 26));
        assert!(fan_should_run(27, true, 28, 26));
        assert!(!fan_should_run(25, true, 28, 26));
    }
}
