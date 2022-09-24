#![no_std]
#![no_main]

use panic_halt as _;

use arduino_hal::prelude::*;

use cabezita::Cabezita;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led = pins.d13.into_output();
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut cabezita = Cabezita::new();


    loop {
        led.toggle();
        arduino_hal::delay_ms(1_000);
        led.toggle();

        let delay = cabezita.increment();
        
        ufmt::uwriteln!(&mut serial, "Hello from Arduino: {}\r", delay).void_unwrap();
        arduino_hal::delay_ms(delay);
    }
}
