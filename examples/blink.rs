//! Blinks off-board LEDs attached to  pb 13,14,15. 
//! Following stm32f1xx_hal example blinky.rs.

#![deny(unsafe_code)]
#![no_std]
#![no_main]

#[cfg(debug_assertions)]
extern crate panic_semihosting;

#[cfg(not(debug_assertions))]
extern crate panic_halt;

// extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
// extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

use nb::block;

//  eg blue pill stm32f103
#[cfg(any(feature = "stm32f100",  feature = "stm32f101", feature = "stm32f103" ))]
use stm32f1xx_hal::{ prelude::*, pac::Peripherals, timer::Timer, };

//  eg Discovery-stm32f303
//use alt_stm32f30x_hal::{  ??
#[cfg(any(feature = "stm32f301",  feature = "stm32f302", feature = "stm32f303"))]
use stm32f3xx_hal::{ prelude::*, pac::Peripherals, timer::Timer, };

// eg Nucleo-64  stm32f411
#[cfg(feature = "stm32f411")]
use stm32f4xx_hal::{ prelude::*, stm32::Peripherals, timer::Timer, };

// eg  Discovery kit stm32l100 and Heltec lora_node STM32L151CCU6
#[cfg(any(feature = "stm32l100",   feature = "stnm32l151" )) ]
use stm32l1xx_hal::{ prelude::*, pac::Peripherals, timer::Timer, };

use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;

#[entry]
fn main() -> ! {
    // Get access to the core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Acquire the GPIOB peripheral
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    // Configure gpio B pin 14 as a push-pull output. The `crh` register is passed to the function
    // in order to configure the port. For pins 0-7, crl should be passed instead.
    let mut led1 = gpiob.pb13.into_push_pull_output(&mut gpiob.crh);
    let mut led2 = gpiob.pb14.into_push_pull_output(&mut gpiob.crh);
    let mut led3 = gpiob.pb15.into_push_pull_output(&mut gpiob.crh);

    // Configure the syst timer to trigger an update every second
    let mut timer = Timer::syst(cp.SYST, &clocks).start_count_down(1.hz());
    //let mut timer = Timer::syst(cp.SYST, 1.hz(), clocks);

    // Wait for the timer to trigger an update and change the state of the LED
    loop {
        block!(timer.wait()).unwrap();
        let _r = led1.set_high();
        let _r = led2.set_high();
        let _r = led3.set_high();
        block!(timer.wait()).unwrap();
        let _r = led1.set_low();
        let _r = led2.set_low();
        let _r = led3.set_low();
    }
}
