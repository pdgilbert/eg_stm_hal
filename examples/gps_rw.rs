//! Serial interface read GPS one usart and write on another usart to USB-TTL console (minicom).
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

//use eg_stm_hal::to_str;

use cortex_m_rt::entry;
//use core::fmt::Write;  // for writeln
use cortex_m_semihosting::hprintln;
//use core::str;
//use core::ascii;
use nb::block;

//use eg_stm_hal::to_str;


#[cfg(feature = "stm32f1xx")]  //  eg blue pill stm32f103
use stm32f1xx_hal::{prelude::*,   
                    pac::Peripherals, 
                    serial::{Config, Serial, StopBits, Tx, Rx},  
		    device::{USART1, USART3}  }; 

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



//use heapless::{consts, Vec};

#[entry]

fn main() -> ! {

    //see serial_loopback_char.rs and serial_cross.rs in examples/ for more USART config notes.
    //    USART    (tx, rx)

    //hprintln!("{}", to_str("just checking to_str".as_bytes())).expect("hprintln error."); 
    //hprintln!("{:?}",      "just checking to_str".as_bytes()).expect("hprintln error."); 

    #[cfg(feature = "stm32f1xx")]
    fn setup() ->  (Tx<USART1>, Rx<USART1>, Tx<USART3>, Rx<USART3>)  {
        let p = Peripherals::take().unwrap();
    	let mut rcc = p.RCC.constrain();  
	let clocks = rcc.cfgr.freeze(&mut p.FLASH.constrain().acr); 
        let mut afio = p.AFIO.constrain(&mut rcc.apb2);
    	let mut gpioa = p.GPIOA.split(&mut rcc.apb2);

    	// next consumes (moves) arguments other than clocks,  &mut rcc.apb2 and afio.
	let (tx1, rx1) = Serial::usart1(
    	    p.USART1,
    	    (gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh),     //tx pa9   for console
	     gpioa.pa10),					     //rx pa10  for console
    	    &mut afio.mapr,
    	    Config::default() .baudrate(9600.bps()) .stopbits(StopBits::STOP1), //.parity_odd()
    	    clocks,
    	    &mut rcc.apb2,
    	    ).split();

    	let mut gpiob = p.GPIOB.split(&mut rcc.apb2);
        let (tx3, rx3) = Serial::usart3(
            p.USART3,
            (gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh),    //tx pb10  for GPS
             gpiob.pb11), 					     //rx pb11  for GPS
            &mut afio.mapr,
            Config::default() .baudrate(9_600.bps()), 
            clocks,
            &mut rcc.apb1,
        ).split();

        (tx1, rx1,   tx3, rx3 )
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
            (gpioa.pa9.into_af7( &mut gpioa.moder, &mut gpioa.afrh),   //tx pa9   for console
	     gpioa.pa10.into_af7(&mut gpioa.moder, &mut gpioa.afrh)),  //tx pb10  for console
            9600.bps(),
            clocks,
            &mut rcc.apb2,
            ).split();

        let (tx2, rx2) = Serial::usart2(
            p.USART2,
            (gpioa.pa2.into_af7(&mut gpioa.moder, &mut gpioa.afrl),    //tx pa2  for GPS
             gpioa.pa3.into_af7(&mut gpioa.moder, &mut gpioa.afrl)),   //rx pa3  for GPS
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
    	   (gpioa.pa9.into_alternate_af7(),            //tx pa9   for console
	    gpioa.pa10.into_alternate_af7()),          //rx pa10  for console
    	   Config::default() .baudrate(9600.bps()),
    	   clocks
           ).unwrap().split(); 

    	// this probably needs fix here. rx2.read() stalls and does not return.
	//p.USART2.cr1.modify(|_,w| w.rxneie().set_bit());  //need RX interrupt? 
        let (tx2, rx2) = Serial::usart2(
           p.USART2,
           (gpioa.pa2.into_alternate_af7(),            //tx pa2  for GPS
	    gpioa.pa3.into_alternate_af7()),           //rx pa3  for GPS
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
           (gpioa.pa9.into_alternate_af7(),            //tx pa9   for console
	    gpioa.pa10.into_alternate_af7()),          //rx pa10  for console
    	   Config::default() .baudrate(9600.bps()),
    	   clocks
           ).unwrap().split(); 

        let (tx2, rx2) = Serial::usart2(
            p.USART2,
            (gpioa.pa2.into_alternate_af7(),           //tx pa2  for GPS
	     gpioa.pa3.into_alternate_af7()),          //rx pa3  for GPS
            Config::default() .baudrate(9600.bps()), 
            clocks,
            ).unwrap().split();

        (tx1, rx1,   tx2, rx2 )
	}


    // End of hal/MCU specific setup. Following should be generic code.

    let (mut tx_con, mut _rx_con,   mut _tx_gps, mut rx_gps) = setup();  // console, GPS

    //writeln!(tx_con, "\r\nconsole connect check.\r\n").unwrap();
    for byte in b"\r\nconsole connect check.\r\n" { block!(tx_con.write(*byte)).ok(); }

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
        let byte = match block!(rx_gps.read()) {
	    Ok(byt)	  => byt,
	    Err(_error) => e,
	    };
        block!(tx_con.write(byte)).ok();
        if   byte == 36  {  //  $ is 36. start of a line
	   buffer.clear();
	   good = true;     //start capturing line
	   };
	if good {
	   if buffer.push(byte).is_err() ||  byte == 13  {  //  \r is 13, \n is 10
              //writeln!(tx_con, "{}", to_str(&buffer)).unwrap();
              for byte in &buffer { block!(tx_con.write(*byte)).ok(); }
              //hprintln!("buffer at {} of {}", buffer.len(), buffer.capacity()).unwrap();
              buffer.clear();
	      good = false;
	      //break; 
	      };
	   };
	}
}
