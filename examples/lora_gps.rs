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

#[cfg(feature = "stm32l1xx") ] // eg  Discovery kit stm32l100 and Heltec lora_node STM32L151CCU6
use stm32l1xx_hal::{prelude::*, 
		    stm32::Peripherals, 
		    serial::{Config, Serial, Tx, Rx},
		    stm32::{USART2},
                    spi::{Spi},
                    delay::Delay,
		    gpio::{gpioa::{PA5, PA6, PA7}, Alternate, AF5,  
                           gpioa::{PA0, PA1}, Output, PushPull},
                    time::MegaHertz,
		    pac::SPI1,
		    };



#[entry]

fn main() -> ! {

    #[cfg(feature = "stm32f1xx")]
    fn setup() ->  (Tx<USART3>, Rx<USART3>,
                    sx127x_lora::LoRa<Spi<SPI1,  Spi1NoRemap,
                                          (PA5<Alternate<PushPull>>, 
		                           PA6<Input<Floating>>, 
			                   PA7<Alternate<PushPull>>)>,
                                  PA1<Output<PushPull>>,  PA0<Output<PushPull>>, 
		                  Delay> )  {
        let cp = cortex_m::Peripherals::take().unwrap();
        let p = Peripherals::take().unwrap();
    	let mut rcc = p.RCC.constrain();  
	let clocks = rcc.cfgr.freeze(&mut p.FLASH.constrain().acr); 
        let mut afio = p.AFIO.constrain(&mut rcc.apb2);

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

     
       let lora = sx127x_lora::LoRa::new(spi, 
                              gpioa.pa1.into_push_pull_output(&mut gpioa.crl), //  cs   on PA1
                              gpioa.pa0.into_push_pull_output(&mut gpioa.crl), // reset on PA0
                              FREQUENCY, 
                              Delay::new(cp.SYST, clocks) ).unwrap();          // delay


        (tx3, rx3,   lora)
	}



    #[cfg(feature = "stm32f3xx")]
    fn setup() ->  (Tx<USART2>, Rx<USART2>, 
                   sx127x_lora::LoRa<Spi<SPI1, (PA5<AF5>, PA6<AF5>, PA7<AF5>)>,
                                     PA1<Output<PushPull>>, 
                                     PA0<Output<PushPull>>, 
                                     Delay> ) {

        let cp = cortex_m::Peripherals::take().unwrap();
        let p = Peripherals::take().unwrap();
    	let mut rcc = p.RCC.constrain();  
	let clocks  = rcc.cfgr.freeze(&mut p.FLASH.constrain().acr); 
        let mut gpioa = p.GPIOA.split(&mut rcc.ahb); 

        let (tx2, rx2) = Serial::usart2(
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
       
       let lora = sx127x_lora::LoRa::new(spi, 
                          gpioa.pa1.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper), //  cs  on PA1
                          gpioa.pa0.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper), //reset on PA0
                          FREQUENCY, 
                          Delay::new(cp.SYST, clocks) ).unwrap();                               // delay
 

        (tx2, rx2,   lora)
	}




    #[cfg(feature = "stm32f4xx")]
    fn setup() ->  (Tx<USART2>, Rx<USART2>,
                    sx127x_lora::LoRa<Spi<SPI1, (PA5<Alternate<AF5>>, 
		                                 PA6<Alternate<AF5>>, 
						 PA7<Alternate<AF5>>)>,
                                     PA1<Output<PushPull>>, 
                                     PA0<Output<PushPull>>, 
                                     Delay> ) {

        let cp = cortex_m::Peripherals::take().unwrap();
        let p = Peripherals::take().unwrap();
        let clocks    =  p.RCC.constrain().cfgr.freeze();
        let gpioa = p.GPIOA.split();

        let (tx2, rx2) = Serial::usart2(
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

       let lora = sx127x_lora::LoRa::new(spi, 
                              gpioa.pa1.into_push_pull_output(),      //  cs   on PA1
                              gpioa.pa0.into_push_pull_output(),      // reset on PA0
                              FREQUENCY, 
                              Delay::new(cp.SYST, clocks) ).unwrap(); // delay


        (tx2, rx2,   lora)
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
            (gpioa.pa2.into_alternate_af7(),           //tx pa2  for GPS
	     gpioa.pa3.into_alternate_af7()),          //rx pa3  for GPS
            Config::default() .baudrate(9600.bps()), 
            clocks,
            ).unwrap().split();

        let lora = ();
 
        (tx2, rx2,   lora)
	}


    // End of hal/MCU specific setup. Following should be generic code.

    let (mut _tx_gps, mut rx_gps,   mut lora) = setup();  //  GPS, lora

    lora.set_tx_power(17,1).unwrap(); //Using PA_BOOST. See your board for correct pin.
    
    // byte buffer length 80
    let mut buffer: Vec<u8, consts::U80> = Vec::new();
    let mut buffer2 = [0;255];   //lora.transmit_payload() WANTS THIS SIZE, much bigger than 80 needed!

    //hprintln!("buffer at {} of {}", buffer.len(), buffer.capacity()).unwrap();  //0 of 80
    buffer.clear();

    //hprintln!("going into write/read loop ^C to exit ...").unwrap();

    let e: u8 = 9;
    let mut good = false;
    let mut size: usize = 0;
    
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
