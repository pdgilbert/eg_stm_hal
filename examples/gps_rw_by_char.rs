//! Serial interface char-by-char read GPS on usart2 and write on usart1 
//! to USB-TTL connected to console (minicom) and also write to semihost. 
//!
//! usart1 connect the Tx pin pa9  to the Rx pin of a serial-usb converter
//! usart1 connect the Rx pin pa10 to the Tx pin of a serial-usb converter
//! Set up the serial console (e.g. minicom) with the same settings used here.
//! (Using 9600bps, could be higher but needs serial console to be the same.)
//!
//! GPS uses 9600bps, 8bit, odd parity, 1 stopbit. This can be confirmed by connecting GPS 
//!  directly to the  USB-TTL and terminal with these settings (minicom 8-N-1) 
//! usart2 connect the Rx pin pa3 to the Tx pin of GPS 
//! usart2 connect the Tx pin pa2 to the Rx pin of GPS
//! 
//! See examples/serial_char.rs for notes about connecting usart1 to 
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
//use core::str::from_utf8;
use nb::block;

#[cfg(feature = "stm32f1xx")]  //  eg blue pill stm32f103
use stm32f1xx_hal::{prelude::*,   pac::Peripherals, serial::{Config, Serial, StopBits}, };

#[cfg(feature = "stm32f3xx")]  //  eg Discovery-stm32f303
use stm32f3xx_hal::{prelude::*, stm32::Peripherals, serial::{Serial}, };
//use stm32f3xx_hal::{prelude::*, stm32::Peripherals, serial::{Config, Serial, StopBits}, };

#[cfg(feature = "stm32f4xx")] // eg Nucleo-64  stm32f411
use stm32f4xx_hal::{prelude::*, pac::Peripherals, serial::{config::Config, Serial }};

#[cfg(feature = "stm32l1xx") ] // eg  Discovery kit stm32l100 and Heltec lora_node STM32L151CCU6
use stm32l1xx_hal::{prelude::*, stm32::Peripherals, serial::{Config, Serial}};


#[entry]
fn main() -> ! {
 
    //see serial_char.rs and serial_string.rs in examples/ for more USART config notes.

    let p = Peripherals::take().unwrap();

    #[cfg(feature = "stm32f1xx")]
    let mut rcc = p.RCC.constrain();
    #[cfg(feature = "stm32f1xx")]
    let clocks = rcc.cfgr.freeze(&mut p.FLASH.constrain().acr);
    #[cfg(feature = "stm32f1xx")]
    let mut afio = p.AFIO.constrain(&mut rcc.apb2);
    #[cfg(feature = "stm32f1xx")]
    let mut gpioa = p.GPIOA.split(&mut rcc.apb2);
    #[cfg(feature = "stm32f1xx")]
    let txrx1 = Serial::usart1(
        p.USART1,
        (gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh),  gpioa.pa10),
        &mut afio.mapr,
        Config::default() .baudrate(9600.bps()) .stopbits(StopBits::STOP1),    // .parity_odd() 
        clocks,
        &mut rcc.apb2,
    );

    #[cfg(feature = "stm32f1xx")]
    let txrx2 = Serial::usart2(
        p.USART2,
        (gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl),   gpioa.pa3),  // (tx, rx)
        &mut afio.mapr,
        Config::default() .baudrate(9_600.bps()) ,
        clocks,
        &mut rcc.apb1,
    );



    #[cfg(feature = "stm32f3xx")]
    let mut rcc   = p.RCC.constrain();
    #[cfg(feature = "stm32f3xx")]
    let clocks    = rcc.cfgr.freeze(&mut p.FLASH.constrain().acr);
    #[cfg(feature = "stm32f3xx")]
    let mut gpioa = p.GPIOA.split(&mut rcc.ahb);
    //#[cfg(feature = "stm32f3xx")]
    //let mut gpiob = p.GPIOB.split(&mut rcc.ahb);

    #[cfg(feature = "stm32f3xx")]
    let txrx1     = Serial::usart1(
        p.USART1,
        (gpioa.pa9.into_af7(&mut gpioa.moder, &mut gpioa.afrh),
	 gpioa.pa10.into_af7(&mut gpioa.moder, &mut gpioa.afrh)),
        9600.bps(),
        clocks,
        &mut rcc.apb2,
    );

    #[cfg(feature = "stm32f3xx")]
    let txrx2 = Serial::usart2(
        p.USART2,
        (gpioa.pa2.into_af7(&mut gpioa.moder, &mut gpioa.afrl),
	 gpioa.pa3.into_af7(&mut gpioa.moder, &mut gpioa.afrl)), //(tx,rx)
        9600.bps(),
        clocks,
        &mut rcc.apb1,
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
    let txrx2 = Serial::usart2(
        p.USART2,
        ( gpioa.pa2.into_alternate_af7(),   gpioa.pa3.into_alternate_af7()),  // (tx, rx)
        Config::default() .baudrate(9600.bps()),  
        //Config::default() .baudrate(115_200.bps()),  .parity_odd() .stopbits(StopBits::STOP1),
        clocks,
    ).unwrap();


    #[cfg(feature = "stm32l1xx")]
    let clocks    =  p.RCC.constrain().cfgr.freeze();
    #[cfg(feature = "stm32l1xx")]
    let gpioa = p.GPIOA.split();
    #[cfg(feature = "stm32l1xx")]
    let txrx1 = Serial::usart1(
        p.USART1,
        (gpioa.pa9.into_alternate_af7(),  gpioa.pa10.into_alternate_af7()),
     	Config::default() .baudrate(9600.bps()),
        clocks,
    ).unwrap();

    #[cfg(feature = "stm32l1xx")]
    let txrx2 = Serial::usart2(
        p.USART2,
        ( gpioa.pa2.into_alternate_af7(),   gpioa.pa3.into_alternate_af7()),  // (tx, rx)
        Config::default() .baudrate(9600.bps()), 
	// Config::default()  .baudrate(115_200.bps()), .parity_odd() .stopbits(StopBits::STOP1),
        clocks,
    ).unwrap();


    // END COMMON USART SETUP


    // Split the serial struct into a receiving and a transmitting part
    let (mut  tx1, mut _rx1) = txrx1.split();  // console
    let (mut _tx2, mut  rx2) = txrx2.split();  // GPS

    hprintln!("testing console output...").unwrap();
 
    for byte in b"Just confirming console works.\r\n" {
       block!(tx1.write(*byte)).unwrap();
    }

    hprintln!("entering read/write loop...").unwrap();

    // note that putting hprintln! in loop slows it too much and loses data.
    let e: u8 = 9;
    loop { // Read a byte and write
      let received = match block!(rx2.read()) {
         Ok(str)     => str,
         Err(_error) => e,
         };
      block!(tx1.write(received)).ok();
    }

    // PUT A TEST HERE THAT WILL SHOW FAILURE. ASSERT SEEMS TO PANIC HALT SO ...

    // Trigger a breakpoint to inspect the values
    //asm::bkpt();

}
