#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use esp_hal::clock::CpuClock;
use esp_hal::gpio::OutputConfig;
use esp_hal::gpio::{Level, Output};
use esp_hal::main;
use esp_hal::time::Duration;
use esp_hal::timer::timg::TimerGroup;
use esp_println::println;
use esp_radio::ble::controller::BleConnector;
use esp_rtos::CurrentThreadHandle;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    // generator version: 1.1.0

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    println!("Starting up...");

    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 98768);
    // COEX needs more RAM - so we've added some more
    esp_alloc::heap_allocator!(size: 64 * 1024);

    println!("Heap allocated");

    // Test LED BEFORE WiFi/BLE init to verify hardware
    let mut led = Output::new(peripherals.GPIO22, Level::Low, OutputConfig::default());
    println!("LED initialized");

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    println!("Timer group created");

    esp_rtos::start(timg0.timer0);
    println!("RTOS started");

    let radio_init = esp_radio::init().expect("Failed to initialize Wi-Fi/BLE controller");
    println!("Radio initialized");

    let (mut _wifi_controller, _interfaces) =
        esp_radio::wifi::new(&radio_init, peripherals.WIFI, Default::default())
            .expect("Failed to initialize Wi-Fi controller");
    println!("WiFi initialized");

    let _connector = BleConnector::new(&radio_init, peripherals.BT, Default::default());
    println!("BLE initialized");

    println!("Entering main loop");
    loop {
        println!("LED ON");
        led.set_high();
        CurrentThreadHandle::get().delay(Duration::from_millis(500));

        println!("LED OFF");
        led.set_low();
        CurrentThreadHandle::get().delay(Duration::from_millis(500));
    }
}
