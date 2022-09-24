#![no_std]
#![no_main]

use panic_halt as _;
use cabezita::Cabezita;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led = pins.d13.into_output();
    let mut cabezita = Cabezita::new();

    loop {
        led.toggle();
        arduino_hal::delay_ms(cabezita.increment());
    }
}
