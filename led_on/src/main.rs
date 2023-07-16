use std::thread;
use std::time::Duration;
use rppal::gpio::{Gpio};

fn main() {
    // Initialize GPIO
    let gpio = Gpio::new().unwrap();

    // Specify the GPIO pin number connected to the LED
    let led_pin = 26;

    // Set the pin as an output pin
    let mut pin = gpio.get(led_pin).unwrap().into_output();
    pin.set_low();
    loop{
        // Turn on the LED by setting the pin to a high level
        pin.set_high();
        println!("Led is On");
        // Sleep for a while to keep the LED on
        thread::sleep(Duration::from_secs(1));

        // Turn off the LED by setting the pin to a low level
        pin.set_low();
        println!("Led is Off");
        // Sleep for a while to keep the LED on
        thread::sleep(Duration::from_secs(1));
    }
}
