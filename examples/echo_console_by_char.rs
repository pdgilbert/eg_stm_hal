//  Using serial1_setup returning serial. Very messy return type and needs to use 
//  even more chip secific parts of crate.
//  compiles for stm32f1xx and stm32f4xx, not for stm32f3xx nor stm32l1xx
//
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

//use embedded_hal::serial;

#[cfg(feature = "stm32f1xx")]  //  eg blue pill stm32f103
use {stm32f1,    stm32f1xx_hal::{prelude::*,   pac::Peripherals, serial::{Config, Serial }} }; 

#[cfg(feature = "stm32f3xx")]  //  eg Discovery-stm32f303
use {stm32f3,  stm32f3xx_hal::{prelude::*, stm32::Peripherals, serial::{ Serial }}};

#[cfg(feature = "stm32f4xx")] // eg Nucleo-64  stm32f411
use {stm32f4, stm32f4xx_hal::{prelude::*,  pac::Peripherals, serial::{config::Config, Serial }}};

#[cfg(feature = "stm32l1xx") ] // eg  Discovery kit stm32l100 and Heltec lora_node STM32L151CCU6
use {stm32l1, stm32l1xx_hal::{prelude::*, stm32::Peripherals, serial::{Config, Serial }}};


#[entry]
fn main() -> ! {

    #[cfg(feature  = "stm32f1xx")]
    type SerialType = stm32f1xx_hal::serial::Serial<stm32f1::stm32f103::USART1,
  (stm32f1xx_hal::gpio::gpioa::PA9<stm32f1xx_hal::gpio::Alternate<stm32f1xx_hal::gpio::PushPull>>,
   stm32f1xx_hal::gpio::gpioa::PA10<stm32f1xx_hal::gpio::Input<stm32f1xx_hal::gpio::Floating>>)>;

    #[cfg(feature = "stm32f1xx")]
    fn serial1_setup() ->  SerialType  {
        let cnfg = Config::default() .baudrate(9600.bps());
	let p = Peripherals::take().unwrap();
    	let mut rcc = p.RCC.constrain();  
	let clocks = rcc.cfgr.freeze(&mut p.FLASH.constrain().acr); 
        //let mut afio = p.AFIO.constrain(&mut rcc.apb2);
    	let mut gpioa = p.GPIOA.split(&mut rcc.apb2);
    	// next consumes (moves) all arguments but clocks and  &mut rcc.apb2
	// but if afio is set above and used in next then it is not consumed.
	let s = Serial::usart1(
    	    p.USART1,
    	    (gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh),     //rx pa9, 
	     gpioa.pa10),					     //tx pa10
    	    &mut p.AFIO.constrain(&mut rcc.apb2).mapr,
    	    //&mut afio.mapr,
    	    cnfg,             //.stopbits(StopBits::STOP1
    	    clocks,
    	    &mut rcc.apb2,
    	    );
	//let z = p.USART1;   //was moved
	//let z = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh); //was moved
	//let z =gpioa.pa10;  //was moved
	//let z = p.AFIO.constrain(&mut rcc.apb2).mapr; //was moved
	//let z = afio.mapr; // NOT moved
	//let z = cnfg;      //was moved
	//let z = clocks;    // NOT moved
	//let z = rcc.apb2;  // NOT moved
	drop(clocks);
	drop(rcc.apb2);
	s
	}


    #[cfg(feature =  "stm32f3xx")]
    type SerialType = stm32f3xx_hal::serial::Serial<stm32f3::stm32f303::USART1,
             (stm32f3xx_hal::gpio::gpioa::PA9<stm32f3xx_hal::gpio::AF7>,
              stm32f3xx_hal::gpio::gpioa::PA10<stm32f3xx_hal::gpio::AF7>)>;

    #[cfg(feature = "stm32f3xx")]
    fn serial1_setup() -> SerialType {
    	let cnfg = 9600.bps();
        let p = Peripherals::take().unwrap();
    	let mut rcc = p.RCC.constrain();
    	let clocks = rcc.cfgr.freeze(&mut p.FLASH.constrain().acr);
    	let mut gpioa = p.GPIOA.split(&mut rcc.ahb); 
    	Serial::usart1(
    	    p.USART1,
    	    (gpioa.pa9.into_af7(&mut gpioa.moder, &mut gpioa.afrh),   //rx pa9
	     gpioa.pa10.into_af7(&mut gpioa.moder, &mut gpioa.afrh)), //tx pa10
    	    cnfg,
    	    clocks,
    	    &mut rcc.apb2,
    	    )
    	}



    #[cfg(feature  = "stm32f4xx")]
    type SerialType = stm32f4xx_hal::serial::Serial<stm32f4::stm32f411::USART1,
    (stm32f4xx_hal::gpio::gpioa::PA9<stm32f4xx_hal::gpio::Alternate<stm32f4xx_hal::gpio::AF7>>,
     stm32f4xx_hal::gpio::gpioa::PA10<stm32f4xx_hal::gpio::Alternate<stm32f4xx_hal::gpio::AF7>>)>;

    #[cfg(feature = "stm32f4xx")]
    fn serial1_setup() -> SerialType {
        let cnfg = Config::default() .baudrate(9600.bps());
        let p = Peripherals::take().unwrap();
    	let rcc = p.RCC.constrain();
    	let clocks = rcc.cfgr.freeze();
    	let gpioa = p.GPIOA.split();
    	p.USART1.cr1.modify(|_,w| w.rxneie().set_bit());  //need RX interrupt? 
    	Serial::usart1(
    	    p.USART1,
    	    (gpioa.pa9.into_alternate_af7(),			      //rx pa9
	     gpioa.pa10.into_alternate_af7()),  		      //tx pa10
    	    cnfg,
    	    clocks,
    	    ).unwrap()
	}



    #[cfg(feature  = "stm32l1xx")]
    type SerialType = stm32l1xx_hal::serial::Serial<stm32l1::stm32l151::USART1>;

    #[cfg(feature = "stm32l1xx")]
    fn serial1_setup() -> SerialType {
        let cnfg = Config::default() .baudrate(9600.bps());
        let p = Peripherals::take().unwrap();
    	let rcc = p.RCC.constrain();
    	let clocks = rcc.cfgr.freeze();
    	let gpioa = p.GPIOA.split();
    	let cnfg = Config::default() .baudrate(9600.bps());
    	p.USART1.cr1.modify(|_,w| w.rxneie().set_bit());  //need RX interrupt? 
    	Serial::usart1(
    	    p.USART1,
    	    (gpioa.pa9.into_alternate_af7(),			      //rx pa9
	     gpioa.pa10.into_alternate_af7()),  		      //tx pa10
    	    cnfg,
    	    clocks,
    	    ).unwrap()
    	}

    // end hal specific conditional setup

    let mut txrx1 = serial1_setup(); 
    //let (mut tx1, mut rx1) =txrx1.split(); // Split into  tx and rx


    hprintln!("testwrite to console ...").unwrap();

    for byte in b"\r\nconsole connect check.\r\n" { block!(txrx1.write(*byte)).ok(); }

    hprintln!("test read and write by char. Please type into the console ...").unwrap();
    //writeln!(tx1, "\r\nPlease type (slowly) into the console below:\r\n").unwrap();
    for byte in b"\r\nType (slowly) below:\r\n" { block!(txrx1.write(*byte)).ok(); }

    loop { // Read a byte and write
       let received = block!(txrx1.read()).unwrap();
       block!(txrx1.write(received)).ok();
       hprintln!("{}", from_utf8(&[received]).unwrap()).unwrap();
     }
}
