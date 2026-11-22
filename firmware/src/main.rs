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

    info!("Smart Home V2 - PIR monitoring started");

    loop {
        // Suspend until the sensor reports motion.
        match pir.wait_for_motion().await {
            Ok(()) => {
                info!("[{} ms] Motion detected", Instant::now().as_millis());
            }
            Err(_) => {
                warn!("Error waiting for motion");
                continue;
            }
        }

        // Suspend until the sensor stops reporting motion.
        match pir.wait_for_no_motion().await {
            Ok(()) => {
                info!("[{} ms] No motion", Instant::now().as_millis());
            }
            Err(_) => {
                warn!("Error waiting for no motion");
            }
        }
    }
}
