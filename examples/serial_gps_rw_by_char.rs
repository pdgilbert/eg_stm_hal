//! Serial interface char-by-char read GPS on usart3 and write on usart1 
//! to USB-TTL connected to console (minicom) and write to semihost 
//!
//! THESE ARE BLUE PILL PIN NUMBERS. CONFIRM PIN NUMBERS OF OTHER BOARDS
//! usart1 connect the Tx pin pa9  to the Rx pin of a serial-usb converter
//! usart1 connect the Rx pin pa10 to the Tx pin of a serial-usb converter
//! Set up the serial console (e.g. minicom) with the same settings used here.
//! (Using 9600bps, could be higher but needs serial console to be the same.)
//!
//! GPS uses 9600bps, 8bit, odd parity, 1 stopbit. This can be confirmed by connecting GPS 
//!  directly to the  USB-TTL and terminal with these settings (minicom 8-N-1) 
//! usart3 connect the Rx pin pb11 to the Tx pin of GPS 
//! usart3 connect the Tx pin pb10 to the Rx pin of GPS
//! 
//! See examples/serial_loopback_char.rs for notes about connecting usart1 to 
//!   serial-usb converter on computer for console output.
//! That file also has for more notes regarding setup below.

#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[cfg(debug_assertions)]
extern crate panic_semihosting;

#[cfg(not(debug_assertions))]
extern crate panic_halt;

//use cortex_m::asm;
use cortex_m_rt::entry;
//use core::fmt::Write;
use cortex_m_semihosting::hprintln;
use core::str::from_utf8;
use nb::block;

//  eg blue pill stm32f103
#[cfg(any(feature = "stm32f100",  feature = "stm32f101", feature = "stm32f103" ))]
use stm32f1xx_hal::{prelude::*, pac::Peripherals, serial::{Config, Serial, StopBits}};

//  eg Discovery-stm32f303
//use alt_stm32f30x_hal::{  ??
#[cfg(any(feature = "stm32f301",  feature = "stm32f302", feature = "stm32f303"))]
use stm32f3xx_hal::{prelude::*, stm32::Peripherals, serial::{config::Config, Serial, config::StopBits}};

// eg Nucleo-64  stm32f411
#[cfg(feature = "stm32f411")]
use stm32f4xx_hal::{prelude::*, stm32::Peripherals, serial::{config::Config, Serial, config::StopBits}};

// eg  Discovery kit stm32l100 and Heltec lora_node STM32L151CCU6
#[cfg(any(feature = "stm32l100",   feature = "stnm32l151" )) ]
use stm32l1xx_hal::{prelude::*, pac::Peripherals, serial::{Config, Serial, StopBits}};


#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut afio = p.AFIO.constrain(&mut rcc.apb2);
    let mut gpioa = p.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = p.GPIOB.split(&mut rcc.apb2);

    //see examples/serial_loopback_char.rs for more USART config notes.

    let serial1 = Serial::usart1(
        p.USART1,
        (gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh),  gpioa.pa10),
        &mut afio.mapr,
        Config::default().baudrate(9600.bps()),
        clocks,
        &mut rcc.apb2,
    );

    //  usart3 to GPS, connect the Tx pin pb10 to the Rx pin of GPS
    let serial3 = Serial::usart3(
        p.USART3,
        (gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh),  gpiob.pb11),
        &mut afio.mapr,
        Config::default() .baudrate(9600.bps())  .parity_odd() .stopbits(StopBits::STOP1),
        clocks,
        &mut rcc.apb1,
    );

    // Split the serial struct into a receiving and a transmitting part
    let (mut tx1, mut _rx1) = serial1.split();  // console
    let (mut _tx3, mut rx3) = serial3.split();  // GPS

    loop { // Read a byte and write
      let received = block!(rx3.read()).unwrap();
      block!(tx1.write(received)).ok();
      hprintln!("{}", from_utf8(&[received]).unwrap()).unwrap();
    }

    // PUT A TEST HERE THAT WILL SHOW FAILURE. ASSERT SEEMS TO PANIC HALT SO ...

    // Trigger a breakpoint to inspect the values
    //asm::bkpt();

}
