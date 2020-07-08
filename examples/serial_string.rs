//! Serial interface test writing a buffer of bytes between two usarts and
//! echo to the computer console connected by usb-ttl dongle on another usart.
//! This example differs from example serial_char in that it attempts to send 
//! a whole buffer rather than a single byte.
//! See example serial_char regarding the usart details, pins connections,
//! and additional comments.

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
use cortex_m_semihosting::hprintln;
//use core::str::from_utf8;

#[cfg(not(feature = "stm32f1xx"))]
use nb::block;

use eg_stm_hal::to_str;

#[cfg(feature = "stm32f1xx")]  //  eg blue pill stm32f103
use stm32f1xx_hal::{prelude::*,   pac::Peripherals, serial::{Config, Serial, StopBits, }, };

#[cfg(feature = "stm32f3xx")]  //  eg Discovery-stm32f303
use stm32f3xx_hal::{prelude::*, stm32::Peripherals, serial::{Serial}, };
//use stm32f3xx_hal::{prelude::*, stm32::Peripherals, serial::{Config, Serial, StopBits}, };

#[cfg(feature = "stm32f4xx")] // eg Nucleo-64  stm32f411
use stm32f4xx_hal::{prelude::*, pac::Peripherals, serial::{config::Config, Serial}};

#[cfg(feature = "stm32l1xx") ] // eg  Discovery kit stm32l100 and Heltec lora_node STM32L151CCU6
use stm32l1xx_hal::{prelude::*, stm32::Peripherals, serial::{Config, Serial}};


#[entry]
fn main() -> ! {

    hprintln!("initializing ...").unwrap();

    hprintln!("testing console output ").unwrap();

    // BEGIN COMMON USART SETUP

    let p = Peripherals::take().unwrap();
    // stm32f4xx warns that mut is not needed in next, but other hals require it
    let mut rcc = p.RCC.constrain();


    // stm32f1xx
    //dma buffer works on stm32f1xx_hal but not others

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
        Config::default() .baudrate(9_600.bps()) .parity_odd() .stopbits(StopBits::STOP1),
        clocks,
        &mut rcc.apb2,
    );
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
    let mut gpiob = p.GPIOB.split(&mut rcc.apb2);
    #[cfg(feature = "stm32f1xx")]
    let txrx3 = Serial::usart3(
        p.USART3,
        ( gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh),  
	  gpiob.pb11),  // (tx, rx)
        &mut afio.mapr,
        Config::default() .baudrate(9_600.bps())  .parity_odd() .stopbits(StopBits::STOP1),
        clocks,
        &mut rcc.apb1,    // WHAT IS  rcc.apb1/2 ?
    );
    // Split the serial struct into a receiving and a transmitting part
    #[cfg(feature = "stm32f1xx")]
    let channels = p.DMA1.split(&mut rcc.ahb);
    #[cfg(feature = "stm32f1xx")]
    let tx1 = txrx1.split().0.with_dma(channels.4);      // console
    // ok let (_, tx1) = tx1.write(b"console connect check.").wait(); 
    // No (_, tx1) = tx1.write(b"console connect check.").wait(); 
    #[cfg(feature = "stm32f1xx")]
    let tx1 = tx1.write(b"console connect check.").wait().1; 
    //for byte in b"\r\nconsole connect check.\r\n" { block!(tx1.write(*byte)).ok(); }

    // re dma read see  https://github.com/stm32-rs/stm32f1xx-hal/blob/v0.5.3/examples/adc-dma-rx.rs

    #[cfg(feature = "stm32f1xx")]
    let ( tx2, rx2)  = txrx2.split();
    #[cfg(feature = "stm32f1xx")]
    let mut tx2  = tx2.with_dma(channels.7);
    #[cfg(feature = "stm32f1xx")]
    let _rx2  = rx2.with_dma(channels.6);

    #[cfg(feature = "stm32f1xx")]
    let ( tx3, rx3)  = txrx3.split();
    #[cfg(feature = "stm32f1xx")]
    let _tx3  = tx3.with_dma(channels.2);
    #[cfg(feature = "stm32f1xx")]
    let rx3  = rx3.with_dma(channels.3);


    //#[cfg(any(feature = "stm32f1xx", feature = "stm32l1xx"))]
    //let channels = p.DMA1.split(&mut rcc.ahb);
    //let mut tx = txrx1.split().0.with_dma(channels.4);     //works on stm32f1xx_hal but not others
    //let (_, tx) = tx.write(b"The quick brown fox").wait(); //works on stm32f1xx_hal but not others

    // stm32f3xx

    // stm32f303vct  alternate funtion modes see  
    // https://www.rlocman.ru/i/File/dat/STMicroelectronics/Microcontrollers_MCU/STM32F303VCT6.pdf p42
    // AF7 on PA9  is usart1_Tx, on PA10 is usart1_Rx,
    // AF7 on PA2  is usart2_Tx, on PA3  is usart2_Rx,
    // AF7 on PB10 is usart3_Tx, on PB11 is usart3_Rx,

    #[cfg(feature = "stm32f3xx")]
    let clocks = rcc.cfgr.freeze(&mut p.FLASH.constrain().acr);
    #[cfg(feature = "stm32f3xx")]
    let mut gpioa = p.GPIOA.split(&mut rcc.ahb); //Why does this need arg, there is only one possibility?

    #[cfg(feature = "stm32f3xx")]
    let txrx1     = Serial::usart1(
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
        115_200.bps(),
        clocks,
        &mut rcc.apb1,
    );

    #[cfg(feature = "stm32f3xx")]
    let mut gpiob = p.GPIOB.split(&mut rcc.ahb);
    #[cfg(feature = "stm32f3xx")]
    let txrx3 = Serial::usart3(
        p.USART3,
        (gpiob.pb10.into_af7(&mut gpiob.moder, &mut gpiob.afrh),
	 gpiob.pb11.into_af7(&mut gpiob.moder, &mut gpiob.afrh)), 
        115_200.bps(),
        clocks,
        &mut rcc.apb1,    // WHAT IS  rcc.apb1/2 ?
    );
    // Split the serial struct into a receiving and a transmitting part
    #[cfg(feature = "stm32f3xx")]
    let mut tx1             = txrx1.split().0;    // console
    #[cfg(feature = "stm32f3xx")]
    let (mut tx2, mut rx2)  = txrx2.split();
    #[cfg(feature = "stm32f3xx")]
    let (mut tx3, mut rx3)  = txrx3.split();   

    #[cfg(feature = "stm32f3xx")]
    for byte in b"\r\nconsole connect check.\r\n" { block!(tx1.write(*byte)).ok(); }



    // stm32f411re 
    // stm32f411re implements only usarts 1, 2, and 6. These can be configured on different pins.
    // alternate funtion modes see https://www.st.com/resource/en/datasheet/stm32f411re.pdf  p47.
    // AF7 on PA9  is usart1_Tx, on PA10 is usart1_Rx,
    // AF7 on PA2  is usart2_Tx, on PA3  is usart2_Rx,
    // AF8 on PA11 is usart6_Tx, on PA12 is usart6_Rx

    #[cfg(feature = "stm32f4xx")]
    let clocks = rcc.cfgr.freeze();
    #[cfg(feature = "stm32f4xx")]
    let gpioa = p.GPIOA.split();
    #[cfg(feature = "stm32f4xx")]
    p.USART1.cr1.modify(|_,w| w.rxneie().set_bit());  //need RX interrupt? 
    //let (tx,rx) = 

    #[cfg(feature = "stm32f4xx")]
    let txrx1 =  Serial::usart1(
        p.USART1,
    	(gpioa.pa9.into_alternate_af7(), 
	 gpioa.pa10.into_alternate_af7()), 
    	Config::default() .baudrate(9600.bps()),
    	clocks
    ).unwrap(); 

    #[cfg(feature = "stm32f4xx")]
    let txrx2 = Serial::usart2(
        p.USART2,
        ( gpioa.pa2.into_alternate_af7(), 
	  gpioa.pa3.into_alternate_af7()),  // (tx, rx)
        Config::default() .baudrate(115_200.bps()),  //  .parity_odd() .stopbits(StopBits::STOP1),
        clocks,
    ).unwrap();

    #[cfg(feature = "stm32f4xx")]
    let txrx3 = Serial::usart6(      // (tx, rx)  NOTE PINS and USART6 !!!
        p.USART6,
        ( gpioa.pa11.into_alternate_af8(),  
	  gpioa.pa12.into_alternate_af8()),
        Config::default() .baudrate(115_200.bps()) ,
        clocks,
    ).unwrap();
    // Split the serial struct into a receiving and a transmitting part
    #[cfg(feature = "stm32f4xx")]
    let mut tx1             = txrx1.split().0;    // console
    #[cfg(feature = "stm32f4xx")]
    let (mut tx2, mut rx2)  = txrx2.split();
    #[cfg(feature = "stm32f4xx")]
    let (mut tx3, mut rx3)  = txrx3.split();   

    #[cfg(feature = "stm32f4xx")]
    for byte in b"\r\nconsole connect check.\r\n" { block!(tx1.write(*byte)).ok(); }



    #[cfg(feature = "stm32l1xx")]
    let clocks = rcc.cfgr.freeze();
    #[cfg(feature = "stm32l1xx")]
    let gpioa = p.GPIOA.split();
    #[cfg(feature = "stm32l1xx")]
    p.USART1.cr1.modify(|_,w| w.rxneie().set_bit());  //need RX interrupt? 
    //let (tx,rx) = 

    #[cfg(feature = "stm32l1xx")]
    let txrx1 =  Serial::usart1(
        p.USART1,
    	(gpioa.pa9.into_alternate_af7(), 
	 gpioa.pa10.into_alternate_af7()), 
    	Config::default() .baudrate(9600.bps()),
    	clocks
    ).unwrap(); 

    #[cfg(feature = "stm32l1xx")]
    let txrx2 = Serial::usart2(
        p.USART2,
        ( gpioa.pa2.into_alternate_af7(), 
	  gpioa.pa3.into_alternate_af7()),  // (tx, rx)
        Config::default() .baudrate(115_200.bps()),  //  .parity_odd() .stopbits(StopBits::STOP1),
        clocks,
    ).unwrap();

    #[cfg(feature = "stm32l1xx")]
    let txrx3 = Serial::usart6(      // (tx, rx)  NOTE PINS and USART6 !!!
        p.USART6,
        ( gpioa.pa11.into_alternate_af8(), 
	  gpioa.pa12.into_alternate_af8()),
        Config::default() .baudrate(115_200.bps()) ,
        clocks,
    ).unwrap();
    // Split the serial struct into a receiving and a transmitting part
    #[cfg(feature = "stm32l1xx")]
    let mut tx1             = txrx1.split().0;    // console
    #[cfg(feature = "stm32l1xx")]
    let (mut tx2, mut rx2)  = txrx2.split();
    #[cfg(feature = "stm32l1xx")]
    let (mut tx3, mut rx3)  = txrx3.split();   

    #[cfg(feature = "stm32l1xx")]
    for byte in b"\r\nconsole connect check.\r\n" { block!(tx1.write(*byte)).ok(); }

    // END COMMON USART SETUP

//    fn putTx1(string: &[u8] ) -> usize {
//	 tx1.write(string).ok() ;
//	 string.len()
//	 }
//    
//    #[not(cfg(feature = "stm32f1xx"))]
//    fn putTx1(string: &[u8] ) -> usize {
//	 for byte in  string { block!(tx1.write(*byte)).ok() };
//	 string.len()
//	 }
//     iterator fails if string is too long
//    for byte in  b"tx2 to rx3 test with X\r\n" { block!(tx1.write(*byte)).unwrap(); }

//    type Tx = stm32f1xx_hal::serial::Tx<USART1>;
//
//    pub fn putTx(tx: Tx,   string: &[u8] ) -> bool {
//       for byte in  string {
//	   block!(tx.write(*byte)).unwrap() 
//	   //match block!(tx.write(*byte)).unwrap() {
//	   //Ok(str)	=> &str,
//	   //Err(error) => '.'asUtf8,
//	   //}
//	   }
//       true
//       }
//    putTx(tx1,  send);

//    let mut received = [0u8; 64];
//    for i in  range(0..len(received))  {
//     received[0] = block!(rx3.read()).unwrap();  
//       i += 1;
//    }

    hprintln!("testing  tx2 to rx3").unwrap();
    hprintln!("   sending on tx2 ...").unwrap();

    let send =  b"The quick brown fox";
    
    // Write and wait until the write is successful
    // For .write() discard the buffer returned as a new buffer is supplied to the the next write.
    // This can be done either with this
    //let (_, mut tx2) = tx2.write(send).wait();
    // or this
    tx2 = tx2.write(send).wait().1;

    //putTx1(send);
    //for byte in send.iter() { block!(tx1.write(*byte)).unwrap(); }   // using iter

    // For .read() the buffer is maintained as part of the tuple, but buf and rx need 
    // to be separated when it is used.
    // (buf, rx)  tuple for RxDma VS read() a single u8
    let mut br3 = (singleton!(: [u8; 32] = [0; 32]).unwrap(),  rx3);

    hprintln!("   receiving on rx3 ...").unwrap();

    br3 = br3.1.read(br3.0).wait();
  
    hprintln!("  checking received = send,  {} = {} byte", to_str(br3.0), to_str(send)).unwrap();


    hprintln!("testing  tx2 to rx3 again").unwrap();
    let send = b" jumps\n";
    //tx2 =  don't reassign last time it is used will prevent a warning.
    tx2.write(send).wait().1;
    br3 = br3.1.read(br3.0).wait();

    hprintln!("  checking received = send,  {} = {} byte", to_str(br3.0), to_str(send)).unwrap();

    hprintln!("  sending  received to console...").unwrap();

    // this cannot be above to_str(br3.0) because buffer does not implement copy trait
    // _tx1 the last time it is used prevents warning.
    let (_, _tx1) = tx1.write(br3.0).wait();  // and send to console

    // sent should be the same as received
    //assert_eq!(received, send, "testing received = send,  {} = {}", received, send);

    // PUT A TEST HERE THAT WILL SHOW FAILURE. ASSERT SEEMS TO PANIC HALT SO ...

    // Trigger a breakpoint to inspect the values
    //asm::bkpt();

    hprintln!("entering empty loop. ^C to exit.").unwrap();
    loop {}
}
