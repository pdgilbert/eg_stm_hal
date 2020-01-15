//! Serial interface write with usart2 and read on usart3, echo on usart1 to
//! console at 9600 bps and echo on semihost.
//!
//! THESE ARE BLUE PILL PIN NUMBERS. CONFIRM PIN NUMBERS OF OTHER BOARDS
//! usart3 connect the Rx pin pb11 to usart2 TX pin pa2.   
//! usart3 connect the Tx pin pb10 to usart2 Rx pin pa3.
//! Connect usart1  to serial-usb converter on computer for console output:
//! usart1 connect the Tx pin pa9  to the Rx pin of a serial-usb converter
//! usart1 connect the Rx pin pa10 to the Tx pin of a serial-usb converter
//! 
//! See examples/serial_loopback_char_test.rs for notes about connecting usart1 to 
//!   serial-usb converter on computer for console output.
//! That file also has for more notes regarding setup below.


#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[cfg(debug_assertions)]
extern crate panic_semihosting;

#[cfg(not(debug_assertions))]
extern crate panic_halt;

use cortex_m::{asm, singleton};
use cortex_m_rt::entry;
//use core::fmt::Write;
use cortex_m_semihosting::hprintln;
//use core::str;
//use core::ascii;
//use nb::block;

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
    
    //see examples/serial_loopback_char_test.rs for more notes regarding this setup.
    let p = Peripherals::take().unwrap();
    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut afio = p.AFIO.constrain(&mut rcc.apb2);
    let mut gpioa = p.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = p.GPIOB.split(&mut rcc.apb2);

    //see examples/serial_loopback_char_test.rs for more USART config notes.
    //    USART    (tx, rx)

    //   usart1 console
    let serial1 = Serial::usart1(
        p.USART1,
        (gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh),  gpioa.pa10),
        &mut afio.mapr,
        Config::default() .baudrate(9600.bps()) .parity_odd() .stopbits(StopBits::STOP1),
        clocks,
        &mut rcc.apb2,
    );

    //  usart2 
    let serial2 = Serial::usart2(
        p.USART2,
        (gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl),  gpioa.pa3),
        &mut afio.mapr,
        Config::default() .baudrate(115_200.bps())  .parity_odd() .stopbits(StopBits::STOP1),
        clocks,
        &mut rcc.apb1,
    );

    //  usart3 
    let serial3 = Serial::usart3(
        p.USART3,
        (gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh),  gpiob.pb11),
        &mut afio.mapr,
        Config::default() .baudrate(115_200.bps())  .parity_odd() .stopbits(StopBits::STOP1),
        clocks,
        &mut rcc.apb1,
    );
    // WHAT IS  rcc.apb1/2 ?

    // Split the serial struct into a receiving and a transmitting part
    let channels = p.DMA1.split(&mut rcc.ahb);

    let tx1 = serial1.split().0.with_dma(channels.4);  

    let (tx2, mut _rx2) = serial2.split();
    let tx2 = tx2.with_dma(channels.7);    
    //let rx2 = rx2.with_dma(channels.6);

    let (_tx3, rx3) = serial3.split();   
    //let tx3 = tx3.with_dma(channels.2);    
    let rx3 = rx3.with_dma(channels.3);

    hprintln!("Now try write ...").unwrap(); 

    // write with usart2 
    let (_, _tx2) = tx2.write(b" hello from usart2 to usart3").wait();

    hprintln!("and read.").unwrap(); 
    // read on usart3
    let buf = singleton!(: [u8; 8] = [0; 8]).unwrap();
    // SEEMS TO STALL HERE. MISSED MESSAGE AND WAITING?
    let (buf, _rx3) = rx3.read(buf).wait();  //buf mutable borrow occurs here
    let bufs = to_str(buf);                // borrow of `*buf` occurs here

    // echo on semihost.

    hprintln!("should echo buf to semihost here").unwrap();
    hprintln!("buf is {:?}", buf).unwrap();  // eg [44, 186, 53, 19, 114, 54, 76, 202]
    hprintln!("bufs is {}", bufs).unwrap();
    
    //hprintln!("received buf {:?}", to_str(buf)).unwrap();
    // these fail when conversion fails
    //hprintln!("{:?}", to_str(buf)).unwrap(); this seems to panic?
    //hprintln!("{:?}", core::str::from_utf8(buf).unwrap()).unwrap(); 

    // echo on usart1 to console.

    let (_bufsx, tx1) = tx1.write(b"\r\nShould echo buf to console here.\r\n").wait();
    tx1.write(buf).wait();   // move out of `buf` occurs here
    //tx1.write(bufs).wait();   doesn't have a size known at compile-time


    asm::bkpt();
    hprintln!("going into empty loop ...").unwrap();
    loop { 
    }

    // PUT A TEST HERE THAT WILL SHOW FAILURE. ASSERT SEEMS TO PANIC HALT SO ...
}
