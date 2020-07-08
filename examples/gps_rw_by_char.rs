//! Serial interface char-by-char read GPS on usart2 and write on usart1 
//! to USB-TTL connected to console (minicom) and also write to semihost. 
//!
//! usart1 connect the Tx pin pa9  to the Rx pin of a serial-usb converter
//! usart1 connect the Rx pin pa10 to the Tx pin of a serial-usb converter
//! Set up the serial console (e.g. minicom) with the same settings used here.
//! (Using 9600bps, could be higher but needs serial console to be the same.)
//!
//! GPS uses 9600bps, 8bit, odd parity, 1 stopbit. This can be confirmed by connecting GPS 
//!  directly to the  USB-TTL and terminal with these settings (minicom 8-N-1) 
//! usart2 connect the Rx pin pa3 to the Tx pin of GPS 
//! usart2 connect the Tx pin pa2 to the Rx pin of GPS
//! 
//! See examples/serial_char.rs for notes about connecting usart1 to 
//!   serial-usb converter on computer for console output.
//! That file also has for more notes regarding setup below.

#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[cfg(debug_assertions)]
extern crate panic_semihosting;

#[cfg(not(debug_assertions))]
extern crate panic_halt;

//use cortex_m::asm;
use cortex_m_rt::entry;
//use core::fmt::Write;
use cortex_m_semihosting::hprintln;
//use core::str::from_utf8;
use nb::block;

#[cfg(feature = "stm32f1xx")]  //  eg blue pill stm32f103
use stm32f1xx_hal::{prelude::*,   
                    pac::Peripherals, 
                    serial::{Config, Serial, StopBits, Tx, Rx},  
		    device::{USART1, USART2}  }; 

#[cfg(feature = "stm32f3xx")]  //  eg Discovery-stm32f303
use stm32f3xx_hal::{prelude::*, 
                    stm32::Peripherals,
                    serial::{ Serial, Tx, Rx},
		    stm32::{USART1, USART2} };

#[cfg(feature = "stm32f4xx")] // eg Nucleo-64  stm32f411
use stm32f4xx_hal::{prelude::*,  
                    pac::Peripherals, 
                    serial::{config::Config, Serial, Tx, Rx},
		    pac::{USART1, USART2} };

#[cfg(feature = "stm32l1xx") ] // eg  Discovery kit stm32l100 and Heltec lora_node STM32L151CCU6
use stm32l1xx_hal::{prelude::*, 
		    stm32::Peripherals, 
		    serial::{Config, Serial, Tx, Rx},
		    stm32::{USART1, USART2} };


#[entry]
fn main() -> ! {
 
    //see serial_char.rs and serial_string.rs in examples/ for more USART config notes.

    #[cfg(feature = "stm32f1xx")]
    fn setup() ->  (Tx<USART1>, Rx<USART1>, Tx<USART2>, Rx<USART2>)  {
        let p = Peripherals::take().unwrap();
    	let mut rcc = p.RCC.constrain();  
	let clocks = rcc.cfgr.freeze(&mut p.FLASH.constrain().acr); 
        let mut afio = p.AFIO.constrain(&mut rcc.apb2);
    	let mut gpioa = p.GPIOA.split(&mut rcc.apb2);

    	// next consumes (moves) arguments other than clocks,  &mut rcc.apb2 and afio.
	let (tx1, rx1) = Serial::usart1(
    	    p.USART1,
    	    (gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh),     //tx pa9, 
	     gpioa.pa10),					     //rx pa10
    	    &mut afio.mapr,
    	    Config::default() .baudrate(9600.bps()) .stopbits(StopBits::STOP1), //.parity_odd()
    	    clocks,
    	    &mut rcc.apb2,
    	    ).split();

        let (tx2, rx2) = Serial::usart2(
            p.USART2,
            (gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl),     //tx pa2 
             gpioa.pa3), 					     //rx pa3
            &mut afio.mapr,
            Config::default() .baudrate(9_600.bps()), 
            clocks,
            &mut rcc.apb1,
        ).split();

        (tx1, rx1,   tx2, rx2 )
	}




    #[cfg(feature = "stm32f3xx")]
    fn setup() ->  (Tx<USART1>, Rx<USART1>, Tx<USART2>, Rx<USART2> )  {
        let p = Peripherals::take().unwrap();
    	let mut rcc = p.RCC.constrain();  
	let clocks  = rcc.cfgr.freeze(&mut p.FLASH.constrain().acr); 
        //Why does next need arg, there is only one possibility?
        let mut gpioa = p.GPIOA.split(&mut rcc.ahb); 
        let (tx1, rx1)  = Serial::usart1(
            p.USART1,
            (gpioa.pa9.into_af7( &mut gpioa.moder, &mut gpioa.afrh),   //tx pa9
	     gpioa.pa10.into_af7(&mut gpioa.moder, &mut gpioa.afrh)),  //tx pb10
            9600.bps(),
            clocks,
            &mut rcc.apb2,
            ).split();

        let (tx2, rx2) = Serial::usart2(
            p.USART2,
            (gpioa.pa2.into_af7(&mut gpioa.moder, &mut gpioa.afrl),    //tx pa2
             gpioa.pa3.into_af7(&mut gpioa.moder, &mut gpioa.afrl)),   //rx pa3
            9600.bps(),    // 115_200.bps(),
            clocks,
            &mut rcc.apb1,
            ).split();
        (tx1, rx1,   tx2, rx2 )
	}


    #[cfg(feature = "stm32f4xx")]
    fn setup() ->  (Tx<USART1>, Rx<USART1>, Tx<USART2>, Rx<USART2> )  {
        let p = Peripherals::take().unwrap();
        let clocks    =  p.RCC.constrain().cfgr.freeze();
        let gpioa = p.GPIOA.split();
        let (tx1, rx1) =  Serial::usart1(
           p.USART1,
    	   (gpioa.pa9.into_alternate_af7(),            //tx pa9
	    gpioa.pa10.into_alternate_af7()),          //rx pa10
    	   Config::default() .baudrate(9600.bps()),
    	   clocks
           ).unwrap().split(); 

    	// this probably needs fix here. rx2.read() stalls and does not return.
	//p.USART2.cr1.modify(|_,w| w.rxneie().set_bit());  //need RX interrupt? 
        let (tx2, rx2) = Serial::usart2(
           p.USART2,
           (gpioa.pa2.into_alternate_af7(),            //tx pa2
	    gpioa.pa3.into_alternate_af7()),           //rx pa3
           Config::default() .baudrate(9600.bps()), 
           clocks,
           ).unwrap().split();

        (tx1, rx1,   tx2, rx2 )
	}



    #[cfg(feature = "stm32l1xx")]
    fn setup() ->  (Tx<USART1>, Rx<USART1>, Tx<USART2>, Rx<USART2> )  {
        let p = Peripherals::take().unwrap();
	let clocks    =  p.RCC.constrain().cfgr.freeze();
        let gpioa = p.GPIOA.split();
        let (tx1, rx1) =  Serial::usart1(
           p.USART1,
           (gpioa.pa9.into_alternate_af7(),            //tx pa9
	    gpioa.pa10.into_alternate_af7()),          //rx pa10
    	   Config::default() .baudrate(9600.bps()),
    	   clocks
           ).unwrap().split(); 

        let (tx2, rx2) = Serial::usart2(
            p.USART2,
            (gpioa.pa2.into_alternate_af7(),           //tx pa2
	     gpioa.pa3.into_alternate_af7()),          //rx pa3
            Config::default() .baudrate(9600.bps()), 
            clocks,
            ).unwrap().split();

        (tx1, rx1,   tx2, rx2 )
	}


    // End of hal/MCU specific setup. Following should be generic code.

    let (mut tx1, mut _rx1, mut _tx2, mut rx2) = setup();  // 1 is console, 2 is GPS

    hprintln!("testing console output...").unwrap();
 
    for byte in b"Just confirming console works.\r\n" {
       block!(tx1.write(*byte)).unwrap();
    }

    hprintln!("entering read/write loop...").unwrap();

    // note that putting hprintln! in loop slows it too much and loses data.
    let e: u8 = 9;
    loop { // Read a byte and write
      let received = match block!(rx2.read()) {
         Ok(str)     => str,
         Err(_error) => e,
         };
      //hprintln!("in loop...").unwrap(); //debugging only, too slow
      block!(tx1.write(received)).ok();
    }

    // PUT A TEST HERE THAT WILL SHOW FAILURE. ASSERT SEEMS TO PANIC HALT SO ...

    // Trigger a breakpoint to inspect the values
    //asm::bkpt();

}
