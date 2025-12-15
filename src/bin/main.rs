#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Level, Output};
use esp_println::println;

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal::main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    println!("Starting blink demo on Lolin32 Lite");

    // GPIO22 is the onboard LED on Lolin32 Lite
    let mut led = Output::new(peripherals.GPIO22, Level::Low, Default::default());
    let delay = Delay::new();

    println!("Blinking LED on GPIO22");

    loop {
        led.toggle();
        delay.delay_millis(500);
    }
}
