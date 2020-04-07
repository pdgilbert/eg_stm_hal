//! Serial interface test writing a string between two usarts and
//! echo to the computer consol connected by usb-ttl dongle on another usart.
//! This example differs from example serial_char only in attempting to send a string.
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
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use core::str::from_utf8;
use nb::block;

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

    hprintln!("initializing ...").unwrap();

    // BEGIN COMMON USART SETUP

    let p = Peripherals::take().unwrap();
    // stm32f4xx warns that mut is not needed in next, but other hals require it
    let mut rcc = p.RCC.constrain();


    // stm32f1xx

    #[cfg(any(feature = "stm32f1xx", feature = "stm32l1xx"))]
    let clocks = rcc.cfgr.freeze(&mut p.FLASH.constrain().acr);

    #[cfg(any(feature = "stm32f1xx", feature = "stm32l1xx"))]
    let mut afio = p.AFIO.constrain(&mut rcc.apb2);

    #[cfg(any(feature = "stm32f1xx", feature = "stm32l1xx"))]
    let mut gpioa = p.GPIOA.split(&mut rcc.apb2);

    #[cfg(any(feature = "stm32f1xx", feature = "stm32l1xx"))]
    let txrx1 = Serial::usart1(
        p.USART1,
        (gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh),   gpioa.pa10),
        &mut afio.mapr,
        Config::default() .baudrate(9_600.bps()) .parity_odd() .stopbits(StopBits::STOP1),
        clocks,
        &mut rcc.apb2,
    );

    //#[cfg(any(feature = "stm32f1xx", feature = "stm32l1xx"))]
    //let channels = p.DMA1.split(&mut rcc.ahb);
    //let mut tx = txrx1.split().0.with_dma(channels.4);     //works on stm32f1xx_hal but not others
    //let (_, tx) = tx.write(b"The quick brown fox").wait(); //works on stm32f1xx_hal but not others

    #[cfg(any(feature = "stm32f1xx", feature = "stm32l1xx"))]
    let txrx2 = Serial::usart2(
        p.USART2,
        (gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl),   gpioa.pa3),  // (tx, rx)
        &mut afio.mapr,
        Config::default() .baudrate(9_600.bps())  .parity_odd() .stopbits(StopBits::STOP1),
        clocks,
        &mut rcc.apb1,
    );

    #[cfg(any(feature = "stm32f1xx", feature = "stm32l1xx"))]
    let mut gpiob = p.GPIOB.split(&mut rcc.apb2);
    #[cfg(any(feature = "stm32f1xx", feature = "stm32l1xx"))]
    let txrx3 = Serial::usart3(
        p.USART3,
        ( gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh),   gpiob.pb11),  // (tx, rx)
        &mut afio.mapr,
        Config::default() .baudrate(9_600.bps())  .parity_odd() .stopbits(StopBits::STOP1),
        clocks,
        &mut rcc.apb1,    // WHAT IS  rcc.apb1/2 ?
    );


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
        (gpioa.pa9.into_af7(&mut gpioa.moder, &mut gpioa.afrh), gpioa.pa10.into_af7(&mut gpioa.moder, &mut gpioa.afrh)),
        9600.bps(),
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

    #[cfg(feature = "stm32f3xx")]
    let mut gpiob = p.GPIOB.split(&mut rcc.ahb);
    #[cfg(feature = "stm32f3xx")]
    let txrx3 = Serial::usart3(
        p.USART3,
        (gpiob.pb10.into_af7(&mut gpiob.moder, &mut gpiob.afrh), gpiob.pb11.into_af7(&mut gpiob.moder, &mut gpiob.afrh)), 
        115_200.bps(),
        clocks,
        &mut rcc.apb1,    // WHAT IS  rcc.apb1/2 ?
    );



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
    	(gpioa.pa9.into_alternate_af7(),  gpioa.pa10.into_alternate_af7()), 
    	Config::default() .baudrate(9600.bps()),
    	clocks
    ).unwrap(); 

   #[cfg(feature = "stm32f4xx")]
    let txrx2 = Serial::usart2(
        p.USART2,
        ( gpioa.pa2.into_alternate_af7(),   gpioa.pa3.into_alternate_af7()),  // (tx, rx)
        Config::default() .baudrate(115_200.bps()),  //  .parity_odd() .stopbits(StopBits::STOP1),
        clocks,
    ).unwrap();

    #[cfg(feature = "stm32f4xx")]
    let txrx3 = Serial::usart6(      // (tx, rx)  NOTE PINS and USART6 !!!
        p.USART6,
        ( gpioa.pa11.into_alternate_af8(),   gpioa.pa12.into_alternate_af8()),
        Config::default() .baudrate(115_200.bps()) ,
        clocks,
    ).unwrap();

    // END COMMON USART SETUP


    // Split the serial struct into a receiving and a transmitting part
    let mut tx1             = txrx1.split().0;  
    let (mut tx2, mut rx2)  = txrx2.split();
    let (mut tx3, mut rx3)  = txrx3.split();   

    hprintln!("testing console output ").unwrap();
 
    let sent =  b"The quick brown fox";
    for byte in sent {
       block!(tx1.write(*byte)).unwrap();
    }
    let s =  b" jumps\n";
    for byte in s.iter() {
       block!(tx1.write(*byte)).unwrap();
    }
    for byte in  b" over the lazy dog.\r\n" {
       block!(tx1.write(*byte)).unwrap();
    }


    hprintln!("testing  tx2 to rx3").unwrap();
    hprintln!("   sending on tx2 ...").unwrap();
    
    // This really needs buffering and separate processes to be done properly!!!

    // Write `The quick brown fox` and wait until the write is successful
    // Read the byte that was just sent. Blocks until the read is complete
    for byte in  sent {
       block!(tx2.write(byte)).ok();
       let received = block!(rx3.read()).unwrap();
    }

    hprintln!("   receiving on rx3 ...").unwrap();

    hprintln!("  checking received = sent,  {} = {} byte", received, sent).unwrap();

    // The sent byte should be the one received
    assert_eq!(received, sent, "testing received = sent,  {} = {}", received, sent);

    // PUT A TEST HERE THAT WILL SHOW FAILURE. ASSERT SEEMS TO PANIC HALT SO ...

    // Now print to semi-host as character rather than byte.
    // Note that sent above was u8 byte (b'X') because tx.write() requires that, but
    //    hprintln!() needs a str and from_utf8() needs a slice, thus [sent].
    
    hprintln!("   tx2 to rx3  characters,  {} = {}", 
        from_utf8(&[received]).unwrap(), from_utf8(&[sent]).unwrap()).unwrap();

    hprintln!("sending received to console on tx1 ...").unwrap();

    for byte in  b"tx2 to rx3 test with X\r\n" {  // iterator fails if string is too long
       block!(tx1.write(*byte)).unwrap();
    }
    //block!(tx1.write(received)).unwrap();
    block!(tx1.write(received)).ok();
    for byte in  b"\r\n" {
       block!(tx1.write(*byte)).unwrap();
    }


    hprintln!("testing  tx3 to rx2").unwrap();
    hprintln!("   sending on tx3 ...").unwrap();

    let sent =b" jumps\n";

    // Write `jumps\n"` and wait until the write is successful
    block!(tx3.write(sent)).ok();

    hprintln!("   receiving on rx2 ...").unwrap();

    // Read the byte that was just sent. Blocks until the read is complete
    let received = block!(rx2.read()).unwrap();

    hprintln!("   checking tx3 to rx2  received = sent,  {} = {} byte", received, sent).unwrap();

    // The sent byte should be the one received
    assert_eq!(received, sent, "testing received = sent,  {} = {}", received, sent);
    
    hprintln!(" tx3 to rx2  characters,  {} = {}", 
        from_utf8(&[received]).unwrap(), from_utf8(&[sent]).unwrap()).unwrap();

    hprintln!("sending received from rx2  to console on tx1 ...").unwrap();

    for byte in  b"tx3 to rx2 test with Y\r\n" {  // iterator fails if string is too long
       block!(tx1.write(*byte)).unwrap();
    }
    //block!(tx1.write(received)).unwrap();
    block!(tx1.write(received)).ok();
    for byte in  b"\r\n" {
       block!(tx1.write(*byte)).unwrap();
    }


    // Trigger a breakpoint to inspect the values
    //asm::bkpt();

    hprintln!("entering empty loop. ^C to exit.").unwrap();
    loop {}
}
