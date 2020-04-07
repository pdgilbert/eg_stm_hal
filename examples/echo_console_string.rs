//! Serial DMA RX transfer. Read 15 chars input from console on USART1, output to semihost. Repeat.
//! There is no echo on the console and it does not handle fast typing.
//! 
//! See examples/serial_loopback_char.rs for notes about connecting usart1 to 
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
use cortex_m_semihosting::hprintln;

use eg_stm_hal::to_str;

#[cfg(feature = "stm32f1xx")]  //  eg blue pill stm32f103
use stm32f1xx_hal::{prelude::*,   pac::Peripherals, serial::{Config, Serial, StopBits}, };

#[cfg(feature = "stm32f3xx")]  //  eg Discovery-stm32f303
use stm32f3xx_hal::{prelude::*, stm32::Peripherals, serial::{Serial}, };

#[cfg(feature = "stm32f4xx")] // eg Nucleo-64  stm32f411
use stm32f4xx_hal::{prelude::*, stm32::Peripherals, serial::{config::Config, Serial}};

#[cfg(feature = "stm32l1xx") ] // eg  Discovery kit stm32l100 and Heltec lora_node STM32L151CCU6
use stm32l1xx_hal::{prelude::*,   pac::Peripherals, };


#[entry]
fn main() -> ! {
     
    //see serial_char.rs and  echo_console_by_char.rs for additional comments.

    let p = Peripherals::take().unwrap();


    #[cfg(any(feature = "stm32f1xx", feature = "stm32l1xx"))]
    let mut rcc = p.RCC.constrain();
    #[cfg(any(feature = "stm32f1xx", feature = "stm32l1xx"))]
    let mut gpioa = p.GPIOA.split(&mut rcc.apb2);
    #[cfg(any(feature = "stm32f1xx", feature = "stm32l1xx"))]
    let txrx1 = Serial::usart1(
        p.USART1,
        (gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh),  gpioa.pa10),
        &mut p.AFIO.constrain(&mut rcc.apb2).mapr,
        Config::default() .baudrate(9600.bps()) .stopbits(StopBits::STOP1),
        rcc.cfgr.freeze(&mut p.FLASH.constrain().acr),  //clocks
        &mut rcc.apb2,
    );


    #[cfg(feature = "stm32f3xx")]
    let mut rcc = p.RCC.constrain();
    #[cfg(feature = "stm32f3xx")]
    let mut gpioa = p.GPIOA.split(&mut rcc.ahb);  //ahb ?
    #[cfg(feature = "stm32f3xx")]
    let txrx1 = Serial::usart1(
        p.USART1,
        (gpioa.pa9.into_alternate_af7(),  gpioa.pa10.into_alternate_af7()),
        9600.bps(),
        rcc.cfgr.freeze(&mut p.FLASH.constrain().acr), //clocks
        &mut rcc.apb2,
    );


    #[cfg(feature = "stm32f4xx")]
    let mut rcc = p.RCC.constrain();
    //let clocks = rcc.cfgr.freeze();
    #[cfg(feature = "stm32f4xx")]
    let gpioa = p.GPIOA.split();
    #[cfg(feature = "stm32f4xx")]
    p.USART1.cr1.modify(|_,w| w.rxneie().set_bit());  //need RX interrupt? 
    //let (tx,rx) = 
    #[cfg(feature = "stm32f4xx")]
    let txrx1 =  Serial::usart1(
        p.USART1,
    	(gpioa.pa9.into_alternate_af7(),  gpioa.pa10.into_alternate_af7()), 
    	Config::default() .baudrate(9600.bps()),
    	p.RCC.constrain().cfgr.freeze(), //clocks
    ).unwrap(); 
    


 
    // Split the serial struct into a receiving and a transmitting part
    let mut tx1             = txrx1.split().0;  
   
    // This works with bluepill but not ...
    let mut bufrx = (singleton!(: [u8; 15] = [0; 15]).unwrap(),
                     txrx1.split().1);
    //               txrx1.split().1.with_dma(channels.5));

    hprintln!("Use ^C in gdb to exit.").unwrap();

    //each pass in loop waits for input of 15 chars typed in console

    loop { 
        bufrx = bufrx.1.read().wait();  
        hprintln!("received {:?}", to_str(bufrx.0)).unwrap();
    }
}
