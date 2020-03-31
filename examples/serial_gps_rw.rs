//! Serial interface read GPS on usart3 and write on usart1 to USB-TTL to console (minicom) and to semihost 
//!
//! THESE ARE BLUE PILL PIN NUMBERS. Confirm pin numbers for other boards.
//! usart3 connect the Rx pin pb11 to the Tx pin of GPS 
//! usart3 connect the Tx pin pb10 to the Rx pin of GPS
//! GPS works  by default at 9600bps. This can be confirmed by connecting GPS 
//!  directly to the  USB-TTL and terminal with these settings (minicom 8-N-1) 
//! I use 8 bit, odd parity, 1 stopbit (minicom 8-N-1), but only 9600bps seems important. 
//! 
//! See examples/serial_loopback_char.rs for notes about connecting usart1 to 
//!   serial-usb converter on computer for console output.
//! That file also has more notes regarding setup below.

#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[cfg(debug_assertions)]
extern crate panic_semihosting;

#[cfg(not(debug_assertions))]
extern crate panic_halt;

//use cortex_m::asm;
use cortex_m::{asm, singleton};
use cortex_m_rt::entry;
//use core::fmt::Write;
use cortex_m_semihosting::hprintln;
//use core::str;
//use core::ascii;
//use nb::block;

use eg_stm_hal::to_str;

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

    hprintln!("{}", to_str("just checking to_str".as_bytes())).expect("hprintln error."); 
    hprintln!("{:?}",      "just checking to_str".as_bytes()).expect("hprintln error."); 

    //see examples/serial_loopback_char.rs for more notes regarding this setup.
    let p = Peripherals::take().unwrap();
    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut afio = p.AFIO.constrain(&mut rcc.apb2);
    let mut gpioa = p.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = p.GPIOB.split(&mut rcc.apb2);

    let channels = p.DMA1.split(&mut rcc.ahb);

    //see examples/serial_loopback_char.rs for more USART config notes.
    //    USART    (tx, rx)

    //  usart1 sending to console 
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
    // WHAT IS  rcc.apb1/2 ?

    // Split the serial struct into a receiving and a transmitting part

    let (tx1, _rx1) = serial1.split();   
    let tx1 = tx1.with_dma(channels.4);    
    //let rx1 = rx1.with_dma(channels.5);

    //let (tx3, rx3) = serial3.split();   
    //let tx3 = tx3.with_dma(channels.2);    
    //let rx3 = rx3.with_dma(channels.3);
    let rx3 = serial3.split().1.with_dma(channels.3);

    // setup buffer 
    
    let (bufs, tx1) = tx1.write(b"First line of simple test writing to tx1.\r\n").wait();
    hprintln!("sent console {}", to_str(bufs)).unwrap();
    let (bufs, tx1) = tx1.write(b" and second line.").wait();
    hprintln!("sent console {}", to_str(bufs)).unwrap();


    let buf0 = singleton!(: [u8; 8] = [0; 8]).unwrap();
      
    // read gps on usart3
    hprintln!("about to read GPS").unwrap();
    let (buf, _rx3) = rx3.read(buf0).wait();  // STALLS HERE WAITING
    hprintln!("done reading GPS").unwrap();

    // write  buf (in bytes ) to semihost
    hprintln!("should echo buf to semihost here").unwrap();
    hprintln!("buf is {:?}", buf).unwrap();  // eg [44, 186, 53, 19, 114, 54, 76, 202]

    // write  buf (in bytes ) to console on tx1
    let (buf, _tx1) = tx1.write(buf).wait(); 
    hprintln!("sent console {:?}", buf).unwrap(); //would not need :? if these were not [u8; 8]


    // convert buf from bytes to characters ??
    //let rxstr = buf.to_owned();                
    //let rxstr = buf.clone();                
    //let rxstr = buf.copy;                
    //let rxstr = to_str(buf);                
    //let rxstr = ascii::escape_default(buf);  
    //let rxstr = ascii::from(buf);  

    asm::bkpt();
    hprintln!("going into empty loop ...").unwrap();
    loop { 
    }

    // PUT A TEST HERE THAT WILL SHOW FAILURE. ASSERT SEEMS TO PANIC HALT SO ...
}
