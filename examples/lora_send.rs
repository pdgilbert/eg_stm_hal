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
//use cortex_m::asm;  //for breakpoint

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

#[cfg(feature = "stm32f7xx")] 
use stm32f7xx_hal::{prelude::*,  
                    pac::Peripherals, 
                    spi::{Spi, Pins, Enabled, ClockDivider, },
                    delay::Delay,
		    gpio::{gpioa::{PA0, PA1}, Output, PushPull},
		    pac::SPI1,
		    }; 

#[cfg(feature = "stm32h7xx")] 
use stm32h7xx_hal::{prelude::*,  
                    pac::Peripherals, 
                    spi::{Spi, Enabled},
                    delay::Delay,
		    gpio::{   //gpioa::{PA5, PA6, PA7}, Alternate, AF5,  really!
                           gpioa::{PA0, PA1}, Output, PushPull},
		    pac::SPI1,
		    }; 

#[cfg(feature = "stm32l0xx")] 
use stm32l0xx_hal::{prelude::*,  
                    pac::Peripherals, 
		    rcc,   // for ::Config but note name conflict with serial
                    spi::{Spi, Pins, },
                    delay::Delay,
		    gpio::{gpioa::{PA0, PA1}, Output, PushPull},
                    pac::SPI1,
		    }; 


#[cfg(feature = "stm32l1xx") ] // eg  Discovery kit stm32l100 and Heltec lora_node STM32L151CCU6
use stm32l1xx_hal::{prelude::*, 
                    stm32::Peripherals, 
		    rcc,   // for ::Config but note name conflict with next
                    spi::{Spi, Pins},
                    delay::Delay,
		    gpio::{//gpioa::{PA5, PA6, PA7}, Input,  Floating,   
                           gpioa::{PA0, PA1}, Output, PushPull},
                    stm32::SPI1,
		    };


#[cfg(feature = "stm32l4xx")]
use stm32l4xx_hal::{prelude::*,  
                    pac::Peripherals, 
                    spi::{Spi},
                    delay::Delay,
		    gpio::{gpioa::{PA5, PA6, PA7}, Alternate, AF5, Input, Floating, 
                           gpioa::{PA0, PA1}, Output, PushPull},
		    pac::SPI1,
		    }; 


//#![feature(extern_crate_item_prelude)]
//extern crate sx127x_lora;
//extern crate linux_embedded_hal as hal;

//use hal::spidev::{self, SpidevOptions};
//use hal::{Pin, Spidev};
//use hal::sysfs_gpio::Direction;
//use hal::Delay;
//
//const LORA_CS_PIN: u64 = 8;
//const LORA_RESET_PIN: u64 = 21;

const FREQUENCY: i64 = 915;  // needs decimal not hz not Mhz

#[entry]
fn main() -> !{
    //    let mut spi = Spidev::open("/dev/spidev0.0").unwrap();
    //    let options = SpidevOptions::new()
    //        .bits_per_word(8)
    //        .max_speed_hz(20_000)
    //        .mode(spidev::SPI_MODE_0)
    //        .build();
    //    spi.configure(&options).unwrap();
    //
    //    let cs = Pin::new(LORA_CS_PIN);
    //    cs.export().unwrap();
    //    cs.set_direction(Direction::Out).unwrap();
    //
    //    let reset = Pin::new(LORA_RESET_PIN);
    //    reset.export().unwrap();
    //    reset.set_direction(Direction::Out).unwrap();

    #[cfg(feature = "stm32f1xx")]
    fn setup() ->  (sx127x_lora::LoRa<Spi<SPI1,  Spi1NoRemap, (PA5<Alternate<PushPull>>, 
                                          PA6<Input<Floating>>, PA7<Alternate<PushPull>>), u8>,
                                      PA1<Output<PushPull>>, PA0<Output<PushPull>>>, 
                    Delay) {

       let cp = cortex_m::Peripherals::take().unwrap();
       let p  = Peripherals::take().unwrap();

       let mut rcc   = p.RCC.constrain();
       let clocks = rcc.cfgr.sysclk(64.mhz()).pclk1(32.mhz()).freeze(&mut p.FLASH.constrain().acr);
       
       let mut afio = p.AFIO.constrain(&mut rcc.apb2);
       let mut gpioa = p.GPIOA.split(&mut rcc.apb2);
       //let mut gpiob = p.GPIOB.split(&mut rcc.apb2);
    
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
                              gpioa.pa1.into_push_pull_output(&mut gpioa.crl),     //  cs   on PA1
                              gpioa.pa0.into_push_pull_output(&mut gpioa.crl),     // reset on PA0
                              FREQUENCY, 
                              &mut delay );                                                // delay
			      // .expect("Failed to communicate with radio module!")
       
       let lora =  lora.unwrap();

       //let mut lora =  match lora {
    	//  Ok(v)   => v,
    	//  Err(error) => {hprintln!("Setup Error: {:?}", error);
	//                 asm::bkpt();
	//                 //panic();
	//                 }
        //  };

       (lora, delay )                                                               // delay again
       };


    #[cfg(feature = "stm32f3xx")]
    fn setup() ->  (sx127x_lora::LoRa<Spi<SPI1, (PA5<AF5>, PA6<AF5>, PA7<AF5>)>,
                                      PA1<Output<PushPull>>, 
                                      PA0<Output<PushPull>>>, 
                    Delay) {
       
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
       
       let mut delay = Delay::new(cp.SYST, clocks);
       
       let lora = sx127x_lora::LoRa::new(spi, 
                          gpioa.pa1.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper), //  cs  on PA1
                          gpioa.pa0.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper), //reset on PA0
                          FREQUENCY, 
                          &mut delay ).unwrap();                            // delay
       
       (lora, delay )                                                       // delay again
       };


    #[cfg(feature = "stm32f4xx")]
    fn setup() ->  (sx127x_lora::LoRa<Spi<SPI1, (PA5<Alternate<AF5>>, PA6<Alternate<AF5>>, PA7<Alternate<AF5>>)>,
                                      PA1<Output<PushPull>>, 
                                      PA0<Output<PushPull>>>, 
                    Delay) {

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
       
       let mut delay = Delay::new(cp.SYST, clocks);
       
       let lora = sx127x_lora::LoRa::new(spi, 
                              gpioa.pa1.into_push_pull_output(),     //  cs   on PA1
                              gpioa.pa0.into_push_pull_output(),     // reset on PA0
                              FREQUENCY, 
                              &mut delay ).unwrap();                 // delay
       
       (lora, delay )                                                // delay again
       };


    // "stm32f7xx" can be done with
    //fn setup() ->  (sx127x_lora::LoRa<Spi<SPI1, (PA5<Alternate<AF5>>, 
    //                                             PA6<Alternate<AF5>>, 
    //                                             PA7<Alternate<AF5>>), Enabled<u8>>,
    //                                  PA1<Output<PushPull>>, 
    //                                  PA0<Output<PushPull>>>, 
    //                Delay) {
    //   needs also    gpio::{gpioa::{PA5, PA6, PA7}, Alternate, AF5,

    #[cfg(feature = "stm32f7xx")]
    fn setup() ->  (sx127x_lora::LoRa<Spi<SPI1, impl Pins<SPI1>, Enabled<u8>>,
                                      PA1<Output<PushPull>>, 
                                      PA0<Output<PushPull>>>, 
                    Delay) {

       let cp = cortex_m::Peripherals::take().unwrap();
       let p  = Peripherals::take().unwrap();

       let mut rcc   = p.RCC.constrain();
       
       let gpioa = p.GPIOA.split();

       let sck  = gpioa.pa5.into_alternate_af5();  // sck   on PA5
       let miso = gpioa.pa6.into_alternate_af5();  // miso  on PA6
       let mosi = gpioa.pa7.into_alternate_af5();  // mosi  on PA7

       //   somewhere 8.mhz needs to be set in spi

       let spi = Spi::new(p.SPI1, (sck, miso, mosi)).enable::<u8>(
          &mut rcc,
          ClockDivider::DIV32,
          sx127x_lora::MODE,
          );

       let clocks = rcc.cfgr.sysclk(64.mhz()).pclk1(32.mhz()).freeze();
       let mut delay = Delay::new(cp.SYST, clocks);
       
       let lora = sx127x_lora::LoRa::new(spi, 
                              gpioa.pa1.into_push_pull_output(),     //  cs   on PA1
                              gpioa.pa0.into_push_pull_output(),     // reset on PA0
                              FREQUENCY, 
                              &mut delay ).unwrap();                 // delay
       
       (lora, delay )                                                // delay again
       };


    #[cfg(feature = "stm32h7xx")]
    fn setup() ->  (sx127x_lora::LoRa<Spi<SPI1, Enabled>,
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

       // this might work too but Spi::sspi1(  giving multiple `spi1` found
       //let spi = Spi::spi1(
       //    p.SPI1,
       //    (gpioa.pa5.into_alternate_af5(),  // sck   on PA5
       //     gpioa.pa6.into_alternate_af5(),  // miso  on PA6
       //     gpioa.pa7.into_alternate_af5()   // mosi  on PA7
       //     ),
       //    sx127x_lora::MODE,
       //    8.mhz(),
       //    ccdr.peripheral.SPI1,
       //    &clocks,
       //    );
       
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
                              gpioa.pa1.into_push_pull_output(),     //  cs   on PA1
                              gpioa.pa0.into_push_pull_output(),     // reset on PA0
                              FREQUENCY, 
                              &mut delay ).unwrap();                 // delay
       
       (lora, delay )                                                // delay again
       };


    #[cfg(feature = "stm32l0xx")]
    fn setup() ->  (sx127x_lora::LoRa<Spi<SPI1, impl Pins<SPI1>>,
                                      PA1<Output<PushPull>>, 
                                      PA0<Output<PushPull>>>, 
                    Delay) {

       let cp = cortex_m::Peripherals::take().unwrap();
       let p         = Peripherals::take().unwrap();
       let mut rcc   = p.RCC.freeze(rcc::Config::hsi16());
       let gpioa   = p.GPIOA.split(&mut rcc);
  
       // following  github.com/stm32-rs/stm32l0xx-hal/blob/master/examples/spi.rs
       let spi = p.SPI1.spi(
                        (gpioa.pa5,   // sck   on PA5
                         gpioa.pa6,   // miso  on PA6
                         gpioa.pa7    // mosi  on PA7
                         ), 
                        sx127x_lora::MODE,
                        8.mhz(),
                        &mut rcc
                        );
             
       let mut delay = cp.SYST.delay(rcc.clocks);

       let lora = sx127x_lora::LoRa::new(spi, 
                              gpioa.pa1.into_push_pull_output(),     //  cs   on PA1
                              gpioa.pa0.into_push_pull_output(),     // reset on PA0
                              FREQUENCY, 
                              &mut delay ).unwrap();                 // delay
       
       (lora, delay )                                                // delay again
       };




    #[cfg(feature = "stm32l1xx")]
    fn setup() ->  (sx127x_lora::LoRa<Spi<SPI1, impl Pins<SPI1>>,
                                      PA1<Output<PushPull>>, 
                                      PA0<Output<PushPull>>>, 
                    Delay) {

       // instead of impl Pins<SPI1>  above could use 
       // Spi<SPI1, (PA5<Input<Floating>>,  PA6<Input<Floating>>, PA7<Input<Floating>>)>
       // which also requires  gpio::{gpioa::{PA5, PA6, PA7}, Input,  Floating, 
       // Possibly should also be able to use  'impl SpiExt<SPI1>' but no luck yet.

       let cp = cortex_m::Peripherals::take().unwrap();
       let p         = Peripherals::take().unwrap();
       let mut rcc   = p.RCC.freeze(rcc::Config::hsi());

       let gpioa = p.GPIOA.split();

       let spi = p.SPI1.spi(
                          (gpioa.pa5,            // sck   on PA5 
                           gpioa.pa6,            // miso  on PA6 
                           gpioa.pa7             // mosi  on PA7
                           ), 
                          sx127x_lora::MODE, 
                          8.mhz(), 
                          &mut rcc
                          );
        
                     
       let mut delay = cp.SYST.delay(rcc.clocks);
       
       let lora = sx127x_lora::LoRa::new(spi, 
                              gpioa.pa1.into_push_pull_output(),     //  cs   on PA1
                              gpioa.pa0.into_push_pull_output(),     // reset on PA0
                              FREQUENCY, 
                              &mut delay ).unwrap();                 // delay
       
       (lora, delay )                                                
       };



    #[cfg(feature = "stm32l4xx")]
    fn setup() ->  (sx127x_lora::LoRa<Spi<SPI1, (PA5<Alternate<AF5, Input<Floating>>>, 
                                                 PA6<Alternate<AF5, Input<Floating>>>, 
                                                 PA7<Alternate<AF5, Input<Floating>>>
                                                 )>,
                                      PA1<Output<PushPull>>, 
                                      PA0<Output<PushPull>>>, 
                    Delay) {

       let cp        = cortex_m::Peripherals::take().unwrap();
       let p         = Peripherals::take().unwrap();
       let mut flash = p.FLASH.constrain();
       let mut rcc   = p.RCC.constrain();
       let mut pwr   = p.PWR.constrain(&mut rcc.apb1r1);
       let clocks    = rcc.cfgr .sysclk(80.mhz()) .pclk1(80.mhz()) 
                                .pclk2(80.mhz()) .freeze(&mut flash.acr, &mut pwr);
      
       let mut gpioa = p.GPIOA.split(&mut rcc.ahb2);

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
                              gpioa.pa1.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper),  //  cs   on PA1
                              gpioa.pa0.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper),  // reset on PA0
                              FREQUENCY, 
                              &mut delay ).unwrap();                 // delay
       
       (lora, delay )                                                // delay again
       };


    // End of hal/MCU specific setup. Following should be generic code.


    let (mut lora, _delay) =  setup();

    lora.set_mode(sx127x_lora::RadioMode::Stdby).unwrap();
    lora.set_signal_bandwidth(125_000).unwrap();
    lora.set_coding_rate_4(5).unwrap();
    lora.set_spreading_factor(7).unwrap();
    lora.set_invert_iq(false).unwrap();
    lora.set_tx_power(17,1).unwrap();    //Using PA_BOOST. See your board for correct pin.

    //hprintln!("mode             {}", lora.get_mode()).unwrap();
    //hprintln!("mode             {}", lora.read_register(Register::RegOpMode.addr())).unwrap();
    hprintln!("bandwidth        {:?}", lora.get_signal_bandwidth()).unwrap();
    //hprintln!("coding_rate      {:?}",  lora.get_coding_rate_4()).unwrap();
    hprintln!("spreading_factor {:?}",  lora.get_spreading_factor()).unwrap();
    //hprintln!("invert_iq        {:?}",  lora.get_invert_iq()).unwrap();
    //hprintln!("tx_power         {:?}",  lora.get_tx_power()).unwrap();


    let message = "Hello, world!";
    let mut buffer = [0;255];
    for (i,c) in message.chars().enumerate() {
        buffer[i] = c as u8;
    }

    let transmit = lora.transmit_payload(buffer, message.len());
    match transmit {
    	Ok(_size)   => hprintln!("Sent packet: {}", message).unwrap(),
    	//Ok(size) => hprintln!("Sent packet with size: {}", size).unwrap(),
    	Err(_error) => hprintln!("Error").unwrap(),
    };

    let mut j : u8  = 0;
    loop { 
       j += 1;
       let message = "message " ;
       for (i,c) in message.chars().enumerate() { buffer[i] = c as u8; }
       buffer[1 + message.len()] = j ;
       
       let transmit = lora.transmit_payload(buffer, message.len());
       match transmit {
           Ok(_size)   => hprintln!("Sent packet: {} {}", message, j).unwrap(),
           //Ok(size) => hprintln!("Sent packet with size: {}", size).unwrap(),
           Err(_error) => hprintln!("Error").unwrap(),
       };
          };
}
