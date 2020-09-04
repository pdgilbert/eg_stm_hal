//! Serial interface read GPS on usart and transmit with LoRa (on SPI).
//! This example is similar to gps_rw and  lora_send.

//   Using  sck, miso, mosi, cs, and reset.
//   See hardware sections below for pin setup.
//   Not yet using D00, D01, D02, D03

//DIO0  triggers RxDone/TxDone status.
//DIO1  triggers RxTimeout and other errors status.
//MOSI, MISO, SCLK for SPI communication. 
//NSS is the chip select (CS) signal. 
//REST is reset.


//https://www.rfwireless-world.com/Tutorials/LoRa-channels-list.html
//channels = {
//   'CH_00_900': 903.08, 'CH_01_900': 905.24, 'CH_02_900': 907.40,
//   'CH_03_900': 909.56, 'CH_04_900': 911.72, 'CH_05_900': 913.88,
//   'CH_06_900': 916.04, 'CH_07_900': 918.20, 'CH_08_900': 920.36,
//   'CH_09_900': 922.52, 'CH_10_900': 924.68, 'CH_11_900': 926.84, 'CH_12_900': 915,
//
//   'CH_10_868': 865.20, 'CH_11_868': 865.50, 'CH_12_868': 865.80,
//   'CH_13_868': 866.10, 'CH_14_868': 866.40, 'CH_15_868': 866.70,
//   'CH_16_868': 867   , 'CH_17_868': 868   ,   
//   }
//
//CodingRates = {"4_5": CODING_RATE.CR4_5,  "4_6": CODING_RATE.CR4_6,
//               "4_7": CODING_RATE.CR4_7,  "4_8": CODING_RATE.CR4_8 }


#![deny(unsafe_code)]
#![no_main]
#![no_std]

const FREQUENCY: i64 = 915;

#[cfg(debug_assertions)]
extern crate panic_semihosting;

#[cfg(not(debug_assertions))]
extern crate panic_halt;

//use cortex_m::asm;

//use cortex_m::singleton;
//or ?
use heapless::{consts, Vec};

use cortex_m_rt::entry;
//use core::fmt::Write;  // for writeln
use cortex_m_semihosting::hprintln;
//use core::str;
//use core::ascii;
use nb::block;

//use eg_stm_hal::to_str;

use sx127x_lora;


#[cfg(feature = "stm32f1xx")]  //  eg blue pill stm32f103
use stm32f1xx_hal::{prelude::*,   
                    pac::Peripherals, 
                    serial::{Config, Serial, Tx, Rx},  //, StopBits
		    device::{USART3},  
                    spi::{Spi, Spi1NoRemap},
                    delay::Delay,
		    gpio::{gpioa::{PA5, PA6, PA7}, Alternate, Input, Floating,  
                           gpioa::{PA0, PA1}, Output, PushPull},
		    device::SPI1,
		    }; 

#[cfg(feature = "stm32f3xx")]  //  eg Discovery-stm32f303
use stm32f3xx_hal::{prelude::*, 
                    stm32::Peripherals,
                    serial::{ Serial, Tx, Rx},
		    stm32::{USART2}, 
                    spi::{Spi},
                    delay::Delay,
		    gpio::{gpioa::{PA5, PA6, PA7}, AF5,  
                           gpioa::{PA0, PA1}, Output, PushPull},
		    stm32::SPI1,
		    };

#[cfg(feature = "stm32f4xx")] // eg Nucleo-64  stm32f411
use stm32f4xx_hal::{prelude::*,  
                    pac::Peripherals, 
                    serial::{config::Config, Serial, Tx, Rx},
		    pac::{USART2}, 
                    spi::{Spi},
                    delay::Delay,
		    gpio::{gpioa::{PA5, PA6, PA7}, Alternate, AF5,  
                           gpioa::{PA0, PA1}, Output, PushPull},
                    time::MegaHertz,
		    pac::SPI1,
		    };

#[cfg(feature = "stm32f7xx")] 
use stm32f7xx_hal::{prelude::*,  
                    pac::Peripherals, 
                    serial::{config::Config, Serial, Tx, Rx},
		    pac::{USART2}, 
                    spi::{Spi},
                    delay::Delay,
		    gpio::{gpioa::{PA5, PA6, PA7}, Alternate, AF5,  
                           gpioa::{PA0, PA1}, Output, PushPull},
                    time::MegaHertz,
		    pac::SPI1,
		    };

#[cfg(feature = "stm32h7xx")] 
use stm32h7xx_hal::{prelude::*,  
                    pac::Peripherals, 
                    serial::{Tx, Rx},
		    pac::{USART2}, 
                    spi::{Spi, Enabled},
                    delay::Delay,
		    gpio::{gpioa::{PA0, PA1}, Output, PushPull},
		    pac::SPI1,
		    };

#[cfg(feature = "stm32l0xx")] 
use stm32l0xx_hal::{prelude::*,  
                    pac::Peripherals, 
                    serial::{config::Config, Serial, Tx, Rx},
		    pac::{USART2}, 
                    spi::{Spi},
                    delay::Delay,
		    gpio::{gpioa::{PA5, PA6, PA7}, Alternate, AF5,  
                           gpioa::{PA0, PA1}, Output, PushPull},
                    time::MegaHertz,
		    pac::SPI1,
		    };

#[cfg(feature = "stm32l1xx") ] // eg  Discovery kit stm32l100 and Heltec lora_node STM32L151CCU6
use stm32l1xx_hal::{prelude::*, 
		    stm32::Peripherals, 
		    serial::{Config, Serial, Tx, Rx},
		    stm32::{USART2},
                    spi::{Spi},
                    delay::Delay,
		    gpio::{gpioa::{PA5, PA6, PA7},   
                           gpioa::{PA0, PA1}, Output, PushPull},
                    pac::SPI1,
		    };

#[cfg(feature = "stm32l4xx")] 
use stm32l4xx_hal::{prelude::*,  
                    pac::Peripherals, 
                    serial::{Config, Serial, Tx, Rx},
		    pac::{USART2}, 
                    spi::{Spi},
                    delay::Delay,
		    gpio::{gpioa::{PA5, PA6, PA7}, Alternate, AF5, Input, Floating,  
                           gpioa::{PA0, PA1}, Output, PushPull},
		    pac::SPI1,
		    };



#[entry]

fn main() -> ! {

    #[cfg(feature = "stm32f1xx")]
    fn setup() ->  (Tx<USART3>, Rx<USART3>,
                    sx127x_lora::LoRa<Spi<SPI1,  Spi1NoRemap,
                                          (PA5<Alternate<PushPull>>, 
		                           PA6<Input<Floating>>, 
			                   PA7<Alternate<PushPull>>), u8>,
                                      PA1<Output<PushPull>>,  
                                      PA0<Output<PushPull>>>, 
		    Delay )  {
        let cp = cortex_m::Peripherals::take().unwrap();
        let p = Peripherals::take().unwrap();
    	let mut rcc = p.RCC.constrain();  
	let clocks = rcc.cfgr.freeze(&mut p.FLASH.constrain().acr); 
        let mut afio = p.AFIO.constrain(&mut rcc.apb2);

    	let mut gpiob = p.GPIOB.split(&mut rcc.apb2);
        let (tx, rx) = Serial::usart3(
            p.USART3,
            (gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh),    //tx pb10  for GPS
             gpiob.pb11), 					     //rx pb11  for GPS
            &mut afio.mapr,
            Config::default() .baudrate(9_600.bps()), 
            clocks,
            &mut rcc.apb1,
        ).split();

    
       let mut gpioa = p.GPIOA.split(&mut rcc.apb2);
       let spi = Spi::spi1(
           p.SPI1,
           (gpioa.pa5.into_alternate_push_pull(&mut gpioa.crl),  //   sck   on PA5
            gpioa.pa6.into_floating_input(&mut gpioa.crl),       //   miso  on PA6
            gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl)   //   mosi  on PA7
            ),
    	   &mut afio.mapr,
           sx127x_lora::MODE,
           8.mhz(),
           clocks, 
           &mut rcc.apb2,
           );

       let mut delay = Delay::new(cp.SYST, clocks);

       let lora = sx127x_lora::LoRa::new(spi, 
                              gpioa.pa1.into_push_pull_output(&mut gpioa.crl), //  cs   on PA1
                              gpioa.pa0.into_push_pull_output(&mut gpioa.crl), // reset on PA0
                              FREQUENCY, 
                              &mut delay ).unwrap();                           // delay
			      // .expect("Failed to communicate with radio module!")


        (tx, rx,  lora,  delay)
	}



    #[cfg(feature = "stm32f3xx")]
    fn setup() ->  (Tx<USART2>, Rx<USART2>, 
                   sx127x_lora::LoRa<Spi<SPI1, (PA5<AF5>, PA6<AF5>, PA7<AF5>)>,
                                     PA1<Output<PushPull>>, 
                                     PA0<Output<PushPull>>> , 
                                     Delay) {

        let cp = cortex_m::Peripherals::take().unwrap();
        let p = Peripherals::take().unwrap();
    	let mut rcc = p.RCC.constrain();  
	let clocks  = rcc.cfgr.freeze(&mut p.FLASH.constrain().acr); 
        let mut gpioa = p.GPIOA.split(&mut rcc.ahb); 

        let (tx, rx) = Serial::usart2(
            p.USART2,
            (gpioa.pa2.into_af7(&mut gpioa.moder, &mut gpioa.afrl),    //tx pa2  for GPS
             gpioa.pa3.into_af7(&mut gpioa.moder, &mut gpioa.afrl)),   //rx pa3  for GPS
            9600.bps(),    // 115_200.bps(),
            clocks,
            &mut rcc.apb1,
            ).split();


       let spi = Spi::spi1(
           p.SPI1,
           (gpioa.pa5.into_af5(&mut gpioa.moder, &mut gpioa.afrl),                // sck   on PA5
            gpioa.pa6.into_af5(&mut gpioa.moder, &mut gpioa.afrl),                // miso  on PA6
            gpioa.pa7.into_af5(&mut gpioa.moder, &mut gpioa.afrl)                 // mosi  on PA7
            ),
           sx127x_lora::MODE,
           8.mhz(),
           clocks,
           &mut rcc.apb2,
           );

       let mut delay = Delay::new(cp.SYST, clocks);
     
       let lora = sx127x_lora::LoRa::new(spi, 
                          gpioa.pa1.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper), //  cs  on PA1
                          gpioa.pa0.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper), //reset on PA0
                          FREQUENCY, 
                          &mut delay ).unwrap();                                               // delay
			  // .expect("Failed to communicate with radio module!")

        (tx, rx,  lora,  delay)
	}


    #[cfg(feature = "stm32f4xx")]
    fn setup() ->  (Tx<USART2>, Rx<USART2>,
                    sx127x_lora::LoRa<Spi<SPI1, (PA5<Alternate<AF5>>, 
		                                 PA6<Alternate<AF5>>, 
						 PA7<Alternate<AF5>>)>,
                                     PA1<Output<PushPull>>, 
                                     PA0<Output<PushPull>>>, 
                                     Delay ) {

        let cp = cortex_m::Peripherals::take().unwrap();
        let p = Peripherals::take().unwrap();
        let clocks    =  p.RCC.constrain().cfgr.freeze();
        let gpioa = p.GPIOA.split();

        let (tx, rx) = Serial::usart2(
           p.USART2,
           (gpioa.pa2.into_alternate_af7(),            //tx pa2  for GPS
	    gpioa.pa3.into_alternate_af7()),           //rx pa3  for GPS
           Config::default() .baudrate(9600.bps()), 
           clocks,
           ).unwrap().split();

       let spi = Spi::spi1(
           p.SPI1,
           (gpioa.pa5.into_alternate_af5(),  // sck   on PA5
            gpioa.pa6.into_alternate_af5(),  // miso  on PA6
            gpioa.pa7.into_alternate_af5()   // mosi  on PA7
            ),
           sx127x_lora::MODE,
           MegaHertz(8).into(),
           clocks,
           );

       let mut delay = Delay::new(cp.SYST, clocks);

       let lora = sx127x_lora::LoRa::new(spi, 
                              gpioa.pa1.into_push_pull_output(),      //  cs   on PA1
                              gpioa.pa0.into_push_pull_output(),      // reset on PA0
                              FREQUENCY, 
                              &mut delay).unwrap();                        // delay


        (tx, rx,  lora,  delay)
	}


    #[cfg(feature = "stm32f7xx")]
    fn setup() ->  (Tx<USART2>, Rx<USART2>,
                    sx127x_lora::LoRa<Spi<SPI1, (PA5<Alternate<AF5>>, 
		                                 PA6<Alternate<AF5>>, 
						 PA7<Alternate<AF5>>), u8>,
                                     PA1<Output<PushPull>>, 
                                     PA0<Output<PushPull>>>, 
                                     Delay ) {

        let cp = cortex_m::Peripherals::take().unwrap();
        let p = Peripherals::take().unwrap();
        let clocks    =  p.RCC.constrain().cfgr.freeze();
        let gpioa = p.GPIOA.split();

        let (tx, rx) = Serial::usart2(
           p.USART2,
           (gpioa.pa2.into_alternate_af7(),            //tx pa2  for GPS
	    gpioa.pa3.into_alternate_af7()),           //rx pa3  for GPS
           Config::default() .baudrate(9600.bps()), 
           clocks,
           ).unwrap().split();

       let spi = Spi::spi1(
           p.SPI1,
           (gpioa.pa5.into_alternate_af5(),  // sck   on PA5
            gpioa.pa6.into_alternate_af5(),  // miso  on PA6
            gpioa.pa7.into_alternate_af5()   // mosi  on PA7
            ),
           sx127x_lora::MODE,
           MegaHertz(8).into(),
           clocks,
           );

       let mut delay = Delay::new(cp.SYST, clocks);

       let lora = sx127x_lora::LoRa::new(spi, 
                              gpioa.pa1.into_push_pull_output(),      //  cs   on PA1
                              gpioa.pa0.into_push_pull_output(),      // reset on PA0
                              FREQUENCY, 
                              &mut delay).unwrap();                        // delay


        (tx, rx,  lora,  delay)
	}


    #[cfg(feature = "stm32h7xx")]
    fn setup() ->  (Tx<USART2>, Rx<USART2>,
                    sx127x_lora::LoRa<Spi<SPI1, Enabled>,
                                      PA1<Output<PushPull>>, 
                                      PA0<Output<PushPull>>>, 
                    Delay) {

       let cp = cortex_m::Peripherals::take().unwrap();
       let p      = Peripherals::take().unwrap();
       let pwr    = p.PWR.constrain();
       let vos    = pwr.freeze();
       let rcc    = p.RCC.constrain();
       let ccdr   = rcc.sys_ck(160.mhz()).freeze(vos, &p.SYSCFG);
       let clocks = ccdr.clocks;

       let gpioa  = p.GPIOA.split(ccdr.peripheral.GPIOA);

       let (tx, rx) = p.USART2.serial((gpioa.pa2.into_alternate_af7(),  //tx pa2 for GPS rx
                                       gpioa.pa3.into_alternate_af7()), //rx pa3 for GPS tx
                                      9600.bps(), 
                                      ccdr.peripheral.USART2, 
                                      &clocks).unwrap().split();


       // following github.com/stm32-rs/stm32h7xx-hal/blob/master/examples/spi.rs
       let spi = p.SPI1.spi(
           (gpioa.pa5.into_alternate_af5(),  // sck   on PA5 
            gpioa.pa6.into_alternate_af5(),  // miso  on PA6 
            gpioa.pa7.into_alternate_af5()   // mosi  on PA7
            ),
           sx127x_lora::MODE,
           8.mhz(),
           ccdr.peripheral.SPI1,
           &clocks,
           );
 
       let mut delay = Delay::new(cp.SYST, clocks);

       let lora = sx127x_lora::LoRa::new(spi, 
                              gpioa.pa1.into_push_pull_output(),      //  cs   on PA1
                              gpioa.pa0.into_push_pull_output(),      // reset on PA0
                              FREQUENCY, 
                              &mut delay).unwrap();                   // delay


        (tx, rx,  lora,  delay)                                       // delay again
	}


    #[cfg(feature = "stm32l0xx")]
    fn setup() ->  (Tx<USART2>, Rx<USART2>,
                    sx127x_lora::LoRa<Spi<SPI1, (PA5<Alternate<AF5>>, 
		                                 PA6<Alternate<AF5>>, 
						 PA7<Alternate<AF5>>)>,
                                     PA1<Output<PushPull>>, 
                                     PA0<Output<PushPull>>>, 
                                     Delay ) {

        let cp = cortex_m::Peripherals::take().unwrap();
        let p = Peripherals::take().unwrap();
        let clocks    =  p.RCC.constrain().cfgr.freeze();
        let gpioa = p.GPIOA.split();

        let (tx, rx) = Serial::usart2(
           p.USART2,
           (gpioa.pa2.into_alternate_af7(),            //tx pa2  for GPS
	    gpioa.pa3.into_alternate_af7()),           //rx pa3  for GPS
           Config::default() .baudrate(9600.bps()), 
           clocks,
           ).unwrap().split();

       let spi = Spi::spi1(
           p.SPI1,
           (gpioa.pa5.into_alternate_af5(),  // sck   on PA5
            gpioa.pa6.into_alternate_af5(),  // miso  on PA6
            gpioa.pa7.into_alternate_af5()   // mosi  on PA7
            ),
           sx127x_lora::MODE,
           MegaHertz(8).into(),
           clocks,
           );

       let mut delay = Delay::new(cp.SYST, clocks);

       let lora = sx127x_lora::LoRa::new(spi, 
                              gpioa.pa1.into_push_pull_output(),      //  cs   on PA1
                              gpioa.pa0.into_push_pull_output(),      // reset on PA0
                              FREQUENCY, 
                              &mut delay).unwrap();                        // delay


        (tx, rx,  lora,  delay)
	}




    #[cfg(feature = "stm32l1xx")]
    fn setup() ->  (Tx<USART2>, Rx<USART2>,
                    
		   )  {
        let cp = cortex_m::Peripherals::take().unwrap();
        let p = Peripherals::take().unwrap();
	let clocks    =  p.RCC.constrain().cfgr.freeze();
        let gpioa = p.GPIOA.split();

        let (tx2, rx2) = Serial::usart2(
            p.USART2,
            (gpioa.pa2.into_push_pull_output(),           //tx pa2  for GPS
	     gpioa.pa3.into_push_pull_output()),          //rx pa3  for GPS
            Config::default() .baudrate(9600.bps()), 
            clocks,
            ).unwrap().split();


       let spi = Spi::spi1(
           p.SPI1,
           (gpioa.pa5.into_push_pull_output(),  // sck   on PA5
            gpioa.pa6.into_push_pull_output(),  // miso  on PA6
            gpioa.pa7.into_push_pull_output()   // mosi  on PA7
            ),
           sx127x_lora::MODE,
           8.mhz(),
           clocks,
           );

       let mut delay = Delay::new(cp.SYST, clocks);

 //CHECK THE PINS ON THIS
       let lora = sx127x_lora::LoRa::new(spi, 
                              gpioa.pa1.into_push_pull_output(),      //  cs   on PA1
                              gpioa.pa0.into_push_pull_output(),      // reset on PA0
                              FREQUENCY, 
                              delay).unwrap();                        // delay


        (tx3, rx3,  lora,  delay)
	}


    #[cfg(feature = "stm32l4xx")]
    fn setup() ->  (Tx<USART2>, Rx<USART2>,
                    sx127x_lora::LoRa<Spi<SPI1, (PA5<Alternate<AF5, Input<Floating>>>, 
		                                 PA6<Alternate<AF5, Input<Floating>>>, 
						 PA7<Alternate<AF5, Input<Floating>>>)>,
                                     PA1<Output<PushPull>>, 
                                     PA0<Output<PushPull>>>, 
                                     Delay ) {

       let cp = cortex_m::Peripherals::take().unwrap();
       let p = Peripherals::take().unwrap();
       let mut flash = p.FLASH.constrain();
       let mut rcc   = p.RCC.constrain();
       let mut pwr   = p.PWR.constrain(&mut rcc.apb1r1);
       let clocks    = rcc.cfgr .sysclk(80.mhz()) .pclk1(80.mhz()) 
                                .pclk2(80.mhz()) .freeze(&mut flash.acr, &mut pwr);
      
       let mut gpioa = p.GPIOA.split(&mut rcc.ahb2);

       let (tx, rx) = Serial::usart2(
          p.USART2,
          (gpioa.pa2.into_af7(&mut gpioa.moder, &mut gpioa.afrl),            //tx pa2  for GPS
           gpioa.pa3.into_af7(&mut gpioa.moder, &mut gpioa.afrl)),           //rx pa3  for GPS
          Config::default() .baudrate(9600.bps()), 
          clocks,
          &mut rcc.apb1r1,
          ).split();

       let spi = Spi::spi1(
          p.SPI1,
          (gpioa.pa5.into_af5(&mut gpioa.moder, &mut gpioa.afrl),  // sck   on PA5
           gpioa.pa6.into_af5(&mut gpioa.moder, &mut gpioa.afrl),  // miso  on PA6
           gpioa.pa7.into_af5(&mut gpioa.moder, &mut gpioa.afrl)   // mosi  on PA7
           ),
          sx127x_lora::MODE,
          8.mhz(),
          clocks,
          &mut rcc.apb2,
          );

       let mut delay = Delay::new(cp.SYST, clocks);

       let lora = sx127x_lora::LoRa::new(spi, 
                             gpioa.pa1.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper),      //  cs   on PA1
                             gpioa.pa0.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper),      // reset on PA0
                             FREQUENCY, 
                             &mut delay).unwrap();                        // delay


       (tx, rx,  lora,  delay)
       }


    // End of hal/MCU specific setup. Following should be generic code.

    let (mut _tx_gps, mut rx_gps,   mut lora,   _delay) = setup();  //  GPS, lora, delay

    lora.set_tx_power(17,1).unwrap(); //Using PA_BOOST. See your board for correct pin.
    
    // byte buffer length 80
    let mut buffer: Vec<u8, consts::U80> = Vec::new();
    let mut buffer2 = [0;255];   //lora.transmit_payload() WANTS THIS SIZE, much bigger than 80 needed!

    //hprintln!("buffer at {} of {}", buffer.len(), buffer.capacity()).unwrap();  //0 of 80
    buffer.clear();

    //hprintln!("going into write/read loop ^C to exit ...").unwrap();

    let e: u8 = 9;
    let mut good = false;
    let mut size: usize;
    
    loop {
        let byte = match block!(rx_gps.read()) {
	    Ok(byt)	  => byt,
	    Err(_error) => e,
	    };
        if   byte == 36  {  //  $ is 36. start of a line
	   buffer.clear();
	   good = true;     //start capturing line
	   };
	if good {
	   if buffer.push(byte).is_err() ||  byte == 13  { //transmit if end of line. \r is 13, \n is 10
              
	      size = buffer.len();                         //packet size
              hprintln!("read buffer {} of {}", size, buffer.capacity()).unwrap();
              //hprintln!("read buffer {:?}", buffer).unwrap();
              
              // seems this should be unnecessary, but ...
	      for i in 0..size    {  //.chars().enumerate() {
                 buffer2[i] = buffer[i] as u8;
                 }
	      
	      //hprintln!("transmit buffer2 {:?}", to_str(&buffer2)).unwrap();
	      
	      let transmit = lora.transmit_payload(buffer2, size);
              match transmit {
    	          //Ok(_v)   => hprintln!("Sent packet: {:?}", buffer2).unwrap(),
    	          Ok(v) => hprintln!("return code: {:?} for size  {}", v, size).unwrap(),
    	          Err(_error) => hprintln!("Error sending packet").unwrap(),
                  };
              buffer.clear();
	      good = false;
	      };
	   };
	}
}
