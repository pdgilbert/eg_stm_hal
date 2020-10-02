//! Serial interface char-by-char read GPS on one usart2 and write on another 
//! to USB-TTL connected to console (minicom). 
//!
//! usart1 connect the Tx pin pa9  to the Rx pin of a serial-usb converter
//! usart1 connect the Rx pin pa10 to the Tx pin of a serial-usb converter
//! Set up the serial console (e.g. minicom) with the same settings used here.
//! (Using 9600bps, could be higher but needs serial console to be the same.)
//!
//! GPS uses 9600bps, 8bit, odd parity, 1 stopbit. This can be confirmed by connecting GPS 
//!  directly to the  USB-TTL and terminal with these settings (minicom 8-N-1) 
//! The usart and pins for the GPS depend on the board. For specifics see setup() sections below. 
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

// setup() does all  hal/MCU specific setup and returns generic hal device for use in main code.

#[cfg(feature = "stm32f0xx")]  // eg stm32f030xc
use stm32f0xx_hal::{prelude::*,   
                    pac::Peripherals, 
                    serial::{Serial, Tx, Rx},  
		    pac::{USART1, USART3}  }; 

    #[cfg(feature = "stm32f0xx")]
    fn setup() ->  (Tx<USART1>, Rx<USART1>, Tx<USART3>, Rx<USART3>)  {

        let mut p = Peripherals::take().unwrap();
        let mut rcc = p.RCC.configure().sysclk(48.mhz()).freeze(&mut p.FLASH);

	let gpioa = p.GPIOA.split(&mut rcc);
	let gpiob = p.GPIOB.split(&mut rcc);

        let (tx1, rx1,  tx3, rx3) = cortex_m::interrupt::free(move |cs| {
            (
                gpioa.pa9.into_alternate_af1(cs),     //tx pa9
                gpioa.pa10.into_alternate_af1(cs),    //rx pa10

                gpiob.pb10.into_alternate_af4(cs),    //tx pb10
                gpiob.pb11.into_alternate_af4(cs),    //rx pb11
            )
        });

	let (tx1, rx1) = Serial::usart1(
    	    p.USART1,
    	    (tx1, rx1),					    
    	    9600.bps(),   
    	    &mut rcc,
    	    ).split();

	let (tx3, rx3) = Serial::usart3(
    	    p.USART3,
    	    (tx3, rx3),					    
    	    9600.bps(),   
    	    &mut rcc,
    	    ).split();

        (tx1, rx1,   tx3, rx3 )
	}


#[cfg(feature = "stm32f1xx")]  //  eg blue pill stm32f103
use stm32f1xx_hal::{prelude::*,   
                    pac::Peripherals, 
                    serial::{Config, Serial, StopBits, Tx, Rx},  
		    device::{USART1, USART3}  }; 

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
    	    (gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh),     //tx pa9  for console
	     gpioa.pa10),					     //rx pa10 for console
    	    &mut afio.mapr,
    	    Config::default() .baudrate(9600.bps()) .stopbits(StopBits::STOP1), //.parity_odd()
    	    clocks,
    	    &mut rcc.apb2,
    	    ).split();

	let mut gpiob = p.GPIOB.split(&mut rcc.apb2);
        let (tx3, rx3) = Serial::usart3(
            p.USART3,
            (gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh),     //tx pb10  for GPS
             gpiob.pb11), 					      //rx pb11  for GPS
            &mut afio.mapr,
            Config::default() .baudrate(9_600.bps()), 
            clocks,
            &mut rcc.apb1,
        ).split();

        (tx1, rx1,   tx3, rx3 )
	}


#[cfg(feature = "stm32f3xx")]  //  eg Discovery-stm32f303
use stm32f3xx_hal::{prelude::*, 
                    stm32::Peripherals,
                    serial::{ Serial, Tx, Rx},
		    stm32::{USART1, USART2} };

    #[cfg(feature = "stm32f3xx")]
    fn setup() ->  (Tx<USART1>, Rx<USART1>, Tx<USART2>, Rx<USART2> )  {
        let p = Peripherals::take().unwrap();
    	let mut rcc = p.RCC.constrain();  
	let clocks  = rcc.cfgr.freeze(&mut p.FLASH.constrain().acr); 
        //Why does next need arg, there is only one possibility?
        let mut gpioa = p.GPIOA.split(&mut rcc.ahb); 
        let (tx1, rx1)  = Serial::usart1(
            p.USART1,
            (gpioa.pa9.into_af7( &mut gpioa.moder, &mut gpioa.afrh),   //tx pa9  for console
	     gpioa.pa10.into_af7(&mut gpioa.moder, &mut gpioa.afrh)),  //tx pb10 for console
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


#[cfg(feature = "stm32f4xx")] // eg Nucleo-64  stm32f411
use stm32f4xx_hal::{prelude::*,  
                    pac::Peripherals, 
                    serial::{config::Config, Serial, Tx, Rx},
		    pac::{USART1, USART2} };

    #[cfg(feature = "stm32f4xx")]
    fn setup() ->  (Tx<USART1>, Rx<USART1>, Tx<USART2>, Rx<USART2> )  {
        let p = Peripherals::take().unwrap();
        let clocks    =  p.RCC.constrain().cfgr.freeze();
        let gpioa = p.GPIOA.split();
        let (tx1, rx1) =  Serial::usart1(
           p.USART1,
    	   (gpioa.pa9.into_alternate_af7(),            //tx pa9  for console
	    gpioa.pa10.into_alternate_af7()),          //rx pa10 for console
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


#[cfg(feature = "stm32f7xx")]
use stm32f7xx_hal::{prelude::*,  
                    pac::Peripherals, 
                    serial::{Config, Serial, Tx, Rx, Oversampling, },
		    pac::{USART1, USART2} };

    #[cfg(feature = "stm32f7xx")]
    fn setup() ->  (Tx<USART1>, Rx<USART1>, Tx<USART2>, Rx<USART2> )  {

        let p = Peripherals::take().unwrap();
    	let clocks = p.RCC.constrain().cfgr.sysclk(216.mhz()).freeze();
        
        let gpioa = p.GPIOA.split();
        
        let (tx1, rx1) =  Serial::new(
           p.USART1,
    	   (gpioa.pa9.into_alternate_af7(),            //tx pa9  for console
	    gpioa.pa10.into_alternate_af7()),          //rx pa10 for console
    	   clocks,
           Config {
                baud_rate: 9600.bps(),
                oversampling: Oversampling::By16,
                character_match: None,
                },
           ).split(); 

        let (tx2, rx2) = Serial::new(
           p.USART2,
           (gpioa.pa2.into_alternate_af7(),            //tx pa2  for GPS
	    gpioa.pa3.into_alternate_af7()),           //rx pa3  for GPS
           clocks,
           Config {
                baud_rate: 9600.bps(),
                oversampling: Oversampling::By16,
                character_match: None,
                },
           ).split();

        (tx1, rx1,   tx2, rx2 )
	}


#[cfg(feature = "stm32h7xx")]
use stm32h7xx_hal::{prelude::*,  
                    pac::Peripherals, 
                    serial::{Tx, Rx},
		    pac::{USART1, USART2} };

    #[cfg(feature = "stm32h7xx")]
    fn setup() ->  (Tx<USART1>, Rx<USART1>, Tx<USART2>, Rx<USART2> )  {

       let p      = Peripherals::take().unwrap();
       let pwr    = p.PWR.constrain();
       let vos    = pwr.freeze();
       let rcc    = p.RCC.constrain();
       let ccdr   = rcc.sys_ck(160.mhz()).freeze(vos, &p.SYSCFG);
       let clocks = ccdr.clocks;
       let gpioa  = p.GPIOA.split(ccdr.peripheral.GPIOA);


       let (tx1, rx1) = p.USART1.serial((gpioa.pa9.into_alternate_af7(),     //tx pa9
                                         gpioa.pa10.into_alternate_af7()),   //rx pa10
                                        9600.bps(), 
                                        ccdr.peripheral.USART1, 
                                        &clocks).unwrap().split();

       
       let (tx2, rx2) = p.USART2.serial((gpioa.pa2.into_alternate_af7(),     //tx pa2
                                         gpioa.pa3.into_alternate_af7()),    //rx pa3
                                        9600.bps(), 
                                        ccdr.peripheral.USART2, 
                                        &clocks).unwrap().split();

        (tx1, rx1,   tx2, rx2 )
	}


#[cfg(feature = "stm32l0xx")]
use stm32l0xx_hal::{prelude::*,  
                    pac::Peripherals, 
		    rcc,   // for ::Config but note name conflict with serial
                    serial::{Config, Tx, Rx, Serial1Ext, Serial2Ext},
		    pac::{USART1, USART2} };

    #[cfg(feature = "stm32l0xx")]
    fn setup() ->  (Tx<USART1>, Rx<USART1>, Tx<USART2>, Rx<USART2> )  {
        let p = Peripherals::take().unwrap();
        let mut rcc = p.RCC.freeze(rcc::Config::hsi16());
        let gpioa   = p.GPIOA.split(&mut rcc);

        let (tx1, rx1) =  p.USART1.usart(
    	   gpioa.pa9,                               //tx pa9  for console
	   gpioa.pa10,                              //rx pa10 for console
    	   Config::default() .baudrate(9600.bps()),
    	   &mut rcc,
           ).unwrap().split(); 

        let (tx2, rx2) = p.USART2.usart(
           gpioa.pa2,                                //tx pa2  for GPS
	   gpioa.pa3,                                //rx pa3  for GPS
           Config::default() .baudrate(9600.bps()), 
           &mut rcc,
           ).unwrap().split();

        (tx1, rx1,   tx2, rx2 )
	}


#[cfg(feature = "stm32l1xx") ] // eg  Discovery kit stm32l100 and Heltec lora_node STM32L151CCU6
use stm32l1xx_hal::{prelude::*, 
		    stm32::Peripherals, 
		    rcc,   // for ::Config but note name conflict with next
                    serial::{Config, SerialExt, Tx, Rx},
		    stm32::{USART1, USART2} };

/*
The Heltec lora_node 151 uses USART2 and USART3 pins for on board LoRa connections and power detection.
See https://resource.heltec.cn/download/LoRa_Node_151/LoRa_Node_151_Pinout_Diagram.pdf.
So only USART1 is available and this example cannot work on Heltec lora_node 151 as it needs 2 USARTs. 
USART1 os used for the GPS as oled_gps and lora_gps examples might work. 
For simplicity of this example the same setup is used on the Discovery kit stm32l100.
*/

    #[cfg(feature = "stm32l1xx")]
    fn setup() ->  (Tx<USART2>, Rx<USART2>, Tx<USART1>, Rx<USART1> )  {

       let p = Peripherals::take().unwrap();
       let mut rcc = p.RCC.freeze(rcc::Config::hsi());
       //let clocks  = rcc.cfgr.freeze();

       let gpioa = p.GPIOA.split();

       let (txc, rxc) = p.USART2.usart(
                           (gpioa.pa2,                 //tx pa2   for console
                            gpioa.pa3),                //rx pa3   for console
                           Config::default() .baudrate(115_200.bps()), 
                           &mut rcc).unwrap().split();

       let (txg, rxg) =  p.USART1.usart(
                            (gpioa.pa9,                //tx pa9  for GPS rx
                             gpioa.pa10),              //rx pa10 for GPS tx
                            Config::default() .baudrate(9600.bps()), 
                            &mut rcc).unwrap().split();

       (txc, rxc,   txg, rxg )
       }


#[cfg(feature = "stm32l4xx")] // eg Nucleo-64  stm32f411
use stm32l4xx_hal::{prelude::*,  
                    pac::Peripherals, 
                    serial::{Config, Serial, Tx, Rx},
		    pac::{USART1, USART2} };

    #[cfg(feature = "stm32l4xx")]
    fn setup() ->  (Tx<USART1>, Rx<USART1>, Tx<USART2>, Rx<USART2> )  {

       let p         = Peripherals::take().unwrap();
       let mut flash = p.FLASH.constrain();
       let mut rcc   = p.RCC.constrain();
       let mut pwr   = p.PWR.constrain(&mut rcc.apb1r1);
       let clocks    = rcc.cfgr .sysclk(80.mhz()) .pclk1(80.mhz()) 
                             .pclk2(80.mhz()) .freeze(&mut flash.acr, &mut pwr);

       let mut gpioa = p.GPIOA.split(&mut rcc.ahb2);

       let (tx1, rx1) =  Serial::usart1(
          p.USART1,
          (gpioa.pa9.into_af7(&mut gpioa.moder, &mut gpioa.afrh),    //tx pa9  for console
           gpioa.pa10.into_af7(&mut gpioa.moder, &mut gpioa.afrh)),  //rx pa10 for console
          Config::default() .baudrate(9600.bps()),
          clocks,
          &mut rcc.apb2,
          ).split(); 

       let (tx2, rx2) = Serial::usart2(
          p.USART2,
          (gpioa.pa2.into_af7(&mut gpioa.moder, &mut gpioa.afrl),    //tx pa2  for GPS
           gpioa.pa3.into_af7(&mut gpioa.moder, &mut gpioa.afrl)),   //rx pa3  for GPS
          Config::default() .baudrate(9600.bps()), 
          clocks,
          &mut rcc.apb1r1,
          ).split();

       (tx1, rx1,   tx2, rx2 )
       }

// End of hal/MCU specific setup. Following should be generic code.

#[entry]
fn main() -> ! {
 
    //see serial_char.rs and serial_string.rs in examples/ for more USART config notes.

    let (mut tx_con, mut _rx_con,   mut _tx_gps, mut rx_gps) = setup();  // console,  GPS

    hprintln!("testing console output...").unwrap();
 
    for byte in b"Just confirming console works.\r\n" {
       block!(tx_con.write(*byte)).unwrap();
    }

    hprintln!("entering read/write loop...").unwrap();

    // note that putting hprintln! in loop slows it too much and loses data.
    let e: u8 = 9;
    loop { // Read a byte and write
      let received = match block!(rx_gps.read()) {
         Ok(str)     => str,
         Err(_error) => e,
         };
      //hprintln!("in loop...").unwrap(); //debugging only, too slow
      block!(tx_con.write(received)).ok();
      }

}
