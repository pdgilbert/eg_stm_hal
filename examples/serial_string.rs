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
//use nb::block;

use eg_stm_hal::to_str;

#[cfg(feature = "stm32f0xx")]  //  eg blue pill stm32f103
use stm32f0xx_hal::{prelude::*,   
                    pac::Peripherals, 
                    serial::{Config, Serial, StopBits, Tx, Rx},  
		    device::{USART1, USART2, USART3},  
                    dma::{TxDma, RxDma, dma1::{C2, C3, C4, C5, C6, C7}},
                    }; 

    #[cfg(feature = "stm32f0xx")]
    fn setup() ->  (TxDma<Tx<USART1>, C4>, RxDma<Rx<USART1>, C5>, 
                    TxDma<Tx<USART2>, C7>, RxDma<Rx<USART2>, C6>, 
                    TxDma<Tx<USART3>, C2>, RxDma<Rx<USART3>, C3> )  {

        let p = Peripherals::take().unwrap();
    	let mut rcc = p.RCC.constrain();  
	let clocks = rcc.cfgr.freeze(&mut p.FLASH.constrain().acr); 
        let mut afio = p.AFIO.constrain(&mut rcc.apb2);
        
        let channels = p.DMA1.split(&mut rcc.ahb);

    	let mut gpioa = p.GPIOA.split(&mut rcc.apb2);

	let txrx1 = Serial::usart1(
    	    p.USART1,
    	    (gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh),     //tx pa9 
	     gpioa.pa10),					     //rx pa10
    	    &mut afio.mapr,
    	    Config::default() .baudrate(9600.bps()), //.stopbits(StopBits::STOP1
    	    clocks,
    	    &mut rcc.apb2,
    	    );

        let ( tx1, rx1)  = txrx1.split();
        let tx1  = tx1.with_dma(channels.4);            // console
        let rx1  = rx1.with_dma(channels.5);

        // ok let (_, tx1) = tx1.write(b"console connect check.").wait(); 
        // No (_, tx1) = tx1.write(b"console connect check.").wait(); 
        let tx1 = tx1.write(b"console connect check.").wait().1; 
 
        let txrx2 = Serial::usart2(
            p.USART2,
            (gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl),     //tx pa2 
             gpioa.pa3), 					     //rx pa3
            &mut afio.mapr,
            Config::default() .baudrate(9_600.bps()) .parity_odd() .stopbits(StopBits::STOP1),
            clocks,
            &mut rcc.apb1,
        );

        let ( tx2, rx2)  = txrx2.split();
        let tx2  = tx2.with_dma(channels.7);
        let rx2  = rx2.with_dma(channels.6);

        let mut gpiob = p.GPIOB.split(&mut rcc.apb2);

        let txrx3 = Serial::usart3(
            p.USART3,
            ( gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh),   //rx pb10  
              gpiob.pb11),  					     //tx pb11
            &mut afio.mapr,
            Config::default() .baudrate(9_600.bps()) .parity_odd() .stopbits(StopBits::STOP1),
            clocks,
            &mut rcc.apb1,    
        );

        let ( tx3, rx3)  = txrx3.split();
        let tx3  = tx3.with_dma(channels.2);
        let rx3  = rx3.with_dma(channels.3);

        (tx1, rx1,   tx2, rx2,   tx3, rx3 )
	}


#[cfg(feature = "stm32f1xx")]  //  eg blue pill stm32f103
use stm32f1xx_hal::{prelude::*,   
                    pac::Peripherals, 
                    serial::{Config, Serial, StopBits, Tx, Rx},  
		    device::{USART1, USART2, USART3},  
                    dma::{TxDma, RxDma, dma1::{C2, C3, C4, C5, C6, C7}},
                    }; 

    #[cfg(feature = "stm32f1xx")]
    fn setup() ->  (TxDma<Tx<USART1>, C4>, RxDma<Rx<USART1>, C5>, 
                    TxDma<Tx<USART2>, C7>, RxDma<Rx<USART2>, C6>, 
                    TxDma<Tx<USART3>, C2>, RxDma<Rx<USART3>, C3> )  {

        let p = Peripherals::take().unwrap();
    	let mut rcc = p.RCC.constrain();  
	let clocks = rcc.cfgr.freeze(&mut p.FLASH.constrain().acr); 
        let mut afio = p.AFIO.constrain(&mut rcc.apb2);
        
        let channels = p.DMA1.split(&mut rcc.ahb);

    	let mut gpioa = p.GPIOA.split(&mut rcc.apb2);

	let txrx1 = Serial::usart1(
    	    p.USART1,
    	    (gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh),     //tx pa9 
	     gpioa.pa10),					     //rx pa10
    	    &mut afio.mapr,
    	    Config::default() .baudrate(9600.bps()), //.stopbits(StopBits::STOP1
    	    clocks,
    	    &mut rcc.apb2,
    	    );

        let ( tx1, rx1)  = txrx1.split();
        let tx1  = tx1.with_dma(channels.4);            // console
        let rx1  = rx1.with_dma(channels.5);

        // ok let (_, tx1) = tx1.write(b"console connect check.").wait(); 
        // No (_, tx1) = tx1.write(b"console connect check.").wait(); 
        let tx1 = tx1.write(b"console connect check.").wait().1; 
 
        let txrx2 = Serial::usart2(
            p.USART2,
            (gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl),     //tx pa2 
             gpioa.pa3), 					     //rx pa3
            &mut afio.mapr,
            Config::default() .baudrate(9_600.bps()) .parity_odd() .stopbits(StopBits::STOP1),
            clocks,
            &mut rcc.apb1,
        );

        let ( tx2, rx2)  = txrx2.split();
        let tx2  = tx2.with_dma(channels.7);
        let rx2  = rx2.with_dma(channels.6);

        let mut gpiob = p.GPIOB.split(&mut rcc.apb2);

        let txrx3 = Serial::usart3(
            p.USART3,
            ( gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh),   //rx pb10  
              gpiob.pb11),  					     //tx pb11
            &mut afio.mapr,
            Config::default() .baudrate(9_600.bps()) .parity_odd() .stopbits(StopBits::STOP1),
            clocks,
            &mut rcc.apb1,    
        );

        let ( tx3, rx3)  = txrx3.split();
        let tx3  = tx3.with_dma(channels.2);
        let rx3  = rx3.with_dma(channels.3);

        (tx1, rx1,   tx2, rx2,   tx3, rx3 )
	}


    //#[cfg(any(feature = "stm32f1xx", feature = "stm32l1xx"))]
    //let channels = p.DMA1.split(&mut rcc.ahb);
    //let mut tx = txrx1.split().0.with_dma(channels.4);     //works on stm32f1xx_hal but not others
    //let (_, tx) = tx.write(b"The quick brown fox").wait(); //works on stm32f1xx_hal but not others


#[cfg(feature = "stm32f3xx")]  //  eg Discovery-stm32f303
use stm32f3xx_hal::{prelude::*, 
                    stm32::Peripherals,
                    serial::{ Serial, Tx, Rx},
		    stm32::{USART1, USART2, USART3} };

    #[cfg(feature = "stm32f3xx")]
    fn setup() ->  (Tx<USART1>, Rx<USART1>, Tx<USART2>, Rx<USART2>, Tx<USART3>, Rx<USART3> )  {
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
            115_200.bps(), // 9600.bps(), 
            clocks,
            &mut rcc.apb1,
            ).split();

        let mut gpiob = p.GPIOB.split(&mut rcc.ahb);

        let (tx3, rx3) = Serial::usart3(
            p.USART3,
            (gpiob.pb10.into_af7(&mut gpiob.moder, &mut gpiob.afrh),   //rx pb10
             gpiob.pb11.into_af7(&mut gpiob.moder, &mut gpiob.afrh)),  //tx pb11
            115_200.bps(), // 9600.bps(), 
            clocks,
            &mut rcc.apb1,  
            ).split();

        (tx1, rx1,   tx2, rx2,   tx3, rx3 )
	}



#[cfg(feature = "stm32f4xx")] // eg Nucleo-64  stm32f411
use stm32f4xx_hal::{prelude::*,  
                    pac::Peripherals, 
                    serial::{config::Config, Serial, Tx, Rx},
		    pac::{USART1, USART2, USART6} };

    #[cfg(feature = "stm32f4xx")]
    fn setup() ->  (Tx<USART1>, Rx<USART1>, Tx<USART2>, Rx<USART2>, Tx<USART6>, Rx<USART6>, )  {
        let p = Peripherals::take().unwrap();
    	let rcc = p.RCC.constrain();  
	let clocks = rcc.cfgr.freeze();
        let gpioa = p.GPIOA.split();
        p.USART1.cr1.modify(|_,w| w.rxneie().set_bit());  //need RX interrupt? 
        let (tx1, rx1) =  Serial::usart1(
           p.USART1,
    	   (gpioa.pa9.into_alternate_af7(),            //tx pa9
	    gpioa.pa10.into_alternate_af7()),          //rx pa10
    	   Config::default() .baudrate(9600.bps()),
    	   clocks
           ).unwrap().split(); 

        p.USART2.cr1.modify(|_,w| w.rxneie().set_bit());  //need RX interrupt? 
        let (tx2, rx2) = Serial::usart2(
           p.USART2,
           (gpioa.pa2.into_alternate_af7(),            //tx pa2
	    gpioa.pa3.into_alternate_af7()),           //rx pa3
           Config::default() .baudrate(115_200.bps()),  //.parity_odd() .stopbits(StopBits::STOP1)
           clocks,
           ).unwrap().split();

        p.USART6.cr1.modify(|_,w| w.rxneie().set_bit());  //need RX interrupt? 
        let (tx3, rx3) = Serial::usart6(      //  NOTE PINS and USART6 !!!
           p.USART6,
           (gpioa.pa11.into_alternate_af8(),           //tx pa11
	    gpioa.pa12.into_alternate_af8()),          //rx pa12
           Config::default() .baudrate(115_200.bps()) ,
           clocks,
           ).unwrap().split();

        (tx1, rx1,   tx2, rx2,   tx3, rx3 )
	}


#[cfg(feature = "stm32f7xx")] 
use stm32f7xx_hal::{prelude::*,  
                    pac::Peripherals, 
                    serial::{Config, Serial, Tx, Rx, Oversampling, },
		    pac::{USART1, USART2, USART3} };

    #[cfg(feature = "stm32f7xx")]
    fn setup() ->  (Tx<USART1>, Rx<USART1>, Tx<USART2>, Rx<USART2>, Tx<USART3>, Rx<USART3>, )  {

        let p = Peripherals::take().unwrap();
    	let clocks = p.RCC.constrain().cfgr.sysclk(216.mhz()).freeze();

        let gpioa = p.GPIOA.split();
        
        let (tx1, rx1) =  Serial::new(
           p.USART1,
    	   (gpioa.pa9.into_alternate_af7(),            //tx pa9
	    gpioa.pa10.into_alternate_af7()),          //rx pa10
    	   clocks,
           Config {
                baud_rate: 9600.bps(),
                oversampling: Oversampling::By16,
                character_match: None,
                },
           ).split(); 

        let (tx2, rx2) = Serial::new(
           p.USART2,
           (gpioa.pa2.into_alternate_af7(),            //tx pa2
	    gpioa.pa3.into_alternate_af7()),           //rx pa3
           clocks,
            Config {
                baud_rate: 115_200.bps(),
                oversampling: Oversampling::By16,
                character_match: None,
                },
           ).split();

        let gpiob = p.GPIOB.split();

        let (tx3, rx3) = Serial::new(  
           p.USART3,
           (gpiob.pb10.into_alternate_af7(),           //tx pb10
	    gpiob.pb11.into_alternate_af7()),          //rx pb11
           clocks,
            Config {
                baud_rate: 115_200.bps(),
                oversampling: Oversampling::By16,
                character_match: None,
                },
           ).split();

        (tx1, rx1,   tx2, rx2,   tx3, rx3 )
	}


#[cfg(feature = "stm32h7xx")] 
use stm32h7xx_hal::{prelude::*,  
                    pac::Peripherals, 
                    serial::{Tx, Rx},
		    pac::{USART1, USART2, USART3} };

    #[cfg(feature = "stm32h7xx")]
    fn setup() ->  (Tx<USART1>, Rx<USART1>, Tx<USART2>, Rx<USART2>, Tx<USART3>, Rx<USART3>, )  {

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
                                        115_200.bps(), 
                                        ccdr.peripheral.USART2, 
                                        &clocks).unwrap().split();


       let gpiob  = p.GPIOB.split(ccdr.peripheral.GPIOB);

       let (tx3, rx3) = p.USART3.serial((gpiob.pb10.into_alternate_af7(),     //tx pb10
                                         gpiob.pb11.into_alternate_af7()),    //rx pb11
                                        115_200.bps(), 
                                        ccdr.peripheral.USART3, 
                                        &clocks).unwrap().split();

       
       (tx1, rx1,   tx2, rx2,   tx3, rx3 )
       }


#[cfg(feature = "stm32l0xx")] 
use stm32l0xx_hal::{prelude::*,  
                    pac::Peripherals, 
                    serial::{config::Config, Serial, Tx, Rx},
		    pac::{USART1, USART2, USART6} };

    #[cfg(feature = "stm32l0xx")]
    fn setup() ->  (Tx<USART1>, Rx<USART1>, Tx<USART2>, Rx<USART2>, Tx<USART6>, Rx<USART6>, )  {
        let p = Peripherals::take().unwrap();
    	let rcc = p.RCC.constrain();  
	let clocks = rcc.cfgr.freeze();
        let gpioa = p.GPIOA.split();
        p.USART1.cr1.modify(|_,w| w.rxneie().set_bit());  //need RX interrupt? 
        let (tx1, rx1) =  Serial::usart1(
           p.USART1,
    	   (gpioa.pa9.into_alternate_af7(),            //tx pa9
	    gpioa.pa10.into_alternate_af7()),          //rx pa10
    	   Config::default() .baudrate(9600.bps()),
    	   clocks
           ).unwrap().split(); 

        p.USART2.cr1.modify(|_,w| w.rxneie().set_bit());  //need RX interrupt? 
        let (tx2, rx2) = Serial::usart2(
           p.USART2,
           (gpioa.pa2.into_alternate_af7(),            //tx pa2
	    gpioa.pa3.into_alternate_af7()),           //rx pa3
           Config::default() .baudrate(115_200.bps()),  //.parity_odd() .stopbits(StopBits::STOP1)
           clocks,
           ).unwrap().split();

        p.USART6.cr1.modify(|_,w| w.rxneie().set_bit());  //need RX interrupt? 
        let (tx3, rx3) = Serial::usart6(      //  NOTE PINS and USART6 !!!
           p.USART6,
           (gpioa.pa11.into_alternate_af8(),           //tx pa11
	    gpioa.pa12.into_alternate_af8()),          //rx pa12
           Config::default() .baudrate(115_200.bps()) ,
           clocks,
           ).unwrap().split();

        (tx1, rx1,   tx2, rx2,   tx3, rx3 )
	}


#[cfg(feature = "stm32l1xx") ] // eg  Discovery kit stm32l100 and Heltec lora_node STM32L151CCU6
use stm32l1xx_hal::{prelude::*, 
		    stm32::Peripherals, 
		    rcc,   // for ::Config but note name conflict with serial
                    serial::{Config, SerialExt, Tx, Rx},
		    stm32::{USART1, USART2, USART3} };

    #[cfg(feature = "stm32l1xx")]
    fn setup() ->  (Tx<USART1>, Rx<USART1>, Tx<USART2>, Rx<USART2>, Tx<USART3>, Rx<USART3> )  {

       let p       = Peripherals::take().unwrap();
       let mut rcc = p.RCC.freeze(rcc::Config::hsi());
       //let clocks  = rcc.cfgr.freeze();

       let gpioa   = p.GPIOA.split();


       // Note that setting the alternate function mode and push_pull input/output
       // is not necessary. The hal code knows to do this for a usart.
       let (tx1, rx1) =  p.USART1.usart(
                            (gpioa.pa9,                //tx pa9 
                             gpioa.pa10),              //rx pa10 
                            Config::default() .baudrate(9600.bps()), 
                            &mut rcc).unwrap().split();

       let (tx2, rx2) = p.USART2.usart(
                           (gpioa.pa2,                 //tx pa2 
                            gpioa.pa3),                //rx pa3 
                           Config::default() .baudrate(115_200.bps()), 
                           &mut rcc).unwrap().split();

       let gpiob   = p.GPIOB.split();

       let (tx3, rx3) = p.USART3.usart(
                           (gpiob.pb10,                                      //tx pb10 
                            gpiob.pb11),                                     //rx pb11 
                           Config::default() .baudrate(115_200.bps()), 
                           &mut rcc).unwrap().split();

        
        (tx1, rx1,   tx2, rx2,   tx3, rx3 )
	}


#[cfg(feature = "stm32l4xx")] 
use stm32l4xx_hal::{prelude::*,  
                    pac::Peripherals, 
                    serial::{Config, Serial, Tx, Rx},
		    pac::{USART1, USART2, USART3} };

    #[cfg(feature = "stm32l4xx")]
    fn setup() ->  (Tx<USART1>, Rx<USART1>, Tx<USART2>, Rx<USART2>, Tx<USART3>, Rx<USART3>, )  {

       let p         = Peripherals::take().unwrap();
       let mut flash = p.FLASH.constrain();
       let rcc       = p.RCC.constrain();  
       let mut rcc   = p.RCC.constrain();
       let mut pwr   = p.PWR.constrain(&mut rcc.apb1r1);
       let clocks    = rcc.cfgr .sysclk(80.mhz()) .pclk1(80.mhz()) 
                             .pclk2(80.mhz()) .freeze(&mut flash.acr, &mut pwr);

       let mut gpioa = p.GPIOA.split(&mut rcc.ahb2);

       let (tx1, rx1) =  Serial::usart1(
          p.USART1,
          (gpioa.pa9.into_af7(&mut gpioa.moder, &mut gpioa.afrh),            //tx pa9
           gpioa.pa10.into_af7(&mut gpioa.moder, &mut gpioa.afrh)),          //rx pa10
          Config::default() .baudrate(9600.bps()),
          clocks,
          &mut rcc.apb2,
          ).split(); 

       let (tx2, rx2) = Serial::usart2(
          p.USART2,
          (gpioa.pa2.into_af7(&mut gpioa.moder, &mut gpioa.afrl),            //tx pa2
           gpioa.pa3.into_af7(&mut gpioa.moder, &mut gpioa.afrl)),           //rx pa3
          Config::default() .baudrate(115_200.bps()),  
          clocks,
          &mut rcc.apb1r1,
          ).split();

       let mut gpiob = p.GPIOB.split(&mut rcc.ahb2);

       let (tx3, rx3) = Serial::usart3(      
          p.USART3,
          (gpiob.pb10.into_af7(&mut gpiob.moder, &mut gpiob.afrh),           //tx pb10
           gpiob.pb11.into_af7(&mut gpiob.moder, &mut gpiob.afrh)),          //rx pb11 
          Config::default() .baudrate(115_200.bps()) ,
          clocks,
          &mut rcc.apb2,
          ).split();

       (tx1, rx1,   tx2, rx2,   tx3, rx3 )
       }


    // End of hal/MCU specific setup. Following should be generic code.



#[entry]
fn main() -> ! {

    hprintln!("initializing ...").unwrap();
    hprintln!("testing console output ").unwrap();
    
    let ( tx1, _rx1,   mut tx2,  _rx2,    _tx3, rx3 ) = setup();  

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

    #[cfg(feature = "stm32f3xx")]
    for byte in b"\r\nconsole connect check.\r\n" { block!(tx1.write(*byte)).ok(); }

    #[cfg(feature = "stm32f4xx")]
    for byte in b"\r\nconsole connect check.\r\n" { block!(tx1.write(*byte)).ok(); }

    #[cfg(feature = "stm32l1xx")]
    for byte in b"\r\nconsole connect check.\r\n" { block!(tx1.write(*byte)).ok(); }

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
