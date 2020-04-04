//! Serial interface char-by-char read GPS on usart3 and write on usart1 
//! to USB-TTL connected to console (minicom) and write to semihost 
//!
//! THESE ARE BLUE PILL PIN NUMBERS. CONFIRM PIN NUMBERS OF OTHER BOARDS
//! usart1 connect the Tx pin pa9  to the Rx pin of a serial-usb converter
//! usart1 connect the Rx pin pa10 to the Tx pin of a serial-usb converter
//! Set up the serial console (e.g. minicom) with the same settings used here.
//! (Using 9600bps, could be higher but needs serial console to be the same.)
//!
//!  usart3 to GPS, connect the Tx pin pb10 to the Rx pin of GPS
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

#[cfg(feature = "stm32f1xx")]  //  eg blue pill stm32f103
use stm32f1xx_hal::{prelude::*,   pac::Peripherals, serial::{Config, Serial, StopBits}, };

#[cfg(feature = "stm32f3xx")]  //  eg Discovery-stm32f303
use stm32f3xx_hal::{prelude::*, stm32::Peripherals, serial::{Serial}, };
//use stm32f3xx_hal::{prelude::*, stm32::Peripherals, serial::{Config, Serial, StopBits}, };

#[cfg(feature = "stm32f4xx")] // eg Nucleo-64  stm32f411
use stm32f4xx_hal::{prelude::*, stm32::Peripherals, serial::{config::Config, Serial }};

#[cfg(feature = "stm32l1xx") ] // eg  Discovery kit stm32l100 and Heltec lora_node STM32L151CCU6
use stm32l1xx_hal::{prelude::*,   pac::Peripherals, serial::{Config, Serial, StopBits}, };


#[entry]
fn main() -> ! {
 
    //see serial_loopback_char.rs and serial_cross.rs in examples/ for more USART config notes.

    let p = Peripherals::take().unwrap();

    #[cfg(any(feature = "stm32f1xx", feature = "stm32l1xx"))]
    let mut rcc = p.RCC.constrain();
    #[cfg(any(feature = "stm32f1xx", feature = "stm32l1xx"))]
    let clocks = rcc.cfgr.freeze(&mut p.FLASH.constrain().acr);
    #[cfg(any(feature = "stm32f1xx", feature = "stm32l1xx"))]
    let mut afio = p.AFIO.constrain(&mut rcc.apb2);
    #[cfg(any(feature = "stm32f1xx", feature = "stm32l1xx"))]
    let mut gpioa = p.GPIOA.split(&mut rcc.apb2);
    #[cfg(any(feature = "stm32f1xx", feature = "stm32l1xx"))]
    let mut gpiob = p.GPIOB.split(&mut rcc.apb2);
    #[cfg(any(feature = "stm32f1xx", feature = "stm32l1xx"))]
    let txrx1 = Serial::usart1(
        p.USART1,
        (gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh),  gpioa.pa10),
        &mut afio.mapr,
        Config::default() .baudrate(9600.bps()) .stopbits(StopBits::STOP1),
        clocks,
        &mut rcc.apb2,
    );
    #[cfg(any(feature = "stm32f1xx", feature = "stm32l1xx"))]
    let txrx3 = Serial::usart3(
        p.USART3,
        ( gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh),   gpiob.pb11),  // (tx, rx)
        &mut afio.mapr,
        Config::default() .baudrate(115_200.bps())  .parity_odd() .stopbits(StopBits::STOP1),
        clocks,
        &mut rcc.apb1,    // WHAT IS  rcc.apb1/2 ?
    );


    #[cfg(feature = "stm32f3xx")]
    let mut rcc   = p.RCC.constrain();
    #[cfg(feature = "stm32f3xx")]
    let clocks    = rcc.cfgr.freeze(&mut p.FLASH.constrain().acr);
    #[cfg(feature = "stm32f3xx")]
    let mut gpioa = p.GPIOA.split(&mut rcc.ahb);
    #[cfg(feature = "stm32f3xx")]
    let mut gpiob = p.GPIOB.split(&mut rcc.ahb);
    #[cfg(feature = "stm32f3xx")]
    let txrx1     = Serial::usart1(
        p.USART1,
        (gpioa.pa9.into_af7(&mut gpioa.moder, &mut gpioa.afrh), gpioa.pa10.into_af7(&mut gpioa.moder, &mut gpioa.afrh)),
        9600.bps(),
        clocks,
        &mut rcc.apb2,
    );
    #[cfg(feature = "stm32f3xx")]
    let txrx3 = Serial::usart3(
        p.USART3,
        (gpiob.pb10.into_af7(&mut gpiob.moder, &mut gpiob.afrh), gpiob.pb11.into_af7(&mut gpiob.moder, &mut gpiob.afrh)), 
        115_200.bps(),
        clocks,
        &mut rcc.apb1,    // WHAT IS  rcc.apb1/2 ?
    );


    #[cfg(feature = "stm32f4xx")]
    let clocks    =  p.RCC.constrain().cfgr.freeze();
    #[cfg(feature = "stm32f4xx")]
    let gpioa = p.GPIOA.split();
    #[cfg(feature = "stm32f4xx")]
    let txrx1 = Serial::usart1(
        p.USART1,
        (gpioa.pa9.into_alternate_af7(),  gpioa.pa10.into_alternate_af7()),
     	Config::default() .baudrate(9600.bps()),
        clocks,
    ).unwrap();
    #[cfg(feature = "stm32f4xx")]
    let txrx3 = Serial::usart6(
        p.USART6,
        ( gpioa.pa11.into_alternate_af8(),   gpioa.pa12.into_alternate_af8()),  // (tx, rx)  NOTE PINS, USART !!!
        Config::default() .baudrate(115_200.bps()) ,
        clocks,
    ).unwrap();


    // Split the serial struct into a receiving and a transmitting part
    let (mut tx1, mut _rx1) = txrx1.split();  // console
    let (mut _tx3, mut rx3) = txrx3.split();  // GPS

    loop { // Read a byte and write
      let received = block!(rx3.read()).unwrap();
      block!(tx1.write(received)).ok();
      hprintln!("{}", from_utf8(&[received]).unwrap()).unwrap();
    }

    // PUT A TEST HERE THAT WILL SHOW FAILURE. ASSERT SEEMS TO PANIC HALT SO ...

    // Trigger a breakpoint to inspect the values
    //asm::bkpt();

}
