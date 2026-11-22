#![no_std]

//! Keyestudio Smart Home V2 reusable hardware drivers.
//!
//! Hardware-independent drivers and control logic
//! for the Keyestudio Smart Home V2 kit.

pub mod fan;
pub mod pir;
pub mod servo;

// Re-export commonly used types and functions.
pub use fan::{fan_should_run, Fan, FanError};
pub use pir::Pir;
pub use servo::servo_pulse_us;
