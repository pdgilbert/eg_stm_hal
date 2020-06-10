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
use cortex_m::singleton;
use cortex_m_rt::entry;
//use core::fmt::Write;
use cortex_m_semihosting::hprintln;
//use core::str;
//use core::ascii;
//use nb::block;

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

    let mut rcc = p.RCC.constrain();

    #[cfg(feature = "stm32f1xx")]
    let clocks = rcc.cfgr.freeze(&mut p.FLASH.constrain().acr);
    #[cfg(feature = "stm32f1xx")]
    let mut afio = p.AFIO.constrain(&mut rcc.apb2);
    #[cfg(feature = "stm32f1xx")]
    let mut gpioa = p.GPIOA.split(&mut rcc.apb2);
    //#[cfg(feature = "stm32f1xx")]
    //let mut gpiob = p.GPIOB.split(&mut rcc.apb2);
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
    let channels = p.DMA1.split(&mut rcc.ahb);

    #[cfg(feature = "stm32f1xx")]
    let txrx2 = Serial::usart2(
        p.USART2,
        (gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl), 
	 gpioa.pa3),  // (tx, rx)
        &mut afio.mapr,
        Config::default() .baudrate(9_600.bps())  .parity_odd() .stopbits(StopBits::STOP1),
        clocks,
        &mut rcc.apb1,
        );
    #[cfg(feature = "stm32f1xx")]
    let tx1 = txrx1.split().0.with_dma(channels.4);  // console
    #[cfg(feature = "stm32f1xx")]
    let rx2 = txrx2.split().1.with_dma(channels.6);  // GPS



    #[cfg(feature = "stm32f3xx")]
    let clocks = rcc.cfgr.freeze(&mut p.FLASH.constrain().acr);
    #[cfg(feature = "stm32f3xx")]
    let mut gpioa = p.GPIOA.split(&mut rcc.apb2);
    //#[cfg(feature = "stm32f3xx")]
    //let mut gpiob = p.GPIOB.split(&mut rcc.apb2);
    #[cfg(feature = "stm32f3xx")]
    let txrx1 = Serial::usart1(
        p.USART1,
        (gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh), 
	 gpioa.pa10),
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
    #[cfg(feature = "stm32f3xx")]
    let tx1 = txrx1.split().0;      // console
    #[cfg(feature = "stm32f3xx")]
    let rx2 = txrx2.split().1;      // GPS



    #[cfg(feature = "stm32f4xx")]
    let clocks =  rcc.cfgr.freeze();
    #[cfg(feature = "stm32f4xx")]
    let mut gpioa = p.GPIOA.split();
    #[cfg(feature = "stm32f4xx")]
    let mut gpiob = p.GPIOB.split();
    #[cfg(feature = "stm32f4xx")]
    let txrx1 = Serial::usart1(
        p.USART1,
        (gpioa.pa9.into_alternate_af7(), 
	 gpioa.pa10.into_alternate_af7()),
        Config::default() .baudrate(9600.bps()) .stopbits(StopBits::STOP1),
        clocks,
        );

    #[cfg(feature = "stm32f4xx")]
    let txrx2 = Serial::usart2(
        p.USART2,
        ( gpioa.pa2.into_alternate_af7(), 
	  gpioa.pa3.into_alternate_af7()),  // (tx, rx)
        Config::default() .baudrate(9_600.bps()),  //  .parity_odd() .stopbits(StopBits::STOP1),
        clocks,
        ).unwrap();
    #[cfg(feature = "stm32f4xx")]
    let tx1 = txrx1.split().0;      // console
    #[cfg(feature = "stm32f4xx")]
    let rx2 = txrx2.split().1;      // GPS



    #[cfg(feature = "stm32l1xx")]
    let clocks =  rcc.cfgr.freeze();
    #[cfg(feature = "stm32l1xx")]
    let mut gpioa = p.GPIOA.split();
    #[cfg(feature = "stm32l1xx")]
    let mut gpiob = p.GPIOB.split();
    #[cfg(feature = "stm32l1xx")]
    let txrx1 = Serial::usart1(
        p.USART1,
        (gpioa.pa9.into_alternate_af7(),
	 gpioa.pa10.into_alternate_af7()),
        Config::default() .baudrate(9600.bps()) .stopbits(StopBits::STOP1),
        clocks,
        );

    #[cfg(feature = "stm32l1xx")]
    let txrx2 = Serial::usart2(
        p.USART2,
        ( gpioa.pa2.into_alternate_af7(), 
	  gpioa.pa3.into_alternate_af7()),  // (tx, rx)
        Config::default() .baudrate(9_600.bps()),  //  .parity_odd() .stopbits(StopBits::STOP1),
        clocks,
        ).unwrap();
    #[cfg(feature = "stm32l1xx")]
    let tx1 = txrx1.split().0;      // console
    #[cfg(feature = "stm32l1xx")]
    let rx2 = txrx2.split().1;      // GPS



//    pub fn to_str_lossy( x:&[u8] ) -> &str {
//       for byte in  x {
//          match core::str::from_utf8(byte) {
//          Ok(str)     => &str,
//          Err(error) => '.'asUtf8,
//          }
//       }


    // SEE  https://github.com/stm32-rs/stm32f1xx-hal/blob/v0.5.3/examples/adc-dma-rx.rs
    //USES U16

    let (_, tx1) = tx1.write(b"\r\nconsole connect check.\r\n").wait(); 
   
    // read gps on usart2
    hprintln!("about to read GPS").unwrap();
    //  OFTEN STALL AFTER PRINTING THIS LINE, BUT NOT ALWAYS
    //  PROBABLY wait() ??
    
    //Vec<u8, consts::U32> = Vec::new();
    //singleton!(: [u8; 32] = [0; 32]).unwrap();  //trait core::array::LengthAtMost32

    // (buf, rx)  tuple for RxDma VS read() a single u8
    let mut br = (singleton!(: [u8; 32] = [0; 32]).unwrap(), rx2);
    br = br.1.read(br.0).wait();                

    // (buf, tx)  tuple for TxDma VS write() a single u8. Why is buffer is needed?
    let mut bt = (singleton!(: [u8; 32] = [0; 32]).unwrap(), tx1); 

    //let str2 = to_str(buf2);
    //asm::bkpt();

    hprintln!("buf is  {:?}", br.0).unwrap(); 
    //hprintln!("to_str(buf2) is {:?}", to_str(bbr.0)).unwrap();  

    //hprintln!("about to write  buf2 to console").unwrap();
    //hprintln!("sending console {:?}", buf2).unwrap(); //would not need :? if these were not [u8; 8]

    //let bufx = buftx1.1.write(bufrx2.0); 
    //let bufx = buftx1.1.TxDma(bufrx2.0);  // wait(); //TxDma VS read  !!!!!!!!!!!!!
    //hprintln!("sent console {:?}", bufx).unwrap(); //would not need :? if these were not [u8; 8]


    // convert buf from bytes to characters ??
    //let rxstr = buf.to_owned();                
    //let rxstr = buf.clone();                
    //let rxstr = buf.copy;                
    //let rxstr = to_str(buf);                
    //let rxstr = ascii::escape_default(buf);  
    //let rxstr = ascii::from(buf);  

    //asm::bkpt();

    hprintln!("going into write/read loop ^C to exit ...").unwrap();

    // NB: bt uses br's buffer and br uses bt's buffer,
    //   whereas in the first read above br used it's own buffer.
    loop {
       //hprintln!(".").unwrap();
       bt = bt.1.write(br.0).wait(); 
       //hprintln!("-").unwrap();
       br = br.1.read(bt.0).wait();
       }

    // PUT A TEST HERE THAT WILL SHOW FAILURE. ASSERT SEEMS TO PANIC HALT SO ...
}
