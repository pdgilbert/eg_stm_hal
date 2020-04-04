//! Single character serial interface loopback test on usart2 pins pa2, pa3.
//! 
//! THESE ARE BLUE PILL PIN NUMBERS. CONFIRM PIN NUMBERS OF OTHER BOARDS
//! Short usart2 TX pin pa2 to RX pin pa3.
//! Based on stm32f1xx_hal/example/serial.rs
//! 
//! Connect usart1  to serial-usb converter on computer for console output:
//! usart1 connect the Tx pin pa9  to the Rx pin of a serial-usb converter
//! usart1 connect the Rx pin pa10 to the Tx pin of a serial-usb converter
//! Set up the serial console (e.g. minicom) with the same settings used here.
//! (Using 9600bps, could be higher but needs serial console to be the same.)

// This example contains the most extensive notes. 
// ANY NOTES SHOULD BE EXAPANDED HERE IF THEY APPLY HERE, 
// OTHERWISE PUT THEM IN THE EXAMPLE WHERE THEY APPLY !

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
    // EXPAND NOTES HERE
    //see serial_loopback_char.rs and serial_cross.rs in examples/ for more USART config notes.
    //and examples/echo_by_char.rs for additional comments.

    //  bluepill
    //    USART       (tx,                                             rx)
    // USART1     ( gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh),  gpioa.pa10)
    // USART1 alt ( gpiob.pb6.into_alternate_push_pull(&mut gpiob.crl),  gpiob.pb7)
    // USART2     ( gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl).  gpioa.pa3)
    // USART3     ( gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh), gpiob.pb11)


    // 1. Get access to the device specific peripherals from the peripheral access crate
    // 2. Take ownership of raw rcc and flash devices and convert to HAL structs
    // 3. Freeze  all system clocks  and store the frozen frequencies in `clocks`
    // 4. Prepare the alternate function I/O registers
    // 5. Prepare the GPIO peripheral
    // 6. Set up the usart device. Take ownership over the USART register and tx/rx pins.
    //    The rest of the registers are used to enable and configure the device.

    hprintln!("initializing ...").unwrap();

    let p = Peripherals::take().unwrap();

    #[cfg(any(feature = "stm32f1xx", feature = "stm32l1xx"))]
    let mut rcc = p.RCC.constrain();
    #[cfg(any(feature = "stm32f1xx", feature = "stm32l1xx"))]
    let clocks = rcc.cfgr.freeze(&mut p.FLASH.constrain().acr);
    #[cfg(any(feature = "stm32f1xx", feature = "stm32l1xx"))]
    let mut gpioa = p.GPIOA.split(&mut rcc.apb2);
    #[cfg(any(feature = "stm32f1xx", feature = "stm32l1xx"))]
    let txrx2 = Serial::usart2(
        p.USART2,
        ( gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl),   gpioa.pa3),  // (tx, rx)
        &mut p.AFIO.constrain(&mut rcc.apb2).mapr,
        Config::default() .baudrate(115_200.bps())  .parity_odd() .stopbits(StopBits::STOP1),
        clocks,
        &mut rcc.apb1,
    );


    #[cfg(feature = "stm32f3xx")]
    let mut rcc   = p.RCC.constrain();
    #[cfg(feature = "stm32f3xx")]
    let clocks    = rcc.cfgr.freeze(&mut p.FLASH.constrain().acr);
    #[cfg(feature = "stm32f3xx")]
    let mut gpioa = p.GPIOA.split(&mut rcc.ahb);
    #[cfg(feature = "stm32f3xx")]
    let txrx2     = Serial::usart2(
        p.USART2,
        (gpioa.pa2.into_af7(&mut gpioa.moder, &mut gpioa.afrl), gpioa.pa3.into_af7(&mut gpioa.moder, &mut gpioa.afrl)), //(tx,rx)
        115_200.bps(),
        clocks,
        &mut rcc.apb1,
    );


    #[cfg(feature = "stm32f4xx")]
    let clocks = p.RCC.constrain().cfgr.freeze();
    #[cfg(feature = "stm32f4xx")]
    let gpioa = p.GPIOA.split();
    //#[cfg(feature = "stm32f4xx")]
    //p.USART2.cr1.modify(|_,w| w.rxneie().set_bit());  //need RX interrupt? 
    //let (tx,rx) = 
    // See examples/serail_cross.rs for stm32f411re uart and alternate function notes.
    #[cfg(feature = "stm32f4xx")]
    let txrx2 =  Serial::usart2(
        p.USART2,
    	(gpioa.pa2.into_alternate_af7(),  gpioa.pa3.into_alternate_af7()),
    	Config::default() .baudrate(115_200.bps()),
    	clocks,
    ).unwrap(); 
    


    // let mut gpiob = p.GPIOB.split(&mut rcc.apb2);

    // .baudrate(115_200.bps()  .baudrate(9_600.bps()
    // StopBits::STOP1   StopBits::STOP2

    // Split the serial struct into a receiving and a transmitting part
    let (mut tx, mut rx) = txrx2.split();

    hprintln!("sending ...").unwrap();

    let sent = b'X';

    // Write `X` and wait until the write is successful
    block!(tx.write(sent)).ok();

    hprintln!("receiving ...").unwrap();

    // Read the byte that was just sent. Blocks until the read is complete
    let received = block!(rx.read()).unwrap();

    hprintln!("testing received = sent,  {} = {} byte", received, sent).unwrap();
    // With tx connected to rx, the sent byte should be the one received
    assert_eq!(received, sent, "testing received = sent,  {} = {}", received, sent);

    // PUT A TEST HERE THAT WILL SHOW FAILURE. ASSERT SEEMS TO PANIC HALT SO ...

    // and now print as chararter rather than byte.
    // Note that sent above was u8 byte (b'X') because tx.write() requires that, but
    //    hprintln!() needs a str and from_utf8() needs a slice, thus [sent].
    
    hprintln!("   strings,  {} = {}", 
        from_utf8(&[received]).unwrap(), from_utf8(&[sent]).unwrap()).unwrap();

    // Trigger a breakpoint to inspect the values
    //asm::bkpt();

    loop {}
}
