//! Continuous character echo minicom input back with usart1 to serial-usb converter on pin (pa9, pa10).
//!
//! Connect the Tx pin pa9  to the Rx pin of a serial-usb converter
//! Connect the Rx pin pa10 to the Tx pin of a serial-usb converter
//! Set up the serial console (e.g. minicom) with the same settings used here.
//! (Using 9600bps, could be higher but needs serial console to be the same.)

#![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_halt as _;
//use cortex_m::asm;
use cortex_m_rt::entry;
//use core::fmt::Write;
use cortex_m_semihosting::hprintln;
use core::str::from_utf8;
use nb::block;

//  eg blue pill stm32f103
#[cfg(feature = "stm32f103")]
use stm32f1xx_hal::{ prelude::*,  pac,  serial::{Config, Serial}, };

//  eg Discovery-stm32f303
//use alt_stm32f30x_hal::{  ??
#[cfg(feature = "stm32f303")]
use stm32f3xx_hal::{ prelude::*, pac, serial::{Config, Serial}, };

// eg Nucleo-64  stm32f411
#[cfg(feature = "stm32f411")]
use stm32f4xx_hal::{ prelude::*, pac, serial::{Config, Serial}, };

// eg  Discovery kit stm32l100 and Heltec lora_node STM32L151CCU6
#[cfg(any(feature = "stm32l100",   feature = "stnm32l151" )) ]
use stm32l1xx_hal::{ prelude::*, pac, serial::{Config, Serial}, };


#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut afio = p.AFIO.constrain(&mut rcc.apb2);
    let mut gpioa = p.GPIOA.split(&mut rcc.apb2);
    // let mut gpiob = p.GPIOB.split(&mut rcc.apb2);

    //see examples/serial_loopback_char_test.rs for more USART config notes.

    let serial = Serial::usart1(
        p.USART1,
        (gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh),  gpioa.pa10),
        &mut afio.mapr,
        Config::default().baudrate(9600.bps()),
        clocks,
        &mut rcc.apb2,
    );

    // Split the serial struct into a receiving and a transmitting part
    let (mut tx, mut rx) = serial.split();

    loop { // Read a byte and write
      let received = block!(rx.read()).unwrap();
      block!(tx.write(received)).ok();
      hprintln!("{}", from_utf8(&[received]).unwrap()).unwrap();
    }

    // PUT A TEST HERE THAT WILL SHOW FAILURE. ASSERT SEEMS TO PANIC HALT SO ...

    // Trigger a breakpoint to inspect the values
    //asm::bkpt();

}
