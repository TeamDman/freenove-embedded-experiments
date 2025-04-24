use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;
use log::{debug, info};
use std::time::Duration;

#[allow(unused)]
fn sleep(duration: Duration) {
    FreeRtos::delay_ms(duration.as_millis() as u32);
}

#[derive(Debug)]
enum ButtonState {
    JustPressed,
    Pressed,
    Released,
}
impl ButtonState {
    fn is_pressed(&self) -> bool {
        matches!(self, ButtonState::JustPressed | ButtonState::Pressed)
    }
}

fn main() -> eyre::Result<()> {
    esp_idf_hal::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    color_eyre::install()?;

    let peripherals = Peripherals::take()?;
    let mut led_pin = PinDriver::output(peripherals.pins.gpio2)?;
    let button_pin = PinDriver::input(peripherals.pins.gpio13)?;

    let mut button_state = ButtonState::Released;
    let mut counter = 0;
    loop {
        button_state = match (button_state, button_pin.is_low()) {
            (ButtonState::Released, true) => ButtonState::JustPressed,
            (ButtonState::JustPressed, true) => ButtonState::Pressed,
            (ButtonState::Pressed, true) => ButtonState::Pressed,
            (ButtonState::Released, false) => ButtonState::Released,
            (ButtonState::JustPressed, false) => ButtonState::Released,
            (ButtonState::Pressed, false) => ButtonState::Released,
        };
        info!("Button state: {:?}, counter: {counter}", button_state);
        led_pin.toggle()?;
        if matches!(button_state, ButtonState::JustPressed) {
            counter = 1000;
        }
        if button_state.is_pressed() {
            led_pin.set_low()?;
            sleep(Duration::from_millis(counter / 10));
            counter -= 1;
            led_pin.set_high()?;
        }
        sleep(Duration::from_millis(10));
    }
}
