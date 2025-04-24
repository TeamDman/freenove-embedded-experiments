use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;
use std::time::Duration;

#[allow(unused)]
fn sleep(duration: Duration) {
    FreeRtos::delay_ms(duration.as_millis() as u32);
}

fn main() -> eyre::Result<()> {
    esp_idf_hal::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    color_eyre::install()?;

    let peripherals = Peripherals::take()?;
    let mut led_pin = PinDriver::output(peripherals.pins.gpio2)?;
    let button_pin = PinDriver::input(peripherals.pins.gpio13)?;

    loop {
        if button_pin.is_high() {
            led_pin.set_low()?;
        } else {
            led_pin.set_high()?;
        }
        sleep(Duration::from_millis(10));
    }
}
