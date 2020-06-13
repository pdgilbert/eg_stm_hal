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

//use cortex_m::singleton;
//or ?
use heapless::{consts, Vec};

use eg_stm_hal::to_str;

use cortex_m_rt::entry;
use core::fmt::Write;  // for writeln
use cortex_m_semihosting::hprintln;
//use core::str;
//use core::ascii;
use nb::block;

//use eg_stm_hal::to_str;

#[cfg(feature = "stm32f1xx")]  //  eg blue pill stm32f103
use stm32f1xx_hal::{prelude::*,   pac::Peripherals, serial::{Config, Serial, StopBits}, };

#[cfg(feature = "stm32f3xx")]  //  eg Discovery-stm32f303
use stm32f3xx_hal::{prelude::*, stm32::Peripherals, serial::{Serial}, };

#[cfg(feature = "stm32f4xx")] // eg Nucleo-64  stm32f411
use stm32f4xx_hal::{prelude::*, stm32::Peripherals, serial::{config::Config, Serial, config::StopBits}};

#[cfg(feature = "stm32l1xx") ] // eg  Discovery kit stm32l100 and Heltec lora_node STM32L151CCU6
use stm32l1xx_hal::{prelude::*, stm32::Peripherals, serial::{Config, Serial}};

//use heapless::{consts, Vec};

#[entry]

fn main() -> ! {

    //see serial_loopback_char.rs and serial_cross.rs in examples/ for more USART config notes.
    //    USART    (tx, rx)

    //hprintln!("{}", to_str("just checking to_str".as_bytes())).expect("hprintln error."); 
    //hprintln!("{:?}",      "just checking to_str".as_bytes()).expect("hprintln error."); 

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
        (gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh),
	 gpioa.pa10),
        &mut afio.mapr,
        Config::default() .baudrate(9600.bps()) .stopbits(StopBits::STOP1),
        clocks,
        &mut rcc.apb2,   // WHAT IS  rcc.apb1/2 ?
        );

    #[cfg(feature = "stm32f1xx")]
    let txrx2 = Serial::usart2(
        p.USART2,
        (gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl), 
	 gpioa.pa3),  // (tx, rx)
        &mut afio.mapr,
        Config::default() .baudrate(9600.bps())  .stopbits(StopBits::STOP1), //.parity_odd() 
        clocks,
        &mut rcc.apb1,
        );



    #[cfg(feature = "stm32f3xx")]
    let mut rcc = p.RCC.constrain();
    #[cfg(feature = "stm32f3xx")]
    let clocks = rcc.cfgr.freeze(&mut p.FLASH.constrain().acr);
    #[cfg(feature = "stm32f3xx")]
    let mut gpioa = p.GPIOA.split(&mut rcc.ahb);
    #[cfg(feature = "stm32f3xx")]
    let txrx1 = Serial::usart1(
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
        9600.bps(), //115_200.bps(),
        clocks,
        &mut rcc.apb1,
        );



    #[cfg(feature = "stm32f4xx")]
    let rcc = p.RCC.constrain();
    #[cfg(feature = "stm32f4xx")]
    let clocks =  rcc.cfgr.freeze();
    #[cfg(feature = "stm32f4xx")]
    let gpioa = p.GPIOA.split();
    #[cfg(feature = "stm32f4xx")]
    p.USART1.cr1.modify(|_,w| w.rxneie().set_bit());  //need RX interrupt? 
    //#[cfg(feature = "stm32f4xx")]
    //p.USART2.cr1.modify(|_,w| w.rxneie().set_bit());  //need RX interrupt? 
    //#[cfg(feature = "stm32f4xx")]
    //let mut gpiob = p.GPIOB.split();

    #[cfg(feature = "stm32f4xx")]
    let txrx1 = Serial::usart1(
        p.USART1,
        (gpioa.pa9.into_alternate_af7(), 
	 gpioa.pa10.into_alternate_af7()),
        Config::default() .baudrate(9600.bps()) .stopbits(StopBits::STOP1),
        clocks,
        ).unwrap();

    #[cfg(feature = "stm32f4xx")]
    let txrx2 = Serial::usart2(
        p.USART2,
        ( gpioa.pa2.into_alternate_af7(), 
	  gpioa.pa3.into_alternate_af7()),  // (tx, rx)
        Config::default() .baudrate(9600.bps()),  //  .parity_odd() .stopbits(StopBits::STOP1),
        clocks,
        ).unwrap();



    #[cfg(feature = "stm32l1xx")]
    let mut rcc = p.RCC.constrain();
    #[cfg(feature = "stm32l1xx")]
    let clocks =  rcc.cfgr.freeze();
    #[cfg(feature = "stm32l1xx")]
    let mut gpioa = p.GPIOA.split();
    #[cfg(feature = "stm32l1xx")]
    p.USART1.cr1.modify(|_,w| w.rxneie().set_bit());  //need RX interrupt? 
    //#[cfg(feature = "stm32l1xx")]
    //let mut gpiob = p.GPIOB.split();

    #[cfg(feature = "stm32l1xx")]
    let txrx1 = Serial::usart1(
        p.USART1,
        (gpioa.pa9.into_alternate_af7(),
	 gpioa.pa10.into_alternate_af7()),
        Config::default() .baudrate(9600.bps()),
        clocks,
        ).unwrap();

    #[cfg(feature = "stm32l1xx")]
    let txrx2 = Serial::usart2(
        p.USART2,
        ( gpioa.pa2.into_alternate_af7(), 
	  gpioa.pa3.into_alternate_af7()),  // (tx, rx)
        Config::default() .baudrate(9600.bps()),  //  .parity_odd() .stopbits(StopBits::STOP1),
        clocks,
        ).unwrap();

    // END COMMON USART SETUP

    let mut tx1 = txrx1.split().0;      // console
    let mut rx2 = txrx2.split().1;      // GPS

    writeln!(tx1, "\r\nconsole connect check.\r\n").unwrap();

    // read gps on usart2
    hprintln!("about to read GPS").unwrap();
    
    // byte buffer length 80
    let mut buffer: Vec<u8, consts::U80> = Vec::new();
    hprintln!("buffer at {} of {}", buffer.len(), buffer.capacity()).unwrap();  //0 of 100
    buffer.clear();

//    while (i < r.len()) && !buffer.push(r[i]).is_err() {
    hprintln!("going into write/read loop ^C to exit ...").unwrap();
    let e: u8 = 9;
    let mut good = false;
    loop {
        let byte = match block!(rx2.read()) {
	    Ok(byt)	  => byt,
	    Err(_error) => e,
	    };
        block!(tx1.write(byte)).ok();
        if   byte == 36  {  //  $ is 36. start of a line
	   buffer.clear();
	   good = true;     //start capturing line
	   };
	if good {
	   if buffer.push(byte).is_err() ||  byte == 13  {  //  \r is 13, \n is 10
              writeln!(tx1, "{}", to_str(&buffer)).unwrap();
              //hprintln!("buffer at {} of {}", buffer.len(), buffer.capacity()).unwrap();
              buffer.clear();
	      good = false;
	      //break; 
	      };
	   };
	}

    //asm::bkpt();


    // (buf, rx)  tuple for RxDma VS read() a single u8
    //let mut br = (singleton!(: [u8; 100] = [0; 100]).unwrap(), rx2);
    //br = br.1.read(br.0).wait();                
    //hprintln!("buf is  {:?}", br.0).unwrap(); 
    //hprintln!("to_str(buf2) is {:?}", to_str(bbr.0)).unwrap();  

    //hprintln!("about to write  buf2 to console").unwrap();
    //hprintln!("sending console {:?}", buf2).unwrap(); //would not need :? if these were not [u8; 8]

    //let bufx = buftx1.1.write(bufrx2.0); 
    //let bufx = buftx1.1.TxDma(bufrx2.0);  // wait(); //TxDma VS read  !!!!!!!!!!!!!
    //hprintln!("sent console {:?}", bufx).unwrap(); //would not need :? if these were not [u8; 8]

    // NB: bt uses br's buffer. And br use bt's buffer, but not sure why.
    //   whereas in the first read above br used it's own buffer.
    //loop {
    //   bt = bt.1.write(br.0).wait(); 
    //   br = br.1.read(bt.0).wait();
    //   }

    // PUT A TEST HERE THAT WILL SHOW FAILURE. ASSERT SEEMS TO PANIC HALT SO ...
}
