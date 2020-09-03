//! Serial interface test writing a single character between two usarts and
//! echo to the computer consol connected by usb-ttl dongle on another usart.
//! 
//! With all HALs and boards the console is on USART1 and uses pins pa9 and pa10.
//! With all HALs and boards one of the serially connected ports is USART2 using
//! pins pa2 and pa3.
//! With HALs stm32f1xx and stm32f3xx the other serially connected port is USART3 
//! using pins pb10 and pb11.
//! With HAL stm32f4xx  the other serially connected port is USART6 using pins pa11
//! and pa12. (USART3 is not available and pb11 is used interanl on some boards.)
//! 
//! Console connection details:
//! Connect usart1  to serial-usb converter on computer for the console.
//! usart1 connect the Tx pin pa9  to the Rx pin of a serial-usb converter
//! usart1 connect the Rx pin pa10 to the Tx pin of a serial-usb converter
//! Set up the serial console (e.g. minicom) with the same settings used here (8-N-1).
//! (Using 9600bps. This could be higher but needs serial console to be the same.)
//! 
//! USART interconnect details:
//! For HALs stm32f1xx and stm32f3xx connect pa2 to pb11 and pa3 to pb10.
//! For HAL stm32f4xx connect pa2 to pa12 and pa3 to pa11.


// This example contains the most extensive notes. 
// ANY NOTES SHOULD BE EXPANDED HERE IF THEY APPLY HERE, 
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
use stm32f1xx_hal::{prelude::*,   
                    pac::Peripherals, 
                    serial::{Config, Serial, StopBits, Tx, Rx},  
		    device::{USART1, USART2, USART3}  }; 

#[cfg(feature = "stm32f3xx")]  //  eg Discovery-stm32f303
use stm32f3xx_hal::{prelude::*, 
                    stm32::Peripherals,
                    serial::{ Serial, Tx, Rx},
		    stm32::{USART1, USART2, USART3} };

#[cfg(feature = "stm32f4xx")] // eg Nucleo-64  stm32f411
use stm32f4xx_hal::{prelude::*,  
                    pac::Peripherals, 
                    serial::{config::Config, Serial, Tx, Rx},
		    pac::{USART1, USART2, USART6} };

#[cfg(feature = "stm32f7xx")] 
use stm32f7xx_hal::{prelude::*,  
                    pac::Peripherals, 
                    serial::{config::Config, Serial, Tx, Rx},
		    pac::{USART1, USART2, USART6} };

#[cfg(feature = "stm32h7xx")] 
use stm32h7xx_hal::{prelude::*,  
                    pac::Peripherals, 
                    serial::{config::Config, Serial, Tx, Rx},
		    pac::{USART1, USART2, USART6} };

#[cfg(feature = "stm32l0xx")] 
use stm32l0xx_hal::{prelude::*,  
                    pac::Peripherals, 
                    serial::{config::Config, Serial, Tx, Rx},
		    pac::{USART1, USART2, USART6} };

#[cfg(feature = "stm32l1xx") ] // eg  Discovery kit stm32l100 and Heltec lora_node STM32L151CCU6
use stm32l1xx_hal::{prelude::*, 
		    stm32::Peripherals, 
		    serial::{Config, Serial, Tx, Rx},
		    stm32::{USART1, USART2, USART3} };

#[cfg(feature = "stm32l4xx")] 
use stm32l4xx_hal::{prelude::*,  
                    pac::Peripherals, 
                    serial::{config::Config, Serial, Tx, Rx},
		    pac::{USART1, USART2, USART6} };


#[entry]
fn main() -> ! {

    //see  examples/echo_by_char.rs for additional comments.

    // 1. Get access to the device specific peripherals 
    // 2. Take ownership of raw rcc and flash devices and convert to HAL structs
    // 3. Freeze system clocks and store the frozen frequencies in `clocks`
    // 4. Prepare the alternate function I/O registers
    // 5. Prepare the GPIO peripheral
    // 6. Set up usart devices. Take ownership of USART register and tx/rx pins.
    //    Other registers are used to enable and configure the device.

    hprintln!("initializing ...").unwrap();

    // BEGIN USART SETUP


    // stm32f4xx warns that mut is not needed in next, but other hals require it
    // let mut rcc = p.RCC.constrain();


    // stm32f1xx

    #[cfg(feature = "stm32f1xx")]
    fn setup() ->  (Tx<USART1>, Rx<USART1>, Tx<USART2>, Rx<USART2>, Tx<USART3>, Rx<USART3> )  {
        let p = Peripherals::take().unwrap();
    	let mut rcc = p.RCC.constrain();  
	let clocks = rcc.cfgr.freeze(&mut p.FLASH.constrain().acr); 
        let mut afio = p.AFIO.constrain(&mut rcc.apb2);
    	let mut gpioa = p.GPIOA.split(&mut rcc.apb2);
    	// next consumes (moves) arguments other than clocks,  &mut rcc.apb2 and afio.
	let (tx1, rx1) = Serial::usart1(
    	    p.USART1,
    	    (gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh),     //tx pa9 
	     gpioa.pa10),					     //rx pa10
    	    &mut afio.mapr,
    	    Config::default() .baudrate(9600.bps()), //.stopbits(StopBits::STOP1
    	    clocks,
    	    &mut rcc.apb2,
    	    ).split();
        let (tx2, rx2) = Serial::usart2(
            p.USART2,
            (gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl),     //tx pa2 
             gpioa.pa3), 					     //rx pa3
            &mut afio.mapr,
            Config::default() .baudrate(9_600.bps()) .parity_odd() .stopbits(StopBits::STOP1),
            clocks,
            &mut rcc.apb1,
        ).split();

        let mut gpiob = p.GPIOB.split(&mut rcc.apb2);

        let (tx3, rx3) = Serial::usart3(
            p.USART3,
            ( gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh),   //rx pb10  
              gpiob.pb11),  					     //tx pb11
            &mut afio.mapr,
            Config::default() .baudrate(9_600.bps()) .parity_odd() .stopbits(StopBits::STOP1),
            clocks,
            &mut rcc.apb1,    
        ).split();

        (tx1, rx1,   tx2, rx2,   tx3, rx3 )
	}


    // stm32f3xx

    // stm32f303vct  alternate funtion modes see  
    // https://www.rlocman.ru/i/File/dat/STMicroelectronics/Microcontrollers_MCU/STM32F303VCT6.pdf p42
    // AF7 on PA9  is usart1_Tx, on PA10 is usart1_Rx,
    // AF7 on PA2  is usart2_Tx, on PA3  is usart2_Rx,
    // AF7 on PB10 is usart3_Tx, on PB11 is usart3_Rx,

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



    // stm32f411re 

    // stm32f411re implements only usarts 1, 2, and 6. These can be configured on different pins.
    // alternate funtion modes see https://www.st.com/resource/en/datasheet/stm32f411re.pdf  p47.
    // AF7 on PA9  is usart1_Tx, on PA10 is usart1_Rx,
    // AF7 on PA2  is usart2_Tx, on PA3  is usart2_Rx,
    // AF8 on PA11 is usart6_Tx, on PA12 is usart6_Rx


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


    #[cfg(feature = "stm32h7xx")]
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



    // stm32l1xx 

    #[cfg(feature = "stm32l1xx")]
    fn setup() ->  (Tx<USART1>, Rx<USART1>, Tx<USART2>, Rx<USART2>, Tx<USART6>, Rx<USART6> )  {
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

        let (tx2, rx2) = Serial::usart2(
            p.USART2,
            (gpioa.pa2.into_alternate_af7(),           //tx pa2
	     gpioa.pa3.into_alternate_af7()),          //rx pa3
            Config::default() .baudrate(115_200.bps()), //.parity_odd().stopbits(StopBits::STOP1),
            clocks,
            ).unwrap().split();

        let (tx3, rx3) = Serial::usart6(      //  NOTE PINS and USART6 !!!
            p.USART6,
            (gpioa.pa11.into_alternate_af8(),          //tx pa11
	     gpioa.pa12.into_alternate_af8()),         //rx pa12
            Config::default() .baudrate(115_200.bps()) ,
            clocks,
            ).unwrap().split();

        (tx1, rx1,   tx2, rx2,   tx3, rx3 )
	}


    #[cfg(feature = "stm32l4xx")]
    fn setup() ->  (Tx<USART1>, Rx<USART1>, Tx<USART2>, Rx<USART2>, Tx<USART6>, Rx<USART6>, )  {

       let p = Peripherals::take().unwrap();
       let mut flash = p.FLASH.constrain();
       let rcc = p.RCC.constrain();  
       let mut rcc = p.RCC.constrain();
       let mut pwr = p.PWR.constrain(&mut rcc.apb1r1);
       let clocks = rcc.cfgr .sysclk(80.mhz()) .pclk1(80.mhz()) 
                             .pclk2(80.mhz()) .freeze(&mut flash.acr, &mut pwr);

       let gpioa = p.GPIOA.split(&mut rcc.ahb2);

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
          (gpioa.pa2.into_af7(&mut gpioa.moder, &mut gpioa.afrh),            //tx pa2
           gpioa.pa3.into_af7(&mut gpioa.moder, &mut gpioa.afrh)),           //rx pa3
          Config::default() .baudrate(115_200.bps()),  
          clocks,
          &mut rcc.apb2,
          ).split();

       let (tx3, rx3) = Serial::usart6(      
          p.USART6,
          (gpioa.pa11.into_af8(&mut gpioa.moder, &mut gpioa.afrh),           //tx pa11   CHECK af8 ?
           gpioa.pa12.into_af8(&mut gpioa.moder, &mut gpioa.afrh)),          //rx pa12   CHECK af8 ?
          Config::default() .baudrate(115_200.bps()) ,
          clocks,
          &mut rcc.apb2,
          ).split();

       (tx1, rx1,   tx2, rx2,   tx3, rx3 )
       }


    // End of hal/MCU specific setup. Following should be generic code.

    
    let (mut tx1, _rx1,  mut tx2, mut rx2,  mut tx3, mut rx3 ) = setup();  

    hprintln!("test write to console ...").unwrap();
    for byte in b"\r\nconsole connect check.\r\n" { block!(tx1.write(*byte)).ok(); }

    hprintln!("testing  tx2 to rx3").unwrap();
    hprintln!("   sending on tx2 ...").unwrap();

    let send = b'X';

    // Write `X` and wait until the write is successful
    block!(tx2.write(send)).ok();

    hprintln!("   receiving on rx3 ...").unwrap();

    // Read the byte that was just send. Blocks until the read is complete
    let received = block!(rx3.read()).unwrap();

    hprintln!("   checking tx2 to rx3 received = send,  {} = {} byte", received, send).unwrap();

    // The send byte should be the one received
    assert_eq!(received, send, "testing received = send,  {} = {}", received, send);

    // PUT A TEST HERE THAT WILL SHOW FAILURE. ASSERT SEEMS TO PANIC HALT SO ...

    // Now print to semi-host as character rather than byte.
    // Note that send above was u8 byte (b'X') because tx.write() requires that, but
    //    hprintln!() needs a str and from_utf8() needs a slice, thus [send].
    
    hprintln!("   tx2 to rx3  characters,  {} = {}", 
        from_utf8(&[received]).unwrap(), from_utf8(&[send]).unwrap()).unwrap();

    hprintln!("   sending received to console on tx1 ...").unwrap();

    for byte in  b"\r\ntx2 to rx3 with X\r\n" {  // iterator fails if string is too long
       block!(tx1.write(*byte)).unwrap();
    }
    //block!(tx1.write(received)).unwrap();
    block!(tx1.write(received)).ok();
    for byte in  b"\r\n" {
       block!(tx1.write(*byte)).unwrap();
    }

    // Trigger a breakpoint 
    // asm::bkpt();

    hprintln!("testing  tx3 to rx2").unwrap();
    hprintln!("   sending on tx3 ...").unwrap();

    let send = b'Y';

    // Write `Y` and wait until the write is successful
    block!(tx3.write(send)).ok();

    hprintln!("   receiving on rx2 ...").unwrap();

    // Read the byte that was just send. Blocks until the read is complete
    let received = block!(rx2.read()).unwrap();

    hprintln!("    checking tx3 to rx2  received = send,  {} = {} byte", received, send).unwrap();

    // The send byte should be the one received
    //assert_eq!(received, send, "testing received = send,  {} = {}", received, send);
    
    hprintln!("   tx3 to rx2  characters,  {} = {}", 
        from_utf8(&[received]).unwrap(), from_utf8(&[send]).unwrap()).unwrap();

    hprintln!("   sending received from rx2  to console on tx1 ...").unwrap();

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
