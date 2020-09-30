//  I2c::i2c1(  works with stm32f1xx, stm32f3xx, trait problem with stm32f4xx
    
//    ads1015-display.rs   builds with stm32f1xx, stm32f3xx, ...
//
//  See https://blog.eldruin.com/ads1x1x-analog-to-digital-converter-driver-in-rust/
//  and https://github.com/eldruin/driver-examples
//    for much more detailed description.
//  based on
//  https://github.com/eldruin/driver-examples/stm32f1-bluepill/examples/ads1015-adc-display-bp.rs
//  and https://github.com/eldruin/driver-examples/stm32f3-discovery/examples/ads1015-display-f3.rs

// Measure voltages with ADS1015 analog/digital converter and print to SSD1306 OLED display.

#![deny(unsafe_code)]
#![no_std]
#![no_main]

use ads1x1x::{channel as AdcChannel, Ads1x1x, FullScaleRange, SlaveAddr};

use core::fmt::Write;
use cortex_m_rt::entry;
use embedded_graphics::{fonts::Font6x8, prelude::*};


use nb::block;
use panic_semihosting as _;
use ssd1306::{prelude::*, Builder};


// setup() does all  hal/MCU specific setup and returns generic hal device for use in main code.

#[cfg(feature = "stm32f0xx")]  //  eg blue pill stm32f103
use stm32f0xx_hal::{prelude::*,   
                    pac::Peripherals, 
                    i2c::{BlockingI2c, DutyCycle, Mode},   
    	            delay::Delay,
		    gpio::{gpiob::{PB8, PB9}, Alternate, OpenDrain,  gpioc::PC13, Output, PushPull,},
		    device::I2C1,
		    }; 
#[cfg(feature = "stm32f0xx")]  //  eg blue pill stm32f103
use embedded_hal::digital::v2::OutputPin;

    #[cfg(feature = "stm32f0xx")]
    fn setup() ->  (BlockingI2c<I2C1,  (PB8<Alternate<OpenDrain>>, PB9<Alternate<OpenDrain>>) >,
                        PC13<Output<PushPull>>, Delay) {
       
       let cp = cortex_m::Peripherals::take().unwrap();
       let dp = Peripherals::take().unwrap();
       
       let mut rcc   = dp.RCC.constrain();
       let clocks    = rcc.cfgr.freeze(&mut dp.FLASH.constrain().acr);

       let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);  // for i2c scl and sda 

       let i2c = BlockingI2c::i2c1(  
   	   dp.I2C1,
   	   
	   (gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh),    // i2c scl on pb8
	    gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh)),   // i2c sda on pb9
   	   
	   &mut dp.AFIO.constrain(&mut rcc.apb2).mapr,

   	   Mode::Fast {
   	       frequency: 100_000.hz(),
   	       duty_cycle: DutyCycle::Ratio2to1,
   	       },
   	   clocks,
   	   &mut rcc.apb1,
   	   1000,
   	   10,
   	   1000,
   	   1000,
   	   );

       // led
       let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);
       let led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);   // led on pc13

       (i2c, led,  Delay::new(cp.SYST, clocks))   // return tuple (i2c, led, delay)
       }



#[cfg(feature = "stm32f1xx")]  //  eg blue pill stm32f103
use stm32f1xx_hal::{prelude::*,   
                    pac::Peripherals, 
                    i2c::{BlockingI2c, DutyCycle, Mode},   
    	            delay::Delay,
		    gpio::{gpiob::{PB8, PB9}, Alternate, OpenDrain,  gpioc::PC13, Output, PushPull,},
		    device::I2C1,
		    }; 
#[cfg(feature = "stm32f1xx")]  //  eg blue pill stm32f103
use embedded_hal::digital::v2::OutputPin;

    #[cfg(feature = "stm32f1xx")]
    fn setup() ->  (BlockingI2c<I2C1,  (PB8<Alternate<OpenDrain>>, PB9<Alternate<OpenDrain>>) >,
                        PC13<Output<PushPull>>, Delay) {
       
       let cp = cortex_m::Peripherals::take().unwrap();
       let dp = Peripherals::take().unwrap();
       
       let mut rcc   = dp.RCC.constrain();
       let clocks    = rcc.cfgr.freeze(&mut dp.FLASH.constrain().acr);

       let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);  // for i2c scl and sda 

       let i2c = BlockingI2c::i2c1(  
   	   dp.I2C1,
   	   
	   (gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh),    // i2c scl on pb8
	    gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh)),   // i2c sda on pb9
   	   
	   &mut dp.AFIO.constrain(&mut rcc.apb2).mapr,

   	   Mode::Fast {
   	       frequency: 100_000.hz(),
   	       duty_cycle: DutyCycle::Ratio2to1,
   	       },
   	   clocks,
   	   &mut rcc.apb1,
   	   1000,
   	   10,
   	   1000,
   	   1000,
   	   );

       // led
       let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);
       let led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);   // led on pc13

       (i2c, led,  Delay::new(cp.SYST, clocks))   // return tuple (i2c, led, delay)
       }



#[cfg(feature = "stm32f3xx")]  //  eg Discovery-stm32f303
use stm32f3xx_hal::{prelude::*, 
                    stm32::Peripherals,
                    i2c::I2c,  
    	            delay::Delay,
		    gpio::{gpiob::{PB6, PB7}, AF4,   gpioe::PE9, Output, PushPull,  },
		    stm32::I2C1,
		    };
	   
    #[cfg(feature = "stm32f3xx")]
    fn setup() ->  (I2c<I2C1, (PB6<AF4>, PB7<AF4>)>,  PE9<Output<PushPull>>, Delay) {

       let cp = cortex_m::Peripherals::take().unwrap();
       let dp = Peripherals::take().unwrap();

       let mut rcc   = dp.RCC.constrain();
       let clocks = rcc.cfgr.freeze(&mut dp.FLASH.constrain().acr);
      
       let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);   // for i2c

       let i2c = I2c::i2c1(
          dp.I2C1, 

	  (gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl),     // i2c scl on pb6, 
	   gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl) ),   // i2c sda on pb7), 

	  100.khz(), 
	  clocks, 
	  &mut rcc.apb1
	  );

       // led
       let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);
       let led = gpioe.pe9.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);  // led on pe9
             //.into()  ?? 

       (i2c, led,  Delay::new(cp.SYST, clocks) )  // return tuple (i2c, led, delay)
       }


#[cfg(feature = "stm32f4xx")] // eg Nucleo-64  stm32f411
use stm32f4xx_hal::{prelude::*,  
                    pac::Peripherals, 
                    i2c::I2c,  
    	            delay::Delay,
		    gpio::{gpiob::{PB8, PB7}, Alternate, AF4,  gpioe::PE9, Output, PushPull,  },
                    pac::I2C1,
		    }; 
#[cfg(feature = "stm32f4xx")] // eg Nucleo-64  stm32f411
use embedded_hal::digital::v2::OutputPin;
	   
    #[cfg(feature = "stm32f4xx")]
    fn setup() ->  (I2c<I2C1, (PB8<Alternate<AF4>>, PB7<Alternate<AF4>>)>,
                              PE9<Output<PushPull>>, Delay) {

       let cp = cortex_m::Peripherals::take().unwrap();
       let dp = Peripherals::take().unwrap();

       let mut rcc   = dp.RCC.constrain();
       let clocks = rcc.cfgr.freeze();
      
       let mut gpiob = dp.GPIOB.split();   // for i2c

       let i2c = I2c::i2c1(
          dp.I2C1, 

	  (gpiob.pb8.into_alternate_af4(),                    // i2c scl on pb8, 
	   gpiob.pb7.into_alternate_af4() ),                  // i2c sda on pb7), 

	  100.khz(), 
	  clocks, 
	  );

       // led
       let mut gpioe = dp.GPIOE.split();
       let led = gpioe.pe9.into_push_pull_output();           // led on pe9
       //let led = gpiob.pb13.into_push_pull_output();          // external led on pb13

       (i2c, led,  Delay::new(cp.SYST, clocks) )     // return tuple (i2c, led, delay)
       }


    // End of hal/MCU specific setup. Following should be generic code.


#[entry]
fn main() -> ! {

    let (i2c, mut led, mut delay) = setup();

    let manager = shared_bus::BusManager::<cortex_m::interrupt::Mutex<_>, _>::new(i2c);
    let mut disp: GraphicsMode<_> = Builder::new().connect_i2c(manager.acquire()).into();

    disp.init().unwrap();
    disp.flush().unwrap();

    let mut adc = Ads1x1x::new_ads1015(manager.acquire(), SlaveAddr::default());
    // need to be able to measure [0-5V]
    adc.set_full_scale_range(FullScaleRange::Within6_144V)
        .unwrap();

    loop {
        // Blink LED 0 to check that everything is actually running.
        // If the LED 0 is off, something went wrong.
        led.set_high().unwrap();
        delay.delay_ms(50_u16);
        led.set_low().unwrap();
        delay.delay_ms(50_u16);

        // Read voltage in all channels
        let values = [
            block!(adc.read(&mut AdcChannel::SingleA0)).unwrap_or(8091),
            block!(adc.read(&mut AdcChannel::SingleA1)).unwrap_or(8091),
            block!(adc.read(&mut AdcChannel::SingleA2)).unwrap_or(8091),
            block!(adc.read(&mut AdcChannel::SingleA3)).unwrap_or(8091),
        ];

        let mut lines: [heapless::String<heapless::consts::U32>; 4] = [
            heapless::String::new(),
            heapless::String::new(),
            heapless::String::new(),
            heapless::String::new(),
        ];

        // write some extra spaces after the number to clear up when the numbers get smaller
        for i in 0..values.len() {
            write!(lines[i], "Channel {}: {}    ", i, values[i]).unwrap();
            disp.draw(
                Font6x8::render_str(&lines[i])
                    .with_stroke(Some(1u8.into()))
                    .translate(Coord::new(0, i as i32 * 16))
                    .into_iter(),
            );
        }
        disp.flush().unwrap();
    }
}
