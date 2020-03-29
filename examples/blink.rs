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

// use nb::block;

#[cfg(feature = "stm32f1xx")]  //  eg blue pill stm32f103
use stm32f1xx_hal::{prelude::*,   pac::Peripherals, };

#[cfg(feature = "stm32f3xx")]  //  eg Discovery-stm32f303
use stm32f3xx_hal::{prelude::*, stm32::Peripherals, };

#[cfg(feature = "stm32f4xx")] // eg Nucleo-64  stm32f411
use stm32f4xx_hal::{prelude::*, stm32::Peripherals, };

#[cfg(feature = "stm32l1xx") ] // eg  Discovery kit stm32l100 and Heltec lora_node STM32L151CCU6
use stm32l1xx_hal::{prelude::*,   pac::Peripherals, };

use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;

// use embedded_hal::prelude::*;
use asm_delay::{ AsmDelay, bitrate, };

#[entry]
fn main() -> ! {

    // Get access to the device specific peripherals from the peripheral access crate
    let dp = Peripherals::take().unwrap();

    // Take ownership over the raw rcc device and convert to  HAL structs
    let mut rcc = dp.RCC.constrain();

    #[cfg(any(feature = "stm32f1xx", feature = "stm32l1xx"))]
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);
    #[cfg(feature = "stm32f3xx")]
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
    #[cfg(feature = "stm32f4xx")]
    let mut gpiob = dp.GPIOB.split();

    // Configure gpio B pin 14 as a push-pull output. 
    // On bluepill the `crh` register is passed to the function
    // in order to configure the port. For pins 0-7, crl should be passed instead.

    #[cfg(any(feature = "stm32f1xx", feature = "stm32l1xx"))]
    let mut led1 = gpiob.pb13.into_push_pull_output(&mut gpiob.crh);
    #[cfg(feature = "stm32f3xx")]
    let mut led1 = gpiob.pb13.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
    #[cfg(feature = "stm32f4xx")]
    let mut led1 = gpiob.pb13.into_push_pull_output();

    #[cfg(any(feature = "stm32f1xx", feature = "stm32l1xx"))]
    let mut led2 = gpiob.pb14.into_push_pull_output(&mut gpiob.crh);
    #[cfg(feature = "stm32f3xx")]
    let mut led2 = gpiob.pb14.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
    #[cfg(feature = "stm32f4xx")]
    let mut led2 = gpiob.pb14.into_push_pull_output();

    #[cfg(any(feature = "stm32f1xx", feature = "stm32l1xx"))]
    let mut led3 = gpiob.pb15.into_push_pull_output(&mut gpiob.crh);
    #[cfg(feature = "stm32f3xx")]
    let mut led3 = gpiob.pb15.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
    #[cfg(feature = "stm32f4xx")]
    let mut led3 = gpiob.pb15.into_push_pull_output();

    //this works on bluepill but need to be more specific about timer using other chips/HALs
    // may need different timer, like Struct stm32f3xx_hal::stm32::SYST
    //use stm32f1xx_hal::{ prelude::*, pac::Peripherals, timer::Timer, };
    //let cp = cortex_m::Peripherals::take().unwrap();//get core peripherals
    // Take ownership over the raw flash device and convert to  HAL structs
    // let mut flash = dp.FLASH.constrain();
    // Freeze the configuration of all clocks and store frozen frequencies in `clocks`
    // let clocks = rcc.cfgr.freeze(&mut flash.acr);
    // Configure the syst timer to trigger an update every second
    // let mut timer = Timer::syst(cp.SYST, &clocks).start_count_down(1.hz());
    // /block!(timer.wait()).unwrap(); 

    let mut d  = AsmDelay::new(bitrate::U32BitrateExt::mhz(16));
    let on  : u32 = 1000;
    let off : u32 = 3000;

    // Wait for the timer to trigger an update and change the state of the LED
    loop {
        //block!(timer.wait()).unwrap(); this works on bluepill but need to be more specific about timer on other chips
        //cortex_m::asm::delay(500_000); this is in clock cycles
        d.delay_ms(off);
        let _r = led1.set_high();
        let _r = led2.set_high();
        let _r = led3.set_high();
        //block!(timer.wait()).unwrap();
        d.delay_ms(on);
        let _r = led1.set_low();
        let _r = led2.set_low();
        let _r = led3.set_low();
    }
}
