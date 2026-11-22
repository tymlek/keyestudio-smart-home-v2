//! Smart Home V2 - PIR monitoring example for the micro:bit V2
//! Use `DEFMT_LOG=info cargo run --release` to run it

#![no_std]
#![no_main]

use defmt::{info, warn};
use embassy_nrf::gpio::{Input, Pull};
use embassy_time::{Duration, Instant, Timer};
use keyestudio_smart_home_v2::Pir;

use {defmt_rtt as _, panic_probe as _};

// P15 on the micro:bit V2 edge connector is nRF52833 P0.13.
#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let peripherals = embassy_nrf::init(Default::default());
    let sensor_pin = Input::new(peripherals.P0_13, Pull::None);
    let mut pir = Pir::new(sensor_pin);
    let mut previous: Option<bool> = None;

    info!("Smart Home V2 - PIR monitoring started");

    loop {
        match pir.motion_detected() {
            Ok(current) => {
                if previous != Some(current) {
                    let timestamp = Instant::now().as_millis();

                    if current {
                        info!("[{} ms] Motion detected", timestamp);
                    } else {
                        info!("[{} ms] No motion", timestamp);
                    }
                    previous = Some(current);
                }
            }
            Err(_) => warn!("PIR GPIO read failed"),
        }
        Timer::after(Duration::from_millis(20)).await;
    }
}
