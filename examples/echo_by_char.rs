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
//use core::fmt::Write;  // for writeln, but not supported by stm32f3xx_hal
use cortex_m_semihosting::hprintln;
use core::str::from_utf8;
use nb::block;

//use embedded_hal::serial;
//use embedded_hal::blocking::serial::{Write, Read};
//use embedded_hal::serial::{Write, Read};

#[cfg(feature = "stm32f1xx")]  //  eg blue pill stm32f103
use stm32f1xx_hal::{prelude::*,   
                    pac::Peripherals, 
                    serial::{Config, Serial, Tx, Rx},  
		    device::USART1 
		    }; 

#[cfg(feature = "stm32f3xx")]  //  eg Discovery-stm32f303
use stm32f3xx_hal::{prelude::*, 
                    stm32::Peripherals,
                    serial::{ Serial, Tx, Rx},
		    stm32::USART1 
		    };

#[cfg(feature = "stm32f4xx")] // eg Nucleo-64  stm32f411
use stm32f4xx_hal::{prelude::*,  
                    pac::Peripherals, 
                    serial::{config::Config, Serial, Tx, Rx},
		    pac::USART1 
		    };

#[cfg(feature = "stm32f7xx")]
use stm32f7xx_hal::{prelude::*,  
                    pac::Peripherals,
                    serial::{Config, Serial, Tx, Rx, Oversampling, },
		    pac::USART1 
		    };

#[cfg(feature = "stm32h7xx")] 
use stm32h7xx_hal::{prelude::*,  
                    pac::Peripherals, 
                    serial::{Tx, Rx},   //Serial, 
		    pac::USART1 
		    };

#[cfg(feature = "stm32l0xx")]
use stm32l0xx_hal::{prelude::*,  
                    pac::Peripherals, 
                    serial::{config::Config, Serial, Tx, Rx},
		    pac::USART1 
		    };

#[cfg(feature = "stm32l1xx") ] // eg  Discovery kit stm32l100 and Heltec lora_node STM32L151CCU6
use stm32l1xx_hal::{prelude::*, 
		    stm32::Peripherals, 
		    serial::{Config, Serial, Tx, Rx},
		    stm32::USART1 
		    };

#[cfg(feature = "stm32l4xx")] 
use stm32l4xx_hal::{prelude::*,  
                    pac::Peripherals, 
                    serial::{Config, Serial, Tx, Rx},
		    pac::USART1 
		    };


#[entry]
fn main() -> ! {

    // A simple abstraction for returning the result of Serial::usart1()  
    //  without  .split() inside setup()  still defeats me. 

    #[cfg(feature = "stm32f1xx")]
    fn setup() ->  (Tx<USART1>, Rx<USART1>)  {
        let p = Peripherals::take().unwrap();
    	let mut rcc = p.RCC.constrain();  
	let clocks = rcc.cfgr.freeze(&mut p.FLASH.constrain().acr); 
        let mut afio = p.AFIO.constrain(&mut rcc.apb2);
    	let mut gpioa = p.GPIOA.split(&mut rcc.apb2);
    	// next consumes (moves) arguments other than clocks,  &mut rcc.apb2 and afio.
	Serial::usart1(
    	    p.USART1,
    	    (gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh),     //tx pa9
	     gpioa.pa10),					     //rx pa10
    	    &mut afio.mapr,
    	    Config::default() .baudrate(9600.bps()),        //.stopbits(StopBits::STOP1
    	    clocks,
    	    &mut rcc.apb2,
    	    ).split()
	}


    #[cfg(feature = "stm32f3xx")]
    	fn setup() -> (Tx<USART1>, Rx<USART1>) {
        let p = Peripherals::take().unwrap();
    	let mut rcc = p.RCC.constrain();
    	let clocks = rcc.cfgr.freeze(&mut p.FLASH.constrain().acr);
    	let mut gpioa = p.GPIOA.split(&mut rcc.ahb); 
    	//let cnfg = 9600.bps();
    	Serial::usart1(
    	    p.USART1,
    	    (gpioa.pa9.into_af7(&mut gpioa.moder, &mut gpioa.afrh),   //tx pa9
	     gpioa.pa10.into_af7(&mut gpioa.moder, &mut gpioa.afrh)), //rx pa10
    	    9600.bps(),
    	    clocks,
    	    &mut rcc.apb2,
    	    ).split()
    	}



    #[cfg(feature = "stm32f4xx")]
    fn setup() -> (Tx<USART1>, Rx<USART1>) {
        let p = Peripherals::take().unwrap();
    	let rcc = p.RCC.constrain();
    	let clocks = rcc.cfgr.freeze();
    	let gpioa = p.GPIOA.split();
    	p.USART1.cr1.modify(|_,w| w.rxneie().set_bit());  //need RX interrupt? 
    	Serial::usart1(
    	    p.USART1,
    	    (gpioa.pa9.into_alternate_af7(),			      //tx pa9
	     gpioa.pa10.into_alternate_af7()),  		      //rx pa10
    	    Config::default() .baudrate(9600.bps()),
    	    clocks,
    	    ).unwrap().split()
	}



    #[cfg(feature = "stm32f7xx")]
    fn setup() -> (Tx<USART1>, Rx<USART1>) {

        let p = Peripherals::take().unwrap();
    	let rcc = p.RCC.constrain();
    	let clocks = rcc.cfgr.sysclk(216.mhz()).freeze();
        
        let gpioa = p.GPIOA.split();

    	Serial::new(
    	    p.USART1,
    	    (gpioa.pa9.into_alternate_af7(),			      //tx pa9
	     gpioa.pa10.into_alternate_af7()),  		      //rx pa10
    	    clocks,
    	    Config {
                    baud_rate: 9600.bps(),
                    oversampling: Oversampling::By16,
                    },
    	    ).split()
	}



    #[cfg(feature = "stm32h7xx")]
    fn setup() -> (Tx<USART1>, Rx<USART1>) {

       let p      = Peripherals::take().unwrap();
       let pwr    = p.PWR.constrain();
       let vos    = pwr.freeze();
       let rcc    = p.RCC.constrain();
       let ccdr   = rcc.sys_ck(160.mhz()).freeze(vos, &p.SYSCFG);
       let clocks = ccdr.clocks;
       let gpioa  = p.GPIOA.split(ccdr.peripheral.GPIOA);

       //let txrx =Serial::usart1(
       //    p.USART1,
       //    (gpioa.pa9.into_alternate_af7(),                          //tx pa9
       //     gpioa.pa10.into_alternate_af7()),                        //rx pa10
       //    9600.bps(),
       //    &clocks,
       //    ).unwrap().split()
       
       p.USART1.serial((gpioa.pa9.into_alternate_af7(),                //tx pa9
                        gpioa.pa10.into_alternate_af7()),              //rx pa10
                       9600.bps(), 
                       ccdr.peripheral.USART1, 
                       &clocks).unwrap().split()
       }


    #[cfg(feature = "stm32l0x")]
    fn setup() -> (Tx<USART1>, Rx<USART1>) {

        let p = Peripherals::take().unwrap();
    	let rcc = p.RCC.constrain();
    	let clocks = rcc.cfgr.freeze();
    	let gpioa = p.GPIOA.split();

    	p.USART1.cr1.modify(|_,w| w.rxneie().set_bit());  //need RX interrupt? 

    	Serial::usart1(
    	    p.USART1,
    	    (gpioa.pa9.into_alternate_af7(),			      //tx pa9
	     gpioa.pa10.into_alternate_af7()),  		      //rx pa10
    	    Config::default() .baudrate(9600.bps()),
    	    clocks,
    	    ).unwrap().split()
	}



    #[cfg(feature = "stm32l1xx")]
    fn setup() -> (Tx<USART1>, Rx<USART1>) {
        let p = Peripherals::take().unwrap();
    	let rcc = p.RCC.constrain();
    	let clocks = rcc.cfgr.freeze();
    	let gpioa = p.GPIOA.split();
    	let cnfg = Config::default() .baudrate(9600.bps());
    	p.USART1.cr1.modify(|_,w| w.rxneie().set_bit());  //need RX interrupt? 
    	Serial::usart1(
    	    p.USART1,
    	    (gpioa.pa9.into_push_pull_output(),			      //tx pa9
	     gpioa.pa10.into_push_pull_output()),  		      //rx pa10
    	    Config::default() .baudrate(9600.bps()),
    	    clocks,
    	    ).unwrap().split()
    	}



    #[cfg(feature = "stm32l4xx")]
    fn setup() -> (Tx<USART1>, Rx<USART1>) {

       let p         = Peripherals::take().unwrap();
       let mut flash = p.FLASH.constrain();
       let mut rcc   = p.RCC.constrain();
       let mut pwr   = p.PWR.constrain(&mut rcc.apb1r1);
       let clocks    = rcc.cfgr .sysclk(80.mhz()) .pclk1(80.mhz()) 
                             .pclk2(80.mhz()) .freeze(&mut flash.acr, &mut pwr);

       let mut gpioa = p.GPIOA.split(&mut rcc.ahb2);

       Serial::usart1(
           p.USART1,
           (gpioa.pa9.into_af7(&mut gpioa.moder, &mut gpioa.afrh),    //tx pa9
            gpioa.pa10.into_af7(&mut gpioa.moder, &mut gpioa.afrh)),  //rx pa10
           Config::default() .baudrate(9600.bps()),
           clocks,
           &mut rcc.apb2,
           ).split()
       }

    // End of hal/MCU specific setup. Following should be generic code.

    let (mut tx1, mut rx1) = setup();

    hprintln!("test write to console ...").unwrap();

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
