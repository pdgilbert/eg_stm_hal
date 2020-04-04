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
use stm32f3xx_hal::{prelude::*, stm32::Peripherals, serial::{Serial}, };
//use stm32f3xx_hal::{prelude::*, stm32::Peripherals, serial::{Config, Serial, StopBits}, };

#[cfg(feature = "stm32f4xx")] // eg Nucleo-64  stm32f411
use stm32f4xx_hal::{prelude::*, stm32::Peripherals, serial::{config::Config, Serial}};

#[cfg(feature = "stm32l1xx") ] // eg  Discovery kit stm32l100 and Heltec lora_node STM32L151CCU6
use stm32l1xx_hal::{prelude::*,   pac::Peripherals, serial::{Config, Serial, StopBits}, };


#[entry]

fn main() -> ! {
    
    //see examples/serial_loopback_char.rs for more notes regarding this setup.

    // Split the serial struct into a receiving and a transmitting parts is HAL specific
    // because dma does not seem to be implemented/needed? for all HALs

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
    let txrx2 = Serial::usart2(
        p.USART2,
        (gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl),   gpioa.pa3),  // (tx, rx)
        &mut afio.mapr,
        Config::default() .baudrate(115_200.bps())  .parity_odd() .stopbits(StopBits::STOP1),
        clocks,
        &mut rcc.apb1,
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
    #[cfg(any(feature = "stm32f1xx", feature = "stm32l1xx"))]
    let channels = p.DMA1.split(&mut rcc.ahb);
 
    #[cfg(any(feature = "stm32f1xx", feature = "stm32l1xx"))]
    let tx1 = txrx1.split().0.with_dma(channels.4);  

    #[cfg(any(feature = "stm32f1xx", feature = "stm32l1xx"))]
    let (tx2, mut _rx2) = txrx2.split();
    #[cfg(any(feature = "stm32f1xx", feature = "stm32l1xx"))]
    let tx2 = tx2.with_dma(channels.7);    
    //let rx2 = rx2.with_dma(channels.6);

    #[cfg(any(feature = "stm32f1xx", feature = "stm32l1xx"))]
    let (_tx3, rx3) = txrx3.split();   
    //let tx3 = tx3.with_dma(channels.2);    
    #[cfg(any(feature = "stm32f1xx", feature = "stm32l1xx"))]
    let rx3 = rx3.with_dma(channels.3);


    // stm32f303vct  alternate funtion modes see  
    // https://www.rlocman.ru/i/File/dat/STMicroelectronics/Microcontrollers_MCU/STM32F303VCT6.pdf p42
    // AF7 on PA9  is usart1_Tx, on PA10 is usart1_Rx,
    // AF7 on PA2  is usart2_Tx, on PA3  is usart2_Rx,
    // AF7 on PB10 is usart3_Tx, on PB11  is usart3_Rx,

    #[cfg(feature = "stm32f3xx")]
    let mut rcc = p.RCC.constrain();
    #[cfg(feature = "stm32f3xx")]
    let clocks = rcc.cfgr.freeze(&mut p.FLASH.constrain().acr);
    #[cfg(feature = "stm32f3xx")]
    let mut gpioa = p.GPIOA.split(&mut rcc.ahb);
    #[cfg(feature = "stm32f3xx")]
    let mut gpiob = p.GPIOB.split(&mut rcc.ahb);
    #[cfg(feature = "stm32f3xx")]
    let txrx1 = Serial::usart1(
        p.USART1,
        (gpioa.pa9.into_af7(&mut gpioa.moder, &mut gpioa.afrh),  gpioa.pa10.into_af7(&mut gpioa.moder, &mut gpioa.afrh)),
        Config::default() .baudrate(9600.bps()) .stopbits(StopBits::STOP1),
        clocks,
        &mut rcc.apb2,
    );
    #[cfg(feature = "stm32f3xx")]
    let txrx2 = Serial::usart2(
        p.USART2,
        (gpioa.pa2.into_af7(&mut gpioa.moder, &mut gpioa.afrh), gpioa.pa3.into_af7(&mut gpioa.moder, &mut gpioa.afrl)), //(tx,rx)
        Config::default() .baudrate(115_200.bps())  .parity_odd() .stopbits(StopBits::STOP1),
        clocks,
        &mut rcc.apb1,
    );
    #[cfg(feature = "stm32f3xx")]
    let txrx3 = Serial::usart3(
        p.USART3,
        (gpiob.pb10.into_af7(&mut gpiob.moder, &mut gpiob.afrh), gpiob.pb11.into_af7(&mut gpiob.moder, &mut gpiob.afrh)), 
        115_200.bps(),
        clocks,
        &mut rcc.apb1,    // WHAT IS  rcc.apb1/2 ?
    );
    #[cfg(feature = "stm32f3xx")]
    let channels = p.DMA1.split(&mut rcc.ahb);

    #[cfg(feature = "stm32f3xx")]
    let tx1 = txrx1.split().0.with_dma(channels.4);  
    #[cfg(feature = "stm32f3xx")]
    let (tx2, mut _rx2) = txrx2.split();
    #[cfg(feature = "stm32f3xx")]
    let tx2 = tx2.with_dma(channels.7);    
    #[cfg(feature = "stm32f3xx")]
    let (_tx3, rx3) = txrx3.split();   
    #[cfg(feature = "stm32f3xx")]
    let rx3 = rx3.with_dma(channels.3);



    // stm32f411re implements only usarts 1, 2, and 6. These can be configured on different pins.
    // alternate funtion modes see https://www.st.com/resource/en/datasheet/stm32f411re.pdf  p47.
    // AF7 on PA9  is usart1_Tx, on PA10 is usart1_Rx,
    // AF7 on PA2  is usart2_Tx, on PA3  is usart2_Rx,
    // AF8 on PA11 is usart6_Tx, on PA12 is usart6_Rx

    #[cfg(feature = "stm32f4xx")]
    let clocks = p.RCC.constrain().cfgr.freeze();
    #[cfg(feature = "stm32f4xx")]
    let mut gpioa = p.GPIOA.split();
    #[cfg(feature = "stm32f4xx")]
    let mut gpiob = p.GPIOB.split();
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
        Config::default() .baudrate(115_200.bps()),  //  .parity_odd() .stopbits(StopBits::STOP1),
        clocks,
    ).unwrap();
    #[cfg(feature = "stm32f4xx")]
    let txrx3 = Serial::usart6(
        p.USART6,
        ( gpioa.pa11.into_alternate_af8(),   gpioa.pa12.into_alternate_af8()),  // (tx, rx)  NOTE PINS, USART !!!
        Config::default() .baudrate(115_200.bps()) ,
        clocks,
    ).unwrap();
    #[cfg(feature = "stm32f4xx")]
    let tx1 = txrx1.split().0;  
    #[cfg(feature = "stm32f4xx")]
    let (tx2, mut _rx2) = txrx2.split();
    #[cfg(feature = "stm32f4xx")]
    let (_tx3, rx3) = txrx3.split();   


    hprintln!("Now try write ...").unwrap(); 

    // write with usart2 
    let (_, _tx2) = tx2.write(b" hello from usart2 to usart3").wait();

    hprintln!("and read.").unwrap(); 
    // read on usart3
    let buf = singleton!(: [u8; 8] = [0; 8]).unwrap();
    // bluepill SEEMS TO STALL HERE. MISSED MESSAGE AND WAITING?
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
