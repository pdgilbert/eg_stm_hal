//! Serial DMA RX transfer. Read 15 chars input from console on USART1, echo back to console, 
//!  and output to semihost. Repeat.
//! 
//! See examples/serial_char.rs for notes about connecting usart1 to 
//!   serial ttl-usb converter on computer for console output.
//! That file also has more notes regarding setup.

#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[cfg(debug_assertions)]
extern crate panic_semihosting;

#[cfg(not(debug_assertions))]
extern crate panic_halt;

use cortex_m::singleton;
use cortex_m_rt::entry;
//use core::fmt::Write;  // for writeln, but not supported by stm32f3xx_hal
use cortex_m_semihosting::hprintln;
//use nb::block;

use eg_stm_hal::to_str;


// setup() does all  hal/MCU specific setup and returns generic hal device for use in main code.

#[cfg(feature = "stm32f0xx")]  //  eg blue pill stm32f103
use stm32f0xx_hal::{prelude::*,   
                    pac::Peripherals, 
                    serial::{Serial, Tx, Rx},
		    dma::{RxDma, dma1::{C5}},     //TxDma,  C4, 
		    pac::USART1 }; 
    
    #[cfg(feature = "stm32f0xx")]
    fn setup() ->  (Tx<USART1>, RxDma<Rx<USART1>>)  {
       
        let mut p = Peripherals::take().unwrap();
        let mut rcc = p.RCC.configure().sysclk(48.mhz()).freeze(&mut p.FLASH);

	let gpioa = p.GPIOA.split(&mut rcc);

        let (tx, rx) = cortex_m::interrupt::free(move |cs| {
            (
                gpioa.pa9.into_alternate_af1(cs),     //tx pa9
                gpioa.pa10.into_alternate_af1(cs),    //rx pa10
            )
        });

	let txrx1 = Serial::usart1(
    	    p.USART1,
    	    (tx, rx),					    
    	    9600.bps(),   
    	    &mut rcc,
    	    );

       let channels = p.DMA1.split(&mut rcc.ahb);
       let (tx1, rx1)  = txrx1.split();
       //let tx1 = tx1.with_dma(channels.4);
       //let rx1 = rx1.with_dma(channels.5);
       (tx1, rx1)
       }



#[cfg(feature = "stm32f1xx")]  //  eg blue pill stm32f103
use stm32f1xx_hal::{prelude::*,   
                    pac::Peripherals, 
                    serial::{Config, Serial, StopBits, Tx, Rx},
		    dma::{dma1, }, 
		    device::USART1 }; 
    
    #[cfg(feature = "stm32f1xx")]
    fn setup() ->  (Tx<USART1>, dma1::C4, Rx<USART1>, dma1::C5)  {

    // fn setup() ->  (TxDma<Tx<USART1>, C4>, RxDma<Rx<USART1>, C5>)  {
    
       // with Tx return  Tx<USART1> ; with TxDma return  TxDma<Tx<USART1>, C4>
       let p = Peripherals::take().unwrap();
       let mut rcc = p.RCC.constrain();  
       let clocks = rcc.cfgr.freeze(&mut p.FLASH.constrain().acr); 
       let mut afio = p.AFIO.constrain(&mut rcc.apb2);
       let mut gpioa = p.GPIOA.split(&mut rcc.apb2);

       let txrx1 = Serial::usart1(
	   p.USART1,
	   (gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh),     //tx pa9, 
            gpioa.pa10),					    //rx pa10
	   &mut afio.mapr,
	   Config::default() .baudrate(9600.bps()) .stopbits(StopBits::STOP1),
	   clocks,
	   &mut rcc.apb2,
	   );  //.split();

       let (tx1, rx1)  = txrx1.split();

       // Note: in stm32f1xx_hal writeln! does not work with TxDma
       //  writeln!(tx1, "\r\ncheck console output.\r\n").unwrap();
       //  and without dma tx1.write() expects u8 not &[u8; 25]

       let dma1 = p.DMA1.split(&mut rcc.ahb);
       let (tx1_ch, rx1_ch) = (dma1.4, dma1.5);
 
       //let tx1 = tx1.with_dma(dma1.4);
       //let rx1 = rx1.with_dma(dma1.5);
       //(tx1, rx1)

       (tx1, tx1_ch,   rx1, rx1_ch)
       }



#[cfg(feature = "stm32f3xx")]  //  eg Discovery-stm32f303
use stm32f3xx_hal::{prelude::*, 
                    stm32::Peripherals,
		    serial::{Serial, Tx, Rx}, 
		    dma::dma1, 
		    stm32::USART1 
		    };

    #[cfg(feature = "stm32f3xx")]
    fn setup() ->  (Tx<USART1>, dma1::C4, Rx<USART1>, dma1::C5)  {

       let p = Peripherals::take().unwrap();
       let mut rcc = p.RCC.constrain();  
       let clocks    = rcc.cfgr.freeze(&mut p.FLASH.constrain().acr);
       let mut gpioa = p.GPIOA.split(&mut rcc.ahb); 

       let txrx1 = Serial::usart1(
    	   p.USART1,
    	   (gpioa.pa9.into_af7( &mut gpioa.moder, &mut gpioa.afrh), 
 	    gpioa.pa10.into_af7(&mut gpioa.moder, &mut gpioa.afrh)),
    	   9600.bps(),
    	   clocks,
    	   &mut rcc.apb2,
           );

       let (tx1, rx1)  = txrx1.split();

       let dma1 = p.DMA1.split(&mut rcc.ahb);
       let (tx1_ch, rx1_ch) = (dma1.ch4, dma1.ch5);
       //let (tx2_ch, rx2_ch) = (dma1.ch6, dma1.ch7);
       //let (tx3_ch, rx3_ch) = (dma1.ch3, dma1.ch2);

       (tx1, tx1_ch,   rx1, rx1_ch)
       }


#[cfg(feature = "stm32f4xx")] // eg Nucleo-64  stm32f411
use stm32f4xx_hal::{prelude::*, 
                    pac::Peripherals, 
		    serial::{config::Config, Serial, Tx, Rx},
		    pac::dma1, 
		    pac::USART1 
		    };

    #[cfg(feature = "stm32f4xx")]
    fn setup() ->  () {  //(Tx<USART1>, dma1::C4, Rx<USART1>, dma1::C5)  {

       let p = Peripherals::take().unwrap();
       let mut rcc = p.RCC.constrain();  
       let clocks = rcc.cfgr.freeze();
       let gpioa = p.GPIOA.split();
       //p.USART1.cr1.modify(|_,w| w.rxneie().set_bit());  //need RX interrupt? 

       let txrx1 =  Serial::usart1(
          p.USART1,
          (gpioa.pa9.into_alternate_af7(), 
	   gpioa.pa10.into_alternate_af7()), 
    	  Config::default() .baudrate(9600.bps()),
    	  clocks,
          ).unwrap();    

       let (mut tx1, mut rx1)  = txrx1.split();
       
       let dma1 = p.DMA1.split(&mut rcc.cfgr);  
       let (tx1_ch, rx1_ch) = (dma1.ch4, dma1.ch5);
       
       (tx1, tx1_ch,   rx1, rx1_ch)
       }



#[cfg(feature = "stm32f7xx")] 
use stm32f7xx_hal::{prelude::*, 
                    pac::Peripherals, 
		    serial::{Config, Serial, Tx, Rx},
		    pac::USART1 
		    };

    #[cfg(feature = "stm32f7xx")]
    fn setup() ->  (Tx<USART1>, Rx<USART1>)  {

       let p = Peripherals::take().unwrap();
       let mut rcc = p.RCC.constrain();  
       let clocks = rcc.cfgr.freeze();
       let gpioa = p.GPIOA.split();
       p.USART1.cr1.modify(|_,w| w.rxneie().set_bit());  //need RX interrupt? 

       let txrx1 =  Serial::usart1(
          p.USART1,
          (gpioa.pa9.into_alternate_af7(), 
	   gpioa.pa10.into_alternate_af7()), 
    	  Config::default() .baudrate(9600.bps()),
    	  clocks,
          ).unwrap();    

       let (mut tx1, mut rx1)  = txrx1.split();
       (tx1, rx1)
       }



#[cfg(feature = "stm32h7xx")] 
use stm32h7xx_hal::{prelude::*, 
                    pac::Peripherals, 
		    serial::{Tx, Rx},
		    pac::USART1 
		    };

    #[cfg(feature = "stm32h7xx")]
    fn setup() ->  (Tx<USART1>, Rx<USART1>)  {

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



#[cfg(feature = "stm32l0xx")]
use stm32l0xx_hal::{prelude::*,  
                    pac::Peripherals, 
		    rcc,   // for ::Config but note name conflict with serial
                    serial::{Config, Tx, Rx, Serial1Ext},
		    pac::USART1 
		    };

    #[cfg(feature = "stm32l0xx")]
    fn setup() -> (Tx<USART1>, Rx<USART1>) {

       let p       = Peripherals::take().unwrap();
       let mut rcc = p.RCC.freeze(rcc::Config::hsi16());
       let gpioa   = p.GPIOA.split(&mut rcc);
        
       p.USART1.usart(
            gpioa.pa9,                                          //tx pa9 
            gpioa.pa10,                                         //rx pa10
            Config::default(), 
            &mut rcc
            ).unwrap().split()
       }



#[cfg(feature = "stm32l1xx") ] // eg  Discovery kit stm32l100 and Heltec lora_node STM32L151CCU6
use stm32l1xx_hal::{prelude::*, 
                    stm32::Peripherals, 
		    rcc,   // for ::Config but note name conflict with next
                    serial::{Config, SerialExt, Tx, Rx},
		    stm32::USART1 
		    };

    #[cfg(feature = "stm32l1xx")]
    fn setup() ->  (Tx<USART1>, Rx<USART1>)  {

       let p = Peripherals::take().unwrap();
       let mut rcc = p.RCC.constrain();  
       let clocks = rcc.cfgr.freeze();
       let gpioa = p.GPIOA.split();
       p.USART1.cr1.modify(|_,w| w.rxneie().set_bit());  //need RX interrupt? 
       let txrx1 =  Serial::usart1(
          p.USART1,
    	  (gpioa.pa9,                                      //tx pa9 
    	   gpioa.pa10),                                    //rx pa10 
    	  Config::default() .baudrate(9600.bps()),
    	  clocks,
          ).unwrap();    
 
       let (mut tx1, mut rx1)  = txrx1.split();
       (tx1, rx1)
       }



#[cfg(feature = "stm32l4xx")] 
use stm32l4xx_hal::{prelude::*, 
                    pac::Peripherals, 
		    serial::{Config, Serial, Tx, Rx},
		    //pac::dma1, 
		    dma::dma1::{C4, C5},
		    pac::USART1 
		    };

    #[cfg(feature = "stm32l4xx")]
    fn setup() -> (Tx<USART1>, C4, Rx<USART1>, C5)  {

       let p = Peripherals::take().unwrap();
       let mut flash = p.FLASH.constrain();
       let mut rcc = p.RCC.constrain();  
       let mut pwr = p.PWR.constrain(&mut rcc.apb1r1);
       let clocks = rcc.cfgr .sysclk(80.mhz()) .pclk1(80.mhz()) 
                             .pclk2(80.mhz()) .freeze(&mut flash.acr, &mut pwr);
       let mut gpioa = p.GPIOA.split(&mut rcc.ahb2);

       let txrx1 =  Serial::usart1(
           p.USART1,
           (gpioa.pa9.into_af7(&mut gpioa.moder, &mut gpioa.afrh),    //tx pa9
            gpioa.pa10.into_af7(&mut gpioa.moder, &mut gpioa.afrh)),  //rx pa10
           Config::default() .baudrate(9600.bps()),
           clocks,
           &mut rcc.apb2,
           );

       let (tx1, rx1)  = txrx1.split();

       let dma1 = p.DMA1.split(&mut rcc.ahb1);
       let (tx1_ch, rx1_ch) = (dma1.4, dma1.5);
       //let (tx2_ch, rx2_ch) = (dma1.6, dma1.7);
       //let (tx3_ch, rx3_ch) = (dma1.3, dma1.2);

       (tx1, tx1_ch,   rx1, rx1_ch)
       }



    // End of hal/MCU specific setup. Following should be generic code.

 

#[entry]
fn main() -> ! {
     
    //see serial_char.rs and  echo_by_char.rs for additional comments.

    let (tx1, tx1_ch,   rx1, rx1_ch) = setup();

    hprintln!("test write to console ...").unwrap();

    let buf = singleton!(: [u8; 15] = *b"\r\ncheck console").unwrap();
    
    let send = tx1.write_all(buf, tx1_ch);
    let x = send.wait();                         //this is 3-tuple
    let (buf, tx1_ch, tx1) = x;
    
    // But attempting to modify rather than re-assign does not work.
    //   (buf, tx1_ch, tx1) = x;
    // gives cannot ... destructuring assignments are not currently supported

    *buf = *b"\r\nSlowly type  ";  //NB. 15 characters
    
    // Note that the buf assigned next is the one that will be used below in recv() and
    //   its size is determined by the size of the argument buf (15 as set above).
    
    let (buf, tx1_ch, tx1) = tx1.write_all(buf, tx1_ch).wait();
    
    let longer_buf = singleton!(: [u8; 36] = *b"15 characters in console.  Repeat.\r\n").unwrap();
    let (_longer_buf, tx1_ch, tx1) = tx1.write_all(longer_buf, tx1_ch).wait();


    // Now read from console into  buf and echo back to console

    hprintln!("Enter 15 characters in console. Repeat.").unwrap();
    hprintln!("Use ^C in gdb to exit.").unwrap();

    // repeat read buf and echo. Outside of loop this would work, even multple times:
    //   let (buf, rx1_ch, rx1) = rx1.read_exact(buf, rx1_ch).wait();
    //   let (buf, tx1_ch, tx1) = tx1.write_all( buf, tx1_ch).wait();
    // but  when moved into a loop there are problems with
    // value moved in previous iteration of loop.
    // That problem would be fixed by modifying mut variables but because of 
    // "cannot ... destructuring assignments" problem the 3-tuple needs to be kept
    // and elements referenced.


    // create recv and send structures that can be modified in loop rather than re-assigned.
    let mut recv = rx1.read_exact(buf, rx1_ch).wait();    //this is 3-tuple (buf, rx1_ch, rx1)
    hprintln!("first 15 characters received {:?}", to_str(recv.0)).unwrap();
    let mut send = tx1.write_all(recv.0, tx1_ch).wait();  //this is 3-tuple (buf, tx1_ch, tx1)

    // Note send (write) is using buf as put into recv (read). The returned buffer in recv and
    //   the argument buffer in send are data. The argument buffer in recv may be a holding spot 
    //   to put return buffer? but it is not part of the program logic. The size of the return
    //   buffer from recv does seem to be determined by the size of the recv argument buffer.
    //   The return buffer from send seems like it should be unnecessary, but it does provide
    //   the buffer needed in the recv argument.
    
    //each pass in loop waits for input of 15 chars typed in console then echos them
    loop { 
       recv = recv.2.read_exact(send.0, recv.1).wait();   
       send = send.2.write_all( recv.0, send.1).wait(); 
       }
}
