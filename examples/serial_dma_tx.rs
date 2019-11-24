//! Test write to console (eg. minicom) on serial USART1 (pins pa9, pa10) with DMA.
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
use heapless::Vec;

//  eg blue pill stm32f103
#[cfg(any(feature = "stm32f100",  feature = "stm32f101", feature = "stm32f103" ))]
use stm32f1xx_hal::{ prelude::*, pac, serial::{Config, Serial, StopBits}, };

//  eg Discovery-stm32f303
//use alt_stm32f30x_hal::{  ??
#[cfg(any(feature = "stm32f301",  feature = "stm32f302", feature = "stm32f303"))]
use stm32f3xx_hal::{ prelude::*, pac, serial::{Config, Serial, StopBits}, };

// eg Nucleo-64  stm32f411
#[cfg(feature = "stm32f411")]
use stm32f4xx_hal::{ prelude::*, pac, serial::{Config, Serial, StopBits}, };

// eg  Discovery kit stm32l100 and Heltec lora_node STM32L151CCU6
#[cfg(any(feature = "stm32l100",   feature = "stnm32l151" )) ]
use stm32l1xx_hal::{ prelude::*, pac, serial::{Config, Serial, StopBits}, };


#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = p.AFIO.constrain(&mut rcc.apb2);
    let channels = p.DMA1.split(&mut rcc.ahb);

    let mut gpioa = p.GPIOA.split(&mut rcc.apb2);
    // let mut gpiob = p.GPIOB.split(&mut rcc.apb2);

    let serial = Serial::usart1(
        p.USART1,
        (gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh),   gpioa.pa10),
        &mut afio.mapr,
        Config::default()
           .baudrate(9_600.bps()) .parity_odd() .stopbits(StopBits::STOP1),
        clocks,
        &mut rcc.apb2,
    );

    let tx = serial.split().0.with_dma(channels.4);
    
    let (_, tx) = tx.write(b"The quick brown fox").wait(); // static byte works but not very flexible

    //let text = ["The ", "quick ", "brown ", "fox" ];  t in iter doesn't have a size known at compile-time
    //let text = ("The ", "quick ", "brown ", "fox" );
    //let txt = ["The ", "quick ", "brown ", "fox" ];
    //let text :Vec<str> = ["The ", "quick ", "brown ", "fox" ];
    //let text :Vec<&str> = txt.iter().map(AsRef::as_ref).collect();

    //for t in text.iter() {
    //   let (_, tx) = tx.write(t).wait();
    //   let (_, tx) = tx.write(t.as_bytes()).wait();
    //   hprintln!("sent {:?}. ", t).unwrap();
    //}

    let (_, tx) = tx.write(b" jumps").wait();
    tx.write(b" over the lazy dog.").wait();

    //asm::bkpt();

    hprintln!("entering empty loop.").unwrap();
    loop {}
}
