//! Blinks off-board LEDs attached to  pb 13,14,15. 
//! compare stm32f1xx_hal example blinky.rs.

#![deny(unsafe_code)]
#![no_std]
#![no_main]

#[cfg(debug_assertions)]
extern crate panic_semihosting;

#[cfg(not(debug_assertions))]
extern crate panic_halt;

// extern crate panic_halt;  // put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // may still require nightly?
// extern crate panic_itm;   // logs messages over ITM; requires ITM support
// extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

// use nb::block;
use cortex_m_rt::entry;
use asm_delay::{ AsmDelay, bitrate, };


#[cfg(feature = "stm32f1xx")]  //  eg blue pill stm32f103
use stm32f1xx_hal::{prelude::*,   
                     pac::Peripherals,
		     gpio::{gpiob::{PB13, PB14, PB15}, Output, PushPull,}, 
		     };

#[cfg(feature = "stm32f1xx")]  //  eg blue pill stm32f103
use embedded_hal::digital::v2::OutputPin;



#[cfg(feature = "stm32f3xx")]  //  eg Discovery-stm32f303
use  stm32f3xx_hal::{prelude::*,
                     stm32::Peripherals, 
		     gpio::{gpiob::{PB13, PB14, PB15}, Output, PushPull,}, 
		     };


#[cfg(feature = "stm32f4xx")] // eg Nucleo-64  stm32f411
use  stm32f4xx_hal::{prelude::*,   
                     pac::Peripherals, 
		     gpio::{gpiob::{PB13, PB14, PB15}, Output, PushPull,}, 
		     };

#[cfg(feature = "stm32f4xx")]  //  eg Nucleo-64  stm32f411
use embedded_hal::digital::v2::OutputPin;


#[cfg(feature = "stm32l1xx") ] // eg  Discovery kit stm32l100 and Heltec lora_node STM32L151CCU6
use stm32l1xx_hal::{prelude::*, 
                     stm32::Peripherals,
		     gpio::{gpiob::{PB13, PB14, PB15}, Output, PushPull,}, 
                     };

#[cfg(feature = "stm32l1xx") ] // eg  Discovery kit stm32l100 and Heltec lora_node STM32L151CCU6
use embedded_hal::digital::v2::OutputPin;


#[entry]
fn main() -> ! {

    // 1. Get device specific peripherals
    // 2. Take ownership of the raw rcc (Reset and Clock Control) device and convert to  HAL structs
    // 3. Configure gpio B pins 13,14,15 as a push-pull output (bluepill). 
    //    On bluepill the `crh` register is passed to the function
    //    in order to configure the port. For pins 0-7, crl should be passed instead.

    #[cfg(feature = "stm32f1xx")]
    fn setup() -> (PB13<Output<PushPull>>, PB14<Output<PushPull>>, PB15<Output<PushPull>>) {
       
       let dp        = Peripherals::take().unwrap();
       let mut rcc   = dp.RCC.constrain(); 
       let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

       // return (led1, led2, led3)
       (gpiob.pb13.into_push_pull_output(&mut gpiob.crh),  // led on pb13
        gpiob.pb14.into_push_pull_output(&mut gpiob.crh),  // led on pb14
        gpiob.pb15.into_push_pull_output(&mut gpiob.crh))  // led on pb15
       };

    #[cfg(feature = "stm32f3xx")]
    fn setup() -> (PB13<Output<PushPull>>, PB14<Output<PushPull>>, PB15<Output<PushPull>>) {

       let dp        = Peripherals::take().unwrap();
       let mut rcc   = dp.RCC.constrain();
       let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);

       // return (led1, led2, led3)
       (gpiob.pb13.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper), //led on pb12
        gpiob.pb14.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper), //led on pb14
        gpiob.pb15.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper)) //led on pb15
       };

    #[cfg(feature = "stm32f4xx")]
    fn setup() -> (PB13<Output<PushPull>>, PB14<Output<PushPull>>, PB15<Output<PushPull>>) {

       let dp    = Peripherals::take().unwrap();
       let gpiob = dp.GPIOB.split();

       // return (led1, led2, led3)
       (gpiob.pb13.into_push_pull_output(),  // led on pb12
        gpiob.pb14.into_push_pull_output(),  // led on pb14
        gpiob.pb15.into_push_pull_output())  // led on pb15
       };

    #[cfg(feature = "stm32l1xx")]
    fn setup() -> (PB13<Output<PushPull>>, PB14<Output<PushPull>>, PB15<Output<PushPull>>) {

       let dp    = Peripherals::take().unwrap();
       let gpiob = dp.GPIOB.split();

       // return (led1, led2, led3)
       (gpiob.pb13.into_push_pull_output(),  // led on pb12
        gpiob.pb14.into_push_pull_output(),  // led on pb14
        gpiob.pb15.into_push_pull_output())  // led on pb15
       };


    // End of hal/MCU specific setup. Following should be generic code.


    let (mut led1, mut led2, mut led3) = setup();


    //works for delay on bluepill but not others
    //use stm32f1xx_hal::timer::Timer;
    // trigger an update every second
    // let mut timer = Timer::syst(cp.SYST, &clocks).start_count_down(1.hz());
    // /block!(timer.wait()).unwrap(); 

    //cortex_m::asm::delay(500_000); this is in clock cycles
    let mut d  = AsmDelay::new(bitrate::U32BitrateExt::mhz(16));
    let on  : u32 = 1000;
    let off : u32 = 3000;

    // Wait for the timer to trigger an update and change the state of the LEDs
    loop {
        d.delay_ms(off);
        let _r = led1.set_high();
        let _r = led2.set_high();
        let _r = led3.set_high();

        d.delay_ms(on);
        let _r = led1.set_low();
        let _r = led2.set_low();
        let _r = led3.set_low();
    }
}
