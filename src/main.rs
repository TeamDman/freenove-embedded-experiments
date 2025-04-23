use std::time::Duration;

use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;
use eyre::bail;

fn sleep(duration: Duration) {
    FreeRtos::delay_ms(duration.as_millis() as u32);
}

fn main() -> eyre::Result<()> {
    esp_idf_hal::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    color_eyre::install()?;

    let peripherals = Peripherals::take()?;
    let mut led = PinDriver::output(peripherals.pins.gpio2)?;
    let mut i = 0;

    let high_duration = Duration::from_millis(1000);
    let low_duration = Duration::from_millis(1000);
    loop {
        led.set_high()?;
        // we are sleeping here to make sure the watchdog isn't triggered
        sleep(high_duration);

        led.set_low()?;
        sleep(low_duration);

        log::info!("BRUH! {i}");
        i+=1;
        if i > 10 {
            bail!("Reached 10!");
        }
    }
}