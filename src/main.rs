use std::time::Duration;

use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::sys::EspError;
use eyre::bail;

fn sleep(duration: Duration) {
    FreeRtos::delay_ms(duration.as_millis() as u32);
}

trait LedControl {
    fn set_high(&mut self) -> Result<(), EspError>;
    fn set_low(&mut self) -> Result<(), EspError>;
}

impl<'a, P: OutputPin> LedControl for PinDriver<'a, P, Output> {
    fn set_high(&mut self) -> Result<(), EspError> {
        self.set_high()
    }
    fn set_low(&mut self) -> Result<(), EspError> {
        self.set_low()
    }
}

fn main() -> eyre::Result<()> {
    esp_idf_hal::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    color_eyre::install()?;

    let peripherals = Peripherals::take()?;
    let led1 = PinDriver::output(peripherals.pins.gpio2)?;
    let led2 = PinDriver::output(peripherals.pins.gpio4)?;
    let led3 = PinDriver::output(peripherals.pins.gpio33)?;
    let led4 = PinDriver::output(peripherals.pins.gpio32)?;

    let mut leds: Vec<(Box<dyn LedControl>, u8)> = vec![
        (Box::new(led1), 0b0001),
        (Box::new(led2), 0b0010),
        (Box::new(led3), 0b0100),
        (Box::new(led4), 0b1000),
    ];

    let mut i = 0;

    loop {
        i += 1;
        for (ref mut led, mask) in leds.iter_mut() {
            if i as u8 & *mask != 0 {
                led.set_high()?;
            } else {
                led.set_low()?;
            }
        }
        sleep(Duration::from_millis(500));
    }
}
