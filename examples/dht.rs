#![deny(unsafe_code)]
#![no_main]
#![no_std]


#[cfg(debug_assertions)]
extern crate panic_semihosting;

#[cfg(not(debug_assertions))]
extern crate panic_halt;

//use cortex_m;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

//https://github.com/michaelbeaumont/dht-sensor
use dht_sensor::*;

//use crate::hal::{delay, gpio, prelude::*, stm32};

//use embedded_hal::blocking::delay::{DelayMs,};


// setup() does all  hal/MCU specific setup and returns generic hal device for use in main code.

#[cfg(feature = "stm32f0xx")]
use stm32f0xx_hal::{prelude::*, 
                    pac::{Peripherals, CorePeripherals}, 
    	            delay::Delay,
		    gpio::{gpioa::PA8, OpenDrain,  Output, },
		    };

    // open_drain_output is really input and output

    #[cfg(feature = "stm32f0xx")]
    fn setup() -> (PA8<Output<OpenDrain>>,  Delay) {
      
       let cp      = CorePeripherals::take().unwrap();
       let mut p   = Peripherals::take().unwrap();
       let mut rcc = p.RCC.configure().freeze(&mut p.FLASH);
      
       let gpioa  = p.GPIOA.split(&mut rcc);

       let pin_a8 = cortex_m::interrupt::free(move |cs| 
                   gpioa.pa8.into_open_drain_output(cs) );

       let mut delay = Delay::new(cp.SYST, &rcc);

       //  1 second delay (for DHT11 setup?) Wait on  sensor initialization?
       delay.delay_ms(1000_u16);
      
       (pin_a8,                   //DHT data will be on A8
        delay)
       }

#[cfg(feature = "stm32f1xx")]
use stm32f1xx_hal::{prelude::*, 
                    pac::{Peripherals, CorePeripherals}, 
    	            delay::Delay,
		    gpio::{gpioa::PA8, OpenDrain,  Output, },
		    };

    // open_drain_output is really input and output

    #[cfg(feature = "stm32f1xx")]
    fn setup() -> (PA8<Output<OpenDrain>>,  Delay) {
      
       let cp = CorePeripherals::take().unwrap();
       let  p = Peripherals::take().unwrap();

       let mut rcc = p.RCC.constrain();
       let clocks = rcc.cfgr.freeze(&mut p.FLASH.constrain().acr);
       
       // delay is used by `dht-sensor` to wait for signals
       let mut delay = Delay::new(cp.SYST, clocks);   //SysTick: System Timer

       let mut gpioa = p.GPIOA.split(&mut rcc.apb2);
       let pin_a8    = gpioa.pa8.into_open_drain_output(&mut gpioa.crh); 
       //let mut pin_a8 = cortex_m::interrupt::free(|cs| pin_a8.into_open_drain_output(cs));
 
       //  1 second delay (for DHT11 setup?) Wait on  sensor initialization?
       delay.delay_ms(1000_u16);
      
       (pin_a8,                   //DHT data will be on A8
        delay)
       }


#[cfg(feature = "stm32f3xx")]
use stm32f3xx_hal::{prelude::*, 
                    stm32::{Peripherals, CorePeripherals}, 
		    delay::Delay ,
		    gpio::{gpioa::PA8, OpenDrain,  Output, },
		    };

    #[cfg(feature = "stm32f3xx")]
    fn setup() -> (PA8<Output<OpenDrain>>,  Delay) {
       
       let cp = CorePeripherals::take().unwrap();
       let  p = Peripherals::take().unwrap();

       let mut rcc   = p.RCC.constrain();
       let clocks    = rcc.cfgr.freeze(&mut p.FLASH.constrain().acr);
       let mut gpioa = p.GPIOA.split(&mut rcc.ahb);
       let pin_a8    = gpioa.pa8.into_open_drain_output(&mut gpioa.moder, &mut gpioa.otyper);
       
       // delay is used by `dht-sensor` to wait for signals
       let mut delay = Delay::new(cp.SYST, clocks);   //SysTick: System Timer

       //  1 second delay (for DHT11 setup?) Wait on  sensor initialization?
       delay.delay_ms(1000_u16);
       
       (pin_a8,                   //DHT data will be on A8
        delay)
       }


#[cfg(feature = "stm32f4xx")]
use stm32f4xx_hal::{prelude::*, 
                    pac::{Peripherals, CorePeripherals}, 
		    delay::Delay, 
		    gpio::{gpioa::PA8, OpenDrain,  Output, },
		    };

    #[cfg(feature = "stm32f4xx")]           // Use HSE oscillator
    fn setup() -> (PA8<Output<OpenDrain>>,  Delay) {
       
       let cp = CorePeripherals::take().unwrap();
       let  p = Peripherals::take().unwrap();

       //let clocks =  p.RCC.constrain().cfgr.freeze();
       // next gives panicked at 'assertion failed: !sysclk_on_pll || 
       //                  sysclk <= sysclk_max && sysclk >= sysclk_min'
       let clocks = p.RCC.constrain().cfgr.use_hse(8.mhz()).sysclk(168.mhz()).freeze();
       let pin_a8 = p.GPIOA.split().pa8.into_open_drain_output();  
              
       // delay is used by `dht-sensor` to wait for signals
       let mut delay = Delay::new(cp.SYST, clocks);   //SysTick: System Timer

       //  1 second delay (for DHT11 setup?) Wait on  sensor initialization?
       delay.delay_ms(1000_u16);

       (pin_a8,                   //DHT data will be on A8
        delay)
       }


#[cfg(feature = "stm32f7xx")]
use stm32f7xx_hal::{prelude::*, 
                    pac::{Peripherals, CorePeripherals}, 
		    delay::Delay, 
		    gpio::{gpioa::PA8, OpenDrain,  Output, },
		    };

    #[cfg(feature = "stm32f7xx")]           // Use HSE oscillator
    fn setup() -> (PA8<Output<OpenDrain>>,  Delay) {
       
       let cp = CorePeripherals::take().unwrap();
       let  p = Peripherals::take().unwrap();
       let clocks = p.RCC.constrain().cfgr.sysclk(216.mhz()).freeze();

       let pin_a8 = p.GPIOA.split().pa8.into_open_drain_output();  
              
       // delay is used by `dht-sensor` to wait for signals
       let mut delay = Delay::new(cp.SYST, clocks);   //SysTick: System Timer

       //  1 second delay (for DHT11 setup?) Wait on  sensor initialization?
       delay.delay_ms(1000_u16);

       (pin_a8,                   //DHT data will be on A8
        delay)
       }


#[cfg(feature = "stm32h7xx")]
use stm32h7xx_hal::{prelude::*, 
                    pac::{Peripherals, CorePeripherals}, 
		    delay::Delay, 
		    gpio::{gpioa::PA8, OpenDrain,  Output, },
		    };

    #[cfg(feature = "stm32h7xx")]  
    fn setup() -> (PA8<Output<OpenDrain>>,  Delay) {
       
       let cp = CorePeripherals::take().unwrap();
       let  p     = Peripherals::take().unwrap();
       let pwr    = p.PWR.constrain();
       let vos    = pwr.freeze();
       let rcc    = p.RCC.constrain();
       let ccdr   = rcc.sys_ck(160.mhz()).freeze(vos, &p.SYSCFG);
       let clocks = ccdr.clocks;

       let pin_a8 = p.GPIOA.split(ccdr.peripheral.GPIOA).pa8.into_open_drain_output();  
              
       // delay is used by `dht-sensor` to wait for signals
       let mut delay = Delay::new(cp.SYST, clocks);   //SysTick: System Timer

       //  1 second delay (for DHT11 setup?) Wait on  sensor initialization?
       delay.delay_ms(1000_u16);

       (pin_a8,                   //DHT data will be on A8
        delay)
       }


#[cfg(feature = "stm32l0xx")]
use stm32l0xx_hal::{prelude::*, 
                    pac::{Peripherals, CorePeripherals}, 
		    rcc,   // for ::Config but note name conflict with serial
		    delay::Delay, 
		    gpio::{gpioa::PA8, OpenDrain,  Output, },
		    };

    #[cfg(feature = "stm32l0xx")]      
    fn setup() -> (PA8<Output<OpenDrain>>,  Delay) {
       
       let cp  = CorePeripherals::take().unwrap();
       let  p      = Peripherals::take().unwrap();
       let mut rcc = p.RCC.freeze(rcc::Config::hsi16());

       //let clocks =  p.RCC.constrain().cfgr.freeze();
       // next gives panicked at 'assertion failed: !sysclk_on_pll || 
       //                  sysclk <= sysclk_max && sysclk >= sysclk_min'
       //let clocks = p.RCC.constrain().cfgr.use_hse(8.mhz()).sysclk(168.mhz()).freeze();
       let pin_a8 = p.GPIOA.split(&mut rcc).pa8.into_open_drain_output();  
              
       // delay is used by `dht-sensor` to wait for signals
       //let mut delay = Delay::new(cp.SYST, clocks);   //SysTick: System Timer
       let mut delay = cp.SYST.delay(rcc.clocks);

       //  1 second delay (for DHT11 setup?) Wait on  sensor initialization?
       delay.delay_ms(1000_u16);

       (pin_a8,                   //DHT data will be on A8
        delay)
       }


#[cfg(feature = "stm32l1xx")]
use stm32l1xx_hal::{prelude::*, 
                    stm32::{Peripherals, CorePeripherals}, 
		    rcc,   // for ::Config but note name conflict with next
		    delay::Delay ,
		    gpio::{gpioa::PA8, OpenDrain,  Output, },
		   };

    #[cfg(feature = "stm32l1xx")]   
    fn setup() -> (PA8<Output<OpenDrain>>,  Delay) {
       
       let cp  = CorePeripherals::take().unwrap();
       let  p  = Peripherals::take().unwrap();
       let rcc = p.RCC.freeze(rcc::Config::hsi());

       //let clocks = p.RCC.constrain().cfgr.use_hse(8.mhz()).sysclk(168.mhz()).freeze();
       let pin_a8 = p.GPIOA.split().pa8.into_open_drain_output();
           
       // delay is used by `dht-sensor` to wait for signals
       //let mut delay = Delay::new(cp.SYST, clocks);   //SysTick: System Timer
          
       let mut delay = cp.SYST.delay(rcc.clocks);

       //  1 second delay (for DHT11 setup?) Wait on  sensor initialization?
       delay.delay_ms(1000_u16);
   
       (pin_a8,  delay)                  //DHT data will be on A8
       }


#[cfg(feature = "stm32l4xx")]
use stm32l4xx_hal::{prelude::*, 
                    pac::{Peripherals, CorePeripherals}, 
		    delay::Delay, 
		    gpio::{gpioa::PA8, OpenDrain,  Output, },
		    };
//#[cfg(feature = "stm32l4xx")]
//use embedded_hal::digital::v2::{InputPin, OutputPin};

    #[cfg(feature = "stm32l4xx")]        
    fn setup() -> (PA8<Output<OpenDrain>>,  Delay) {
       
       let cp = CorePeripherals::take().unwrap();
       let  p = Peripherals::take().unwrap();
       let mut flash = p.FLASH.constrain();
       let mut rcc = p.RCC.constrain();
       let mut pwr = p.PWR.constrain(&mut rcc.apb1r1);
       let clocks = rcc.cfgr .sysclk(80.mhz()) .pclk1(80.mhz()) 
                             .pclk2(80.mhz()) .freeze(&mut flash.acr, &mut pwr);

       let gpioa   = p.GPIOA.split(&mut rcc.ahb2);
       let pin_a8  = gpioa.pa8.into_open_drain_output(&mut gpioa.moder, &mut gpioa.otyper);
       
       // delay is used by `dht-sensor` to wait for signals
       let mut delay = Delay::new(cp.SYST, clocks);   //SysTick: System Timer

       //  1 second delay (for DHT11 setup?) Wait on  sensor initialization?
       delay.delay_ms(1000_u16);

       (pin_a8, delay)                   //DHT data will be on A8
       }


// End of hal/MCU specific setup. Following should be generic code.


#[entry]
fn main() -> ! {
    let (mut pin_a8, mut delay) = setup();
    
    hprintln!("Reading sensor...").unwrap();
    
    let r = dht11::Reading::read(&mut delay, &mut pin_a8);

    match r {
        Ok(dht11::Reading {
            temperature,
            relative_humidity,
        }) => hprintln!("{} deg C, {}% RH", temperature, relative_humidity).unwrap(),
        Err(e) => hprintln!("Error {:?}", e).unwrap(),
    }
    hprintln!("empty looping").unwrap();

    loop {}
}
