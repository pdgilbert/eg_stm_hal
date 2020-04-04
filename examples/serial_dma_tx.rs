//! Write to console (eg. minicom) on USART1 (pins pa9, pa10) with DMA.
//! Compare with stm32f3xx_hal  serial-dma  examples.
//! Verify minicom settings correspond to code here (8-N-1)

#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[cfg(debug_assertions)]
extern crate panic_semihosting;

#[cfg(not(debug_assertions))]
extern crate panic_halt;

//use cortex_m::asm;
use cortex_m_rt::entry;

use cortex_m_semihosting::hprintln;
//use heapless::Vec;

#[cfg(feature = "stm32f1xx")]  //  eg blue pill stm32f103
use stm32f1xx_hal::{prelude::*,   pac::Peripherals, serial::{Config, Serial, StopBits}, };

#[cfg(feature = "stm32f3xx")]  //  eg Discovery-stm32f303
use stm32f3xx_hal::{prelude::*, stm32::Peripherals, serial::{Config, Serial, StopBits}, };

#[cfg(feature = "stm32f4xx")] // eg Nucleo-64  stm32f411
use stm32f4xx_hal::{prelude::*, stm32::Peripherals, serial::{config::Config, Serial, config::StopBits}};

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
    let mut gpioa = p.GPIOA.split(&mut rcc.apb2);
    #[cfg(any(feature = "stm32f1xx", feature = "stm32l1xx"))]
    let txrx = Serial::usart1(
        p.USART1,
        (gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh),   gpioa.pa10),
        &mut p.AFIO.constrain(&mut rcc.apb2).mapr,
        Config::default() .baudrate(9_600.bps()) .parity_odd() .stopbits(StopBits::STOP1),
        clocks,
        &mut rcc.apb2,
    );
    #[cfg(any(feature = "stm32f1xx", feature = "stm32l1xx"))]
    let channels = p.DMA1.split(&mut rcc.ahb);


    #[cfg(feature = "stm32f3xx")]
    let mut rcc = p.RCC.constrain();
    #[cfg(feature = "stm32f3xx")]
    let clocks = rcc.cfgr.freeze(&mut p.FLASH.constrain().acr);
    #[cfg(feature = "stm32f3xx")]
    let mut gpioa = p.GPIOA.split(&mut rcc.ahb);  //ahb ?
    #[cfg(feature = "stm32f3xx")]
    let txrx = Serial::usart1(
        p.USART1,
        (gpioa.pa9.into_alternate_af7(),  gpioa.pa10.into_alternate_af7()),
        9600.bps(),
        clocks,
        &mut rcc.apb2,
    );
    #[cfg(feature = "stm32f3xx")]
    let channels = p.DMA1.split(&mut rcc.ahb);


    #[cfg(feature = "stm32f4xx")]
    let clocks = p.RCC.constrain().cfgr.freeze();
    //let clocks = rcc.cfgr.freeze();
    #[cfg(feature = "stm32f4xx")]
    let gpioa = p.GPIOA.split();
    #[cfg(feature = "stm32f4xx")]
    p.USART1.cr1.modify(|_,w| w.rxneie().set_bit());  //need RX interrupt? 
    //let (tx,rx) = 
    // See examples/serail_cross.rs for stm32f411re uart and alternate function notes.
    #[cfg(feature = "stm32f4xx")]
    let txrx =  Serial::usart1(
        p.USART1,
    	(gpioa.pa9.into_alternate_af7(),  gpioa.pa10.into_alternate_af7()), 
    	Config::default() .baudrate(9600.bps()),
    	clocks
    ).unwrap(); 
    #[cfg(feature = "stm32f4xx")]
    let channels = p.DMA1.split(&mut rcc.ahb);
   

    let tx = txrx.split().0.with_dma(channels.4);   
    let (_, tx) = tx.write(b"The quick brown fox").wait(); // static byte works but not very flexible
    let (_, tx) = tx.write(b" jumps").wait();
    tx.write(b" over the lazy dog.").wait();
    //asm::bkpt();
    hprintln!("entering empty loop.").unwrap();
    loop {}
}
