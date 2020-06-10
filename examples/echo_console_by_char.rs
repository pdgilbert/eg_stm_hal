//! Echo console input back to console + semihost output, char by char
//!
//! Connect the Tx pin pa9  to the Rx pin of usb-ttl converter
//! Connect the Rx pin pa10 to the Tx pin of usb-ttl converter
//! Set up the serial console (e.g. minicom) with the same settings used here.
//! (Using 9600bps, could be higher but needs serial console to be the same.)

#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[cfg(debug_assertions)]
extern crate panic_semihosting;

#[cfg(not(debug_assertions))]
extern crate panic_halt;

//use cortex_m::asm;
use cortex_m_rt::entry;
use core::fmt::Write;  // for writeln
use cortex_m_semihosting::hprintln;
use core::str::from_utf8;
use nb::block;

#[cfg(feature = "stm32f1xx")]  //  eg blue pill stm32f103
use stm32f1xx_hal::{prelude::*,   pac::Peripherals, serial::{Config, Serial, StopBits}, };

#[cfg(feature = "stm32f3xx")]  //  eg Discovery-stm32f303
use stm32f3xx_hal::{prelude::*, stm32::Peripherals, serial::{ Serial}, };

#[cfg(feature = "stm32f4xx")] // eg Nucleo-64  stm32f411
use stm32f4xx_hal::{prelude::*, stm32::Peripherals, serial::{config::Config, Serial }};

#[cfg(feature = "stm32l1xx") ] // eg  Discovery kit stm32l100 and Heltec lora_node STM32L151CCU6
use stm32l1xx_hal::{prelude::*, stm32::Peripherals, serial::{config::Config, Serial }};


#[entry]
fn main() -> ! {
 
    //see serial_loopback_char.rs and serial_cross.rs in examples/ for more USART config notes.

    // 1. Get access to the device specific peripherals from the peripheral access crate
    // 2. Take ownership of raw rcc and flash devices and convert to HAL structs
    // 3. Freeze  all system clocks  and store the frozen frequencies in `clocks`
    // 4. Prepare the alternate function I/O registers
    // 5. Prepare the GPIO peripheral
    // 6. Set up the usart device. Take ownership over the USART register and tx/rx pins.
    //    The rest of the registers are used to enable and configure the device.

    let p = Peripherals::take().unwrap();

    #[cfg(feature = "stm32f1xx")]
    let mut rcc = p.RCC.constrain();
    #[cfg(feature = "stm32f1xx")]
    let clocks = rcc.cfgr.freeze(&mut p.FLASH.constrain().acr);
    #[cfg(feature = "stm32f1xx")]
    let mut gpioa = p.GPIOA.split(&mut rcc.apb2);
    #[cfg(feature = "stm32f1xx")]
    let txrx1 = Serial::usart1(
        p.USART1,
        (gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh),  gpioa.pa10),
        &mut p.AFIO.constrain(&mut rcc.apb2).mapr,
        Config::default() .baudrate(9600.bps()) .stopbits(StopBits::STOP1),
        clocks,
        &mut rcc.apb2,
    );


    #[cfg(feature = "stm32f3xx")]
    let mut rcc = p.RCC.constrain();
    #[cfg(feature = "stm32f3xx")]
    let clocks = rcc.cfgr.freeze(&mut p.FLASH.constrain().acr);
    #[cfg(feature = "stm32f3xx")]
    let mut gpioa = p.GPIOA.split(&mut rcc.ahb);  //ahb ?
    #[cfg(feature = "stm32f3xx")]
    let txrx1 = Serial::usart1(
        p.USART1,
        (gpioa.pa9.into_af7(&mut gpioa.moder, &mut gpioa.afrh),  gpioa.pa10.into_af7(&mut gpioa.moder, &mut gpioa.afrh)),
        9600.bps(),
        clocks,
        &mut rcc.apb2,
    );


    #[cfg(any(feature = "stm32f4xx", feature = "stm32l1xx"))]
    let clocks = p.RCC.constrain().cfgr.freeze();
    #[cfg(any(feature = "stm32f4xx", feature = "stm32l1xx"))]
    let gpioa = p.GPIOA.split();
    #[cfg(any(feature = "stm32f4xx", feature = "stm32l1xx"))]
    p.USART1.cr1.modify(|_,w| w.rxneie().set_bit());  //need RX interrupt? 
    //let (tx,rx) = 
    // See examples/serail_cross.rs for stm32f411re uart and alternate function notes.
    #[cfg(any(feature = "stm32f4xx", feature = "stm32l1xx"))]
    let txrx1 =  Serial::usart1(
        p.USART1,
    	(gpioa.pa9.into_alternate_af7(),  gpioa.pa10.into_alternate_af7()), 
    	Config::default() .baudrate(9600.bps()),
    	clocks
    ).unwrap(); 
    

    // Split the serial txrx1 struct into a receiving and a transmitting part
    let (mut tx, mut rx) =txrx1.split();


    hprintln!("test formatted write to consile ...").unwrap();
    let number = 42;
    // write! and writeln! cause method not found in `stm32f3xx_hal but works in other HALs
    writeln!(tx, "\r\nHello {}. Converted number set to 42.\r\n", number).unwrap();
 
    hprintln!("test read and write by char. Please type into the console ...").unwrap();

    loop { // Read a byte and write
       let received = block!(rx.read()).unwrap();
       block!(tx.write(received)).ok();
       hprintln!("{}", from_utf8(&[received]).unwrap()).unwrap();
     }

     // PUT A TEST HERE THAT WILL SHOW FAILURE. ASSERT SEEMS TO PANIC HALT SO ...

     // Trigger a breakpoint to inspect the values
     //asm::bkpt();

}
