//! Serial interface read GPS on usart2 and write on usart1 to USB-TTL to console (minicom) and to semihost.
//! This example is similar to gps_rw_by_char but tries to buffer strings of data.
//! See example is gps_rw_by_char for usart settings and pin connections.

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
use stm32f3xx_hal::{prelude::*, stm32::Peripherals, serial::{Serial}, };

#[cfg(feature = "stm32f4xx")] // eg Nucleo-64  stm32f411
use stm32f4xx_hal::{prelude::*, stm32::Peripherals, serial::{config::Config, Serial, config::StopBits}};

#[cfg(feature = "stm32l1xx") ] // eg  Discovery kit stm32l100 and Heltec lora_node STM32L151CCU6
use stm32l1xx_hal::{prelude::*,   pac::Peripherals, serial::{Config, Serial, StopBits}, };


#[entry]

fn main() -> ! {

    //see serial_loopback_char.rs and serial_cross.rs in examples/ for more USART config notes.
    //    USART    (tx, rx)

    hprintln!("{}", to_str("just checking to_str".as_bytes())).expect("hprintln error."); 
    hprintln!("{:?}",      "just checking to_str".as_bytes()).expect("hprintln error."); 

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
    // WHAT IS  rcc.apb1/2 ?

    #[cfg(any(feature = "stm32f1xx", feature = "stm32l1xx"))]
    let txrx2 = Serial::usart2(
        p.USART2,
        (gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl),   gpioa.pa3),  // (tx, rx)
        &mut afio.mapr,
        Config::default() .baudrate(9_600.bps())  .parity_odd() .stopbits(StopBits::STOP1),
        clocks,
        &mut rcc.apb1,
    );



    #[cfg(feature = "stm32f3xx")]
    let mut rcc = p.RCC.constrain();
    #[cfg(feature = "stm32f3xx")]
    let clocks = rcc.cfgr.freeze(&mut p.FLASH.constrain().acr);
    #[cfg(feature = "stm32f3xx")]
    let mut gpioa = p.GPIOA.split(&mut rcc.apb2);
    #[cfg(feature = "stm32f3xx")]
    let mut gpiob = p.GPIOB.split(&mut rcc.apb2);

    #[cfg(feature = "stm32f3xx")]
    let txrx1 = Serial::usart1(
        p.USART1,
        (gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh),  gpioa.pa10),
        Config::default() .baudrate(9600.bps()) .stopbits(StopBits::STOP1),
        clocks,
        &mut rcc.apb2,
    );

    #[cfg(feature = "stm32f3xx")]
    let txrx2 = Serial::usart2(
        p.USART2,
        (gpioa.pa2.into_af7(&mut gpioa.moder, &mut gpioa.afrl), gpioa.pa3.into_af7(&mut gpioa.moder, &mut gpioa.afrl)), //(tx,rx)
        115_200.bps(),
        clocks,
        &mut rcc.apb1,
    );


    #[cfg(feature = "stm32f4xx")]
    let mut rcc = p.RCC.constrain();
    #[cfg(feature = "stm32f4xx")]
    let clocks =  rcc.cfgr.freeze();
    #[cfg(feature = "stm32f4xx")]
    let mut gpioa = p.GPIOA.split();
    #[cfg(feature = "stm32f4xx")]
    let mut gpiob = p.GPIOB.split();
    #[cfg(feature = "stm32f4xx")]
    let txrx1 = Serial::usart1(
        p.USART1,
        (gpioa.pa9.into_alternate_af7(),  gpioa.pa10.into_alternate_af7()),
        Config::default() .baudrate(9600.bps()) .stopbits(StopBits::STOP1),
        clocks,
    );

   #[cfg(feature = "stm32f4xx")]
    let txrx2 = Serial::usart2(
        p.USART2,
        ( gpioa.pa2.into_alternate_af7(),   gpioa.pa3.into_alternate_af7()),  // (tx, rx)
        Config::default() .baudrate(115_200.bps()),  //  .parity_odd() .stopbits(StopBits::STOP1),
        clocks,
    ).unwrap();


    // Split the serial struct into a receiving and a transmitting part

    //let mut tx1     = txrx1.split().0;  
    let (tx1, _rx1) = txrx1.split();   
    //let tx1 = tx1.with_dma(channels.4);    

    let rx2 = txrx2.split().1; 
    //let (mut tx2, mut rx2)  = txrx2.split();

    // setup buffer 
    
    let (bufs, tx1) = tx1.write(b"Hello to console.\r\n").wait();
    hprintln!("sent console {}", to_str(bufs)).unwrap();

    let buf0 = singleton!(: [u8; 8] = [0; 8]).unwrap();
      
    // read gps on usart2
    hprintln!("about to read GPS").unwrap();
    let (buf, _rx2) = rx2.read(buf0).wait();  
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
