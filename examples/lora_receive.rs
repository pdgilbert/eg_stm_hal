//   Using  sck, miso, mosi, cs, and reset.
//   See hardware sections below for pin setup.
//   Not yet using D00, D01, D02, D03

//DIO0  triggers RxDone/TxDone status.
//DIO1  triggers RxTimeout and other errors status.
//MOSI, MISO, SCLK for SPI communication. 
//NSS is the chip select (CS) signal. 
//REST is reset.

#![no_std]
#![no_main]

#[cfg(debug_assertions)]
extern crate panic_semihosting;

#[cfg(not(debug_assertions))]
extern crate panic_halt;

// use nb::block;
use cortex_m_rt::entry;
use cortex_m_semihosting::*;
//use asm_delay::{ AsmDelay, bitrate, };

use sx127x_lora;

#[cfg(feature = "stm32f1xx")]  //  eg blue pill stm32f103
use stm32f1xx_hal::{prelude::*,   
                    pac::Peripherals, 
                    spi::{Spi, Spi1NoRemap},
                    delay::Delay,
		    gpio::{gpioa::{PA5, PA6, PA7}, Alternate, Input, Floating,  
                           gpioa::{PA0, PA1}, Output, PushPull},
		    device::SPI1,
		    }; 

#[cfg(feature = "stm32f3xx")]  //  eg Discovery-stm32f303
use stm32f3xx_hal::{prelude::*, 
                    stm32::Peripherals,
                    spi::{Spi},
                    delay::Delay,
		    gpio::{gpioa::{PA5, PA6, PA7}, AF5,  
                           gpioa::{PA0, PA1}, Output, PushPull},
		    stm32::SPI1,
		    };

#[cfg(feature = "stm32f4xx")] // eg Nucleo-64  stm32f411
use stm32f4xx_hal::{prelude::*,  
                    pac::Peripherals, 
                    spi::{Spi},
                    delay::Delay,
		    gpio::{gpioa::{PA5, PA6, PA7}, Alternate, AF5,  
                           gpioa::{PA0, PA1}, Output, PushPull},
                    time::MegaHertz,
		    pac::SPI1,
		    }; 



const FREQUENCY: i64 = 915;

#[entry]
fn main() -> !{

    #[cfg(feature = "stm32f1xx")]
    fn setup() ->  sx127x_lora::LoRa< Spi<SPI1,  Spi1NoRemap,
                         (PA5<Alternate<PushPull>>, PA6<Input<Floating>>, PA7<Alternate<PushPull>>)>,
                      PA1<Output<PushPull>>,  PA0<Output<PushPull>>, Delay> {

       let cp = cortex_m::Peripherals::take().unwrap();
       let p  = Peripherals::take().unwrap();

       let mut rcc   = p.RCC.constrain();
       let clocks = rcc.cfgr.sysclk(64.mhz()).pclk1(32.mhz()).freeze(&mut p.FLASH.constrain().acr);
       
       let mut afio = p.AFIO.constrain(&mut rcc.apb2);
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

     
       // return LoRa object
       sx127x_lora::LoRa::new(spi, 
                              gpioa.pa1.into_push_pull_output(&mut gpioa.crl),     //  cs   on PA1
                              gpioa.pa0.into_push_pull_output(&mut gpioa.crl),     // reset on PA0
                              FREQUENCY, 
                              Delay::new(cp.SYST, clocks) ).unwrap()  // delay
       };

    #[cfg(feature = "stm32f3xx")]
    fn setup() ->  sx127x_lora::LoRa<Spi<SPI1, (PA5<AF5>, PA6<AF5>, PA7<AF5>)>,
                                     PA1<Output<PushPull>>, 
                                     PA0<Output<PushPull>>, 
                                     Delay> {
       
       let cp = cortex_m::Peripherals::take().unwrap();
       let p  = Peripherals::take().unwrap();

       let mut rcc   = p.RCC.constrain();
       let clocks = rcc.cfgr.sysclk(64.mhz()).pclk1(32.mhz()).freeze(&mut p.FLASH.constrain().acr);
       
       let mut gpioa = p.GPIOA.split(&mut rcc.ahb);
       //let mut gpiob = p.GPIOB.split(&mut rcc.ahb);

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
       
       // return LoRa object
       sx127x_lora::LoRa::new(spi, 
                          gpioa.pa1.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper), //  cs  on PA1
                          gpioa.pa0.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper), //reset on PA0
                          FREQUENCY, 
                          Delay::new(cp.SYST, clocks) ).unwrap()                                 // delay
       };


    #[cfg(feature = "stm32f4xx")]
    fn setup() ->  sx127x_lora::LoRa<Spi<SPI1, (PA5<Alternate<AF5>>, PA6<Alternate<AF5>>, PA7<Alternate<AF5>>)>,
                                     PA1<Output<PushPull>>, 
                                     PA0<Output<PushPull>>, 
                                     Delay> {

       let cp = cortex_m::Peripherals::take().unwrap();
       let p  = Peripherals::take().unwrap();

       let rcc   = p.RCC.constrain();
       let clocks = rcc.cfgr.sysclk(64.mhz()).pclk1(32.mhz()).freeze();
       
       let gpioa = p.GPIOA.split();
       //let gpiob = p.GPIOB.split();

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
       
       //let reset = gpiof.pf13.into_push_pull_output(&mut gpiof.moder, &mut gpiof.otyper);
       //let cs    = gpiod.pd14.into_push_pull_output(&mut gpiod.moder, &mut gpiod.otyper);
       
       // return LoRa object
       sx127x_lora::LoRa::new(spi, 
                              gpioa.pa1.into_push_pull_output(),     //  cs   on PA1
                              gpioa.pa0.into_push_pull_output(),     // reset on PA0
                              FREQUENCY, 
                              Delay::new(cp.SYST, clocks) ).unwrap()  // delay
       };



    // End of hal/MCU specific setup. Following should be generic code.


    let mut lora =  setup();
    
    loop {
        let poll = lora.poll_irq(Some(60)); //60 30 Second timeout
        match poll {
            Ok(size) =>{
                hprintln!("New Packet with size {} and RSSI: {}", size, lora.get_packet_rssi().unwrap()).unwrap();
                let buffer = lora.read_packet().unwrap(); // Received buffer. NOTE: 255 bytes are always returned
                hprint!("with Payload: ").unwrap();
                for i in 0..size{
                    hprint!("{}", buffer[i] as char).unwrap();
                }
                hprintln!().unwrap();
            },
            Err(_error) => hprintln!("Timeout").unwrap(),
        }
    }

}
