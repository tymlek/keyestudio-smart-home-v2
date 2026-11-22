//! Servo positioning utilities.

/// Calculates the pulse width for a positional servo.
///
/// `angle` is specified in degrees (0..=180).
///
/// `min_us` and `max_us` define the calibration endpoints.
///
/// Example:
///
/// ```
/// use keyestudio_smart_home_v2::servo_pulse_us;
///
/// let pulse = servo_pulse_us(90, 1000, 2000);
///
/// assert_eq!(pulse, 1500);
/// ```
pub fn servo_pulse_us(angle: u8, min_us: u16, max_us: u16) -> u16 {
    let angle = angle.min(180) as u32;
    let min = min_us as u32;
    let span = max_us.saturating_sub(min_us) as u32;

    (min + span * angle / 180) as u16
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
}
