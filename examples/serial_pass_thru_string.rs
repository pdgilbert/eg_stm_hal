//! Serial DMA RX transfer. Read 15 chars input from console on USART1, output to semihost. Repeat.
//! There is no echo on the console and it does not handle fast typing.
//! 
//! See examples/serial_loopback_char_test.rs for notes about connecting usart1 to 
//!   serial ttl-usb converter on computer for console output.
//! That file also has for more notes regarding setup.

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

//  eg blue pill stm32f103
#[cfg(any(feature = "stm32f100",  feature = "stm32f101", feature = "stm32f103" ))]
use stm32f1xx_hal::{prelude::*, pac::Peripherals, serial::{Config, Serial, StopBits}};

//  eg Discovery-stm32f303
//use alt_stm32f30x_hal::{  ??
#[cfg(any(feature = "stm32f301",  feature = "stm32f302", feature = "stm32f303"))]
use stm32f3xx_hal::{prelude::*, pac::Peripherals, serial::{Config, Serial, StopBits}};

// eg Nucleo-64  stm32f411
#[cfg(feature = "stm32f411")]
use stm32f4xx_hal::{prelude::*, stm32::Peripherals, serial::{Config, Serial, StopBits}};

// eg  Discovery kit stm32l100 and Heltec lora_node STM32L151CCU6
#[cfg(any(feature = "stm32l100",   feature = "stnm32l151" )) ]
use stm32l1xx_hal::{prelude::*, pac::Peripherals, serial::{Config, Serial, StopBits}};


#[entry]
fn main() -> ! {
    
    let p = Peripherals::take().unwrap();
    let mut flash = p.FLASH.constrain();
    let mut rcc   = p.RCC.constrain();
    let clocks    = rcc.cfgr.freeze(&mut flash.acr);
    let mut afio  = p.AFIO.constrain(&mut rcc.apb2);
    let channels  = p.DMA1.split(&mut rcc.ahb);
    let mut gpioa = p.GPIOA.split(&mut rcc.apb2);

    //see examples/serial_loopback_char_test.rs for more USART config notes.

    let serial = Serial::usart1(
        p.USART1,
        (gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh),   gpioa.pa10),
        &mut afio.mapr,
        Config::default() .baudrate(9_600.bps()) .parity_odd() .stopbits(StopBits::STOP1),
        clocks,
        &mut rcc.apb2,
    );

    
    // cannot get this to work in loop as (buf, rx) so ...
    let mut bufrx = (singleton!(: [u8; 15] = [0; 15]).unwrap(),
                     serial.split().1.with_dma(channels.5));

    hprintln!("Use ^C in gdb to exit.").unwrap();

    //each pass in loop waits for input of 15 chars typed in console

    loop { 
        bufrx = bufrx.1.read(bufrx.0).wait();  
        hprintln!("received {:?}", to_str(bufrx.0)).unwrap();
    }
}
