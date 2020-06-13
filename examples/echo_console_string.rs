//! Serial DMA RX transfer. Read 15 chars input from console on USART1, output to semihost. Repeat.
//! There is no echo on the console and it does not handle fast typing.
//! 
//! See examples/serial_char.rs for notes about connecting usart1 to 
//!   serial ttl-usb converter on computer for console output.
//! That file also has more notes regarding setup.

#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[cfg(debug_assertions)]
extern crate panic_semihosting;

#[cfg(not(debug_assertions))]
extern crate panic_halt;

use cortex_m::singleton;
use cortex_m_rt::entry;
use core::fmt::Write;  // for writeln
use cortex_m_semihosting::hprintln;

use eg_stm_hal::to_str;

#[cfg(feature = "stm32f1xx")]  //  eg blue pill stm32f103
use stm32f1xx_hal::{prelude::*,   pac::Peripherals, serial::{Config, Serial, StopBits}, };

#[cfg(feature = "stm32f3xx")]  //  eg Discovery-stm32f303
use stm32f3xx_hal::{prelude::*, stm32::Peripherals, serial::{Serial}, };

#[cfg(feature = "stm32f4xx")] // eg Nucleo-64  stm32f411
use stm32f4xx_hal::{prelude::*, stm32::Peripherals, serial::{config::Config, Serial}};

#[cfg(feature = "stm32l1xx") ] // eg  Discovery kit stm32l100 and Heltec lora_node STM32L151CCU6
use stm32l1xx_hal::{prelude::*, stm32::Peripherals, serial::{Config, Serial}};


#[entry]
fn main() -> ! {
     
    //see serial_char.rs and  echo_console_by_char.rs for additional comments.
    
    let p = Peripherals::take().unwrap();

    let mut rcc = p.RCC.constrain();

    #[cfg(feature = "stm32f1xx")]
    let clocks    = rcc.cfgr.freeze(&mut p.FLASH.constrain().acr);
    #[cfg(feature = "stm32f1xx")]
    let mut gpioa = p.GPIOA.split(&mut rcc.apb2);
    #[cfg(feature = "stm32f1xx")]
    let txrx1 = Serial::usart1(
        p.USART1,
        (gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh), 
	 gpioa.pa10),
        &mut p.AFIO.constrain(&mut rcc.apb2).mapr,
        Config::default() .baudrate(9600.bps()) .stopbits(StopBits::STOP1),
        clocks,
        &mut rcc.apb2,
    );
    #[cfg(feature = "stm32f1xx")]
    let channels = p.DMA1.split(&mut rcc.ahb);
    #[cfg(feature = "stm32f1xx")]
    let (mut tx1, mut rx1)  = txrx1.split();
    #[cfg(feature = "stm32f1xx")]
    let rx1 =rx1.with_dma(channels.5);


    #[cfg(feature = "stm32f3xx")]
    let clocks    = rcc.cfgr.freeze(&mut p.FLASH.constrain().acr);
    #[cfg(feature = "stm32f3xx")]
    let mut gpioa = p.GPIOA.split(&mut rcc.ahb); 
    #[cfg(feature = "stm32f3xx")]
    let txrx1 = Serial::usart1(
        p.USART1,
        (gpioa.pa9.into_af7( &mut gpioa.moder, &mut gpioa.afrh), 
	 gpioa.pa10.into_af7(&mut gpioa.moder, &mut gpioa.afrh)),
        9600.bps(),
        clocks,
        &mut rcc.apb2,
    );
    #[cfg(feature = "stm32f3xx")]
    let (mut tx1, mut rx1)  = txrx1.split();


    #[cfg(feature = "stm32f4xx")]
    let clocks = rcc.cfgr.freeze();
    #[cfg(feature = "stm32f4xx")]
    let gpioa = p.GPIOA.split();
    #[cfg(feature = "stm32f4xx")]
    p.USART1.cr1.modify(|_,w| w.rxneie().set_bit());  //need RX interrupt? 
    //let (tx,rx) = 
    #[cfg(feature = "stm32f4xx")]
    let txrx1 =  Serial::usart1(
        p.USART1,
    	(gpioa.pa9.into_alternate_af7(), 
	 gpioa.pa10.into_alternate_af7()), 
    	Config::default() .baudrate(9600.bps()),
    	clocks,
    ).unwrap();    
    #[cfg(feature = "stm32f4xx")]
    let (mut tx1, mut rx1)  = txrx1.split();


    #[cfg(feature = "stm32l1xx")]
    let clocks = rcc.cfgr.freeze();
    #[cfg(feature = "stm32l1xx")]
    let gpioa = p.GPIOA.split();
    #[cfg(feature = "stm32l1xx")]
    p.USART1.cr1.modify(|_,w| w.rxneie().set_bit());  //need RX interrupt? 
    //let (tx,rx) = 
    #[cfg(feature = "stm32l1xx")]
    let txrx1 =  Serial::usart1(
        p.USART1,
    	(gpioa.pa9.into_alternate_af7(),
    	 gpioa.pa10.into_alternate_af7()), 
    	Config::default() .baudrate(9600.bps()),
    	clocks,
    ).unwrap();    
    #[cfg(feature = "stm32l1xx")]
    let (mut tx1, mut rx1)  = txrx1.split();


    // END COMMON USART SETUP

    writeln!(tx1, "\r\ncheck console output.\r\n").unwrap();

    //let mut buf = [0u8; 64];
    let buf = singleton!(: [u8; 15] = [0; 15]).unwrap();
    let mut bufrx = (buf,  rx1);

    hprintln!("Enter 15 characters in console. Repeat.").unwrap();
    hprintln!("Use ^C in gdb to exit.").unwrap();

    writeln!(tx1, "\r\nEnter 15 characters here in the console. Repeat.\r\n").unwrap();

    //#[cfg(feature = "stm32f1xx")]
    bufrx = bufrx.1.read(bufrx.0).wait();  
    //else
    //bufrx = rx1.read();
    //serial.read(&mut buf)
   
    hprintln!("received {:?}", to_str(bufrx.0)).unwrap();
    writeln!(tx1, "{}", to_str(bufrx.0)).unwrap();
    //bt = tx1.write(bufrx.0).wait(); 
    //let (_, mut tx1) = tx1.write(bufrx.0).wait();

    // cannot get this to work in loop as (buf, rx1), there seem to be circular problems
    // with move/borrow/mut  but something like this works ...
 
    //each pass in loop waits for input of 15 chars typed in console
    loop { 
       //#[cfg(feature = "stm32f1xx")]  //removing ... not supported in this position
       bufrx = bufrx.1.read(bufrx.0).wait();  
       //else
       //bufrx = bufrx.read();
       hprintln!("received {:?}", to_str(bufrx.0)).unwrap();
       writeln!(tx1, "{}", to_str(bufrx.0)).unwrap();
       }
}
