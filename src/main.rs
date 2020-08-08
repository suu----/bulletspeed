#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;
use lpc11xx;

use panic_halt as _;

use cortex_m_semihosting::{debug, hprintln};

#[entry]
fn main() -> ! {
    asm::nop(); // To not have main optimize to abort in release mode, remove when you add code
    let perip = lpc11xx::Peripherals::take().unwrap();
    hprintln!("Hello, world!").unwrap();


        // gpio初期化(PC8を出力に指定)
        let gpio = &perip.GPIO1;
        gpio.dir.write(|w| unsafe{w.bits(1) } );
        loop {
            gpio.data.write(|w| unsafe{w.bits(1)} );
        }

}
