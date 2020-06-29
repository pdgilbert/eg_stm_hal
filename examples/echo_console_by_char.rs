//! Echo console input back to console + semihost output, char by char
//!
//! Connect the Tx pin pa9  to the Rx pin of usb-ttl converter
//! Connect the Rx pin pa10 to the Tx pin of usb-ttl converter
//! Set up the serial console (e.g. minicom) with the same settings used here.
//! (Using 9600bps, could be higher but needs serial console to be the same.)

#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[cfg(debug_assertions)]
extern crate panic_semihosting;

#[cfg(not(debug_assertions))]
extern crate panic_halt;

use cortex_m_rt::entry;
//use core::fmt::Write;  // for writeln
use cortex_m_semihosting::hprintln;
use core::str::from_utf8;
use nb::block;

//use embedded_hal::prelude::*, serial::{Config, Serial } ;

#[cfg(feature = "stm32f1xx")]  //  eg blue pill stm32f103
use stm32f1xx_hal::{prelude::*,  pac::Peripherals, serial::{Config, Serial }, 
       rcc::RccExt, flash::FlashExt, }; 

#[cfg(feature = "stm32f3xx")]  //  eg Discovery-stm32f303
use stm32f3xx_hal::{prelude::*,  stm32::Peripherals, serial::{ Serial}, };

#[cfg(feature = "stm32f4xx")] // eg Nucleo-64  stm32f411
use stm32f4xx_hal::{prelude::*,  pac::Peripherals, serial::{config::Config, Serial }};

#[cfg(feature = "stm32l1xx") ] // eg  Discovery kit stm32l100 and Heltec lora_node STM32L151CCU6
use stm32l1xx_hal::{prelude::*,  stm32::Peripherals, serial::{Config, Serial }};


#[entry]
fn main() -> ! {
 
    //see serial_char.rs and serial_string.rs in examples/ for more USART config notes.

    // 1. Get access to the device specific peripherals from the peripheral access crate
    // 2. Take ownership of raw rcc and flash devices and convert to HAL structs
    // 3. Freeze  all system clocks  and store the frozen frequencies in `clocks`
    // 4. Prepare the alternate function I/O registers
    // 5. Prepare the GPIO peripheral
    // 6. Set up the usart device. Take ownership over the USART register and tx/rx pins.
    //    The rest of the registers are used to enable and configure the device.

    let p = Peripherals::take().unwrap();

    #[cfg(feature = "stm32f1xx")]
    let mut rcc = p.RCC.constrain();
    #[cfg(feature = "stm32f3xx")]
    let mut rcc = p.RCC.constrain();
    #[cfg(feature = "stm32f4xx")]
    let rcc = p.RCC.constrain();
    #[cfg(feature = "stm32l1xx")]
    let rcc = p.RCC.constrain();


    #[cfg(feature = "stm32f1xx")]
    let clocks = rcc.cfgr.freeze(&mut p.FLASH.constrain().acr); 
    #[cfg(feature = "stm32f3xx")]
    let clocks = rcc.cfgr.freeze(&mut p.FLASH.constrain().acr);
    #[cfg(feature = "stm32f4xx")]
    let clocks = rcc.cfgr.freeze();
    #[cfg(feature = "stm32l1xx")]
    let clocks = rcc.cfgr.freeze();


    #[cfg(feature = "stm32f1xx")]
    let mut gpioa = p.GPIOA.split(&mut rcc.apb2);   // why an argument and why mutable?
    #[cfg(feature = "stm32f3xx")]
    let mut gpioa = p.GPIOA.split(&mut rcc.ahb); 
    #[cfg(feature = "stm32f4xx")]
    let gpioa = p.GPIOA.split();
    #[cfg(feature = "stm32l1xx")]
    let gpioa = p.GPIOA.split();


    #[cfg(feature = "stm32f1xx")]
    let pin_rx1 = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);     //pa9
    #[cfg(feature = "stm32f3xx")]
    let pin_rx1 = gpioa.pa9.into_af7(&mut gpioa.moder, &mut gpioa.afrh);  //pa9
    #[cfg(feature = "stm32f4xx")]
    let pin_rx1 = gpioa.pa9.into_alternate_af7();                         //pa9
    #[cfg(feature = "stm32l1xx")]
    let pin_rx1 = gpioa.pa9.into_alternate_af7();                         //pa9


    #[cfg(feature = "stm32f1xx")]
    let pin_tx1 = gpioa.pa10;                                             //pa10
    #[cfg(feature = "stm32f3xx")]
    let pin_tx1 = gpioa.pa10.into_af7(&mut gpioa.moder, &mut gpioa.afrh); //pa10
    #[cfg(feature = "stm32f4xx")]
    let pin_tx1 = gpioa.pa10.into_alternate_af7();                        //pa10
    #[cfg(feature = "stm32l1xx")]
    let pin_tx1 = gpioa.pa10.into_alternate_af7();                        //pa10


    #[cfg(feature = "stm32f1xx")]
    let cnfg =  Config::default() .baudrate(9600.bps());  //.stopbits(StopBits::STOP1),
    #[cfg(feature = "stm32f3xx")]
    let cnfg = 9600.bps();
    #[cfg(feature = "stm32f4xx")]
    let cnfg = Config::default() .baudrate(9600.bps());
    #[cfg(feature = "stm32l1xx")]
    let cnfg = Config::default() .baudrate(9600.bps());


    #[cfg(feature = "stm32f4xx")]
    p.USART1.cr1.modify(|_,w| w.rxneie().set_bit());  //need RX interrupt? 
    #[cfg(feature = "stm32l1xx")]
    p.USART1.cr1.modify(|_,w| w.rxneie().set_bit());  //need RX interrupt? 
   

    let txrx1 = Serial::usart1(
        p.USART1,
        (pin_rx1, pin_tx1),
        #[cfg(feature = "stm32f1xx")]
        &mut p.AFIO.constrain(&mut rcc.apb2).mapr,
        cnfg,
        clocks,
        #[cfg(any(feature = "stm32f1xx", feature = "stm32f3xx"))]
        &mut rcc.apb2,
    );

    #[cfg(any(feature = "stm32f4xx", feature = "stm32l1xx"))]
    let txrx1 = txrx1.unwrap();

    // end hal specific conditional setup

    // Split the serial txrx1 struct into a receiving and a transmitting part
    let (mut tx1, mut rx1) =txrx1.split();


    hprintln!("testwrite to console ...").unwrap();
    //let number = 42;
    // write! and writeln! cause method not found in `stm32f3xx_hal but work in other HALs
    //writeln!(tx1, "\r\nHello {}. Converted number 42 for formatted write.\r\n", number).unwrap();

    for byte in b"\r\nconsole connect check.\r\n" { block!(tx1.write(*byte)).ok(); }

    hprintln!("test read and write by char. Please type into the console ...").unwrap();
    //writeln!(tx1, "\r\nPlease type (slowly) into the console below:\r\n").unwrap();
    for byte in b"\r\nType (slowly) below:\r\n" { block!(tx1.write(*byte)).ok(); }

    loop { // Read a byte and write
       let received = block!(rx1.read()).unwrap();
       block!(tx1.write(received)).ok();
       hprintln!("{}", from_utf8(&[received]).unwrap()).unwrap();
     }
}
