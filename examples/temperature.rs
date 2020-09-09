/*!  Measure the internal mcu temperature sensor and an analog external TMP36 sensor.
     see https://github.com/stm32-rs/stm32f1xx-hal/blob/master/examples/adc.rs
     for stm32f4xx see examples in
 	 https://docs.rs/stm32f4xx-hal/0.8.3/stm32f4xx_hal/adc/struct.Adc.html
     http://ctms.engin.umich.edu/CTMS/Content/Activities/TMP35_36_37.pdf
     TMP36   analog temperature sensor

      Notes of Interest:  
       -I don't understand the details of setting  ADC or ADC clocks. If you know what you are
 	 doing you can probably do better than what is done below. Please let me know of important
 	 improvements by entering an issue at https://github.com/pdgilbert/eg_stm_hal

     For digital temperature sensor exanples see  ds1820.rs, dht.rs and dht11.rs.
*/

#![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_semihosting as _;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;


// setup() does all  hal/MCU specific setup and returns generic hal device for use in main code.

#[cfg(feature = "stm32f1xx")]  //  eg blue pill stm32f103
use stm32f1xx_hal::{prelude::*, 
                    pac::Peripherals, 
                    adc::Adc,
		    gpio::{gpiob::{PB1}, Analog},
		    device::{ADC1, ADC2},
                    };

    #[cfg(feature = "stm32f1xx")]
    fn setup() ->  (AdcCh<Adc<ADC1>, ()>, AdcCh<Adc<ADC2>, PB1<Analog>>) {
  
       let p = Peripherals::take().unwrap();
       let mut flash = p.FLASH.constrain();
       let mut rcc = p.RCC.constrain();

       // Configure ADC clocks. See Notes of Interest above.

       let clocks = rcc.cfgr.adcclk(2.mhz()).freeze(&mut flash.acr);
    
       /*  With above on bluepill  clocks.sysclk() is  8 Mhz and  clocks.adcclk() is  2 Mhz.
           With below on bluepill  clocks.sysclk() is 56 Mhz and  clocks.adcclk() is 14 Mhz.
           The mcu temp does not seem to be affected by this difference
           but the external analog temperature (tmp36) is high by 6-7deg C with clock below.    
           
	   let clocks = rcc.cfgr.use_hse(8.mhz()).sysclk(56.mhz())
            .pclk1(28.mhz()).adcclk(14.mhz()).freeze(&mut flash.acr);
       */ 

       hprintln!("sysclk freq: {}", clocks.sysclk().0).unwrap();  
       hprintln!("adc freq: {}", clocks.adcclk().0).unwrap();    

       /*  The MCU temperature sensor is internally connected to the ADC12_IN16 input channel
           so no channel needs to be specified here.
       
           See https://docs.rs/stm32f1xx-hal/0.6.1/stm32f1xx_hal/adc/struct.Adc.html
           regarding how well (badly) read_temp() works. "... varies from chip to chip due 
	   to process variation (up to 45 C from one chip to another)."
	   It is better for temperature change rather than actual temperature.
       */ 
              
       let mcutemp = AdcCh{
                        adc: Adc::adc1(p.ADC1, &mut rcc.apb2, clocks),  // MCU temperature using ADC1
			ch: () ,                                        // no channel
			};
    
        impl ReadTempC for AdcCh<Adc<ADC1>, ()> {
           fn read_tempC(&mut self) -> i32 {
	      self.adc.read_temp()
	      }
          };


       let mut gpiob = p.GPIOB.split(&mut rcc.apb2);
        
       let tmp36 = AdcCh{
                         adc: Adc::adc2(p.ADC2, &mut rcc.apb2, clocks), // TMP36 on PB1 using ADC2
			 ch:  gpiob.pb1.into_analog(&mut gpiob.crl),    //  using channel pb1
			 };
      
       impl ReadMV for AdcCh<Adc<ADC2>, PB1<Analog>> {
          fn read_mv(&mut self) -> u32 {
	     self.adc.read(&mut self.ch).unwrap()
	     }
          };
      
       impl ReadTempC for AdcCh<Adc<ADC2>, PB1<Analog>> {
          fn read_tempC(&mut self) -> i32 {
	     let v:  u32 = self.adc.read(&mut self.ch).unwrap();
	     (v as f32 / 12.412122 ) as i32 - 50 as i32
	     }
          };

       (mcutemp, tmp36)
       }


#[cfg(feature = "stm32f3xx")]  //  eg Discovery-stm32f303
use stm32f3xx_hal::{prelude::*, 
                    stm32::Peripherals, 
                    adc::Adc,
		    gpio::{gpiob::{PB1}, Analog},
		    stm32::{ADC1, ADC2},
                    };

    #[cfg(feature = "stm32f3xx")]
    fn setup() ->  (Adc<ADC1>, Adc<ADC2>, PB1<Analog>)  {
    
       let p = Peripherals::take().unwrap();
       let mut flash = p.FLASH.constrain();
       let mut rcc = p.RCC.constrain();

       // Configure ADC clocks. See Notes of Interest above.

       let clocks = rcc.cfgr.adcclk(2.mhz()).freeze(&mut flash.acr);

       let mcutemp = AdcCh{
                        adc: Adc::adc1(p.ADC1, &mut rcc.apb2, clocks),  // MCU temperature using ADC1
			ch: () ,                                        // no channel
			};
    
        impl ReadTempC for AdcCh<Adc<ADC1>, ()> {
           fn read_tempC(&mut self) -> i32 {
	      self.adc.read_temp()
	      }
          };


       let mut gpiob = p.GPIOB.split(&mut rcc.apb2);
        
       let tmp36 = AdcCh{
                         adc: Adc::adc2(p.ADC2, &mut rcc.apb2, clocks), // TMP36 on PB1 using ADC2
			 ch:  gpiob.pb1.into_analog(&mut gpiob.crl),    //  using channel pb1
			 };
      
       impl ReadMV for AdcCh<Adc<ADC2>, PB1<Analog>> {
          fn read_mv(&mut self) -> u32 {
	     self.adc.read(&mut self.ch).unwrap()
	     }
          };

       (mcutemp, tmp36)
       }


#[cfg(feature = "stm32f4xx")] // eg Nucleo-64  stm32f411
use stm32f4xx_hal::{prelude::*, 
                    pac::Peripherals, 
                    adc::{Adc, Temperature, config::{AdcConfig, SampleTime}},
		    gpio::{gpiob::{PB1}, Analog},
		    stm32::{ADC1}, //ADC2},          // 405 has ADC2 but 401 and 411 have only one adc
                    };

//use stm32f4xx_hal::{ gpio::gpioa, adc::{ Adc, config::AdcConfig, config::{SampleTime,
//           Sequence, Eoc, Scan, Clock}, }, };

    #[cfg(feature = "stm32f4xx")]
    fn setup() ->  (AdcCh<&'static Adc<ADC1>, Temperature>, AdcCh<&'static Adc<ADC1>, PB1<Analog>>) {
    //fn setup() ->  AdcCh<Adc<ADC1>, Temperature> {
    //fn setup() ->  AdcCh<Adc<ADC1>, PB1<Analog>> {

       // see https://docs.rs/stm32f4xx-hal/0.8.3/stm32f4xx_hal/adc/struct.Adc.html
       // and https://docs.rs/stm32f4xx-hal/0.8.3/stm32f4xx_hal/adc/struct.Adc.html#method.adc2
       
       let p = Peripherals::take().unwrap();
       let rcc = p.RCC.constrain();

       //from datasheet:To synchronize A/D conversion and timers, the ADCs could be triggered by 
       //any of TIM1,TIM2, TIM3, TIM4 or TIM5 timer.
       
       let clocks = rcc.cfgr.hclk(48.mhz()).sysclk(48.mhz()).pclk1(24.mhz()).pclk2(24.mhz()).freeze();

       hprintln!("sysclk freq: {}", clocks.sysclk().0).unwrap();  
      
       // stm32f411: The temperature sensor is internally connected to the ADC_IN18 input channel 
        
       let adc = Adc::adc1(p.ADC1, true, AdcConfig::default());
       
       // one-shot conversion       
       let mcutemp = AdcCh{
                        adc: &adc,                                     // MCU temperature using ADC1
                         ch: Temperature,                              // channel internally 18
                     };
      
       impl ReadTempC for AdcCh<&'static Adc<ADC1>, Temperature> {
          fn read_tempC(&mut self) -> i32 {
	     let ch  = &mut self.ch;
	     let adc = &self.adc;
             let v:  u32 = adc.read(ch).unwrap() as u32; 
             (v as f32 / 12.412122 ) as i32 - 50  as i32 // NOT RIGHT
             }
          };


       let gpiob = p.GPIOB.split();

       // one-shot conversion
       
       let tmp36 = AdcCh{
        		 adc: &adc,                                    // TMP36 on PB1 using ADC1 
       	                  ch: gpiob.pb1.into_analog(),	               //  using channel on pb1
       	       };
       
       impl ReadMV for AdcCh<&'static Adc<ADC1>, PB1<Analog>> {
          fn read_mv(&mut self) -> u32 {
	     let ch  = &mut self.ch;
	     let adc = &self.adc;
	     adc.read(ch).unwrap() as u32
	     }
          };

       impl ReadTempC for AdcCh<&'static Adc<ADC1>, PB1<Analog>> {
          fn read_tempC(&mut self) -> i32 {
	     let ch = &mut self.ch;
	     let adc = &self.adc;
	     let v:  u32 = adc.read(ch).unwrap() as u32;
	     (v as f32 / 12.412122 ) as i32 - 50 as i32
	     }
          };

       //mcutemp
       //tmp36
       (mcutemp, tmp36)
       }



#[cfg(feature = "stm32l1xx") ] // eg  Discovery kit stm32l100 and Heltec lora_node STM32L151CCU6
use stm32l1xx_hal::{prelude::*, 
                    stm32::Peripherals, 
                    adc::Adc,
		    gpio::{gpiob::{PB1}, Analog},  
		    pac::{ADC1, ADC2},
                    };


// End of hal/MCU specific setup. Following should be generic code.


pub struct AdcCh <T, U> {                  // combined ADC and  channel
   adc : T,  //Adc<ADC1>,
   ch  : U,  //PB1<Analog>,
   }

pub trait ReadTempC {                      // generics for reading AdcCh temp
   #![allow(non_snake_case)]
   fn read_tempC(&mut self) -> i32;        // temperature in degrees Celsius   
   }

pub trait ReadMV {                         // generics for reading AdcCh
   fn read_mv(&mut self)    -> u32;        // millivolts
   }


#[entry]
fn main() -> ! {

    let (mut mcutemp,   mut tmp36) = setup();  

    /*TMP35 has linear output with scale calculation as follows.
      Vin = 3.3v * ADCvalue / 4096     (12 bit adc has  2**12 = 4096 steps)
      TMP35 scale is 100 deg C per 1.0v (slope 10mV/deg C) and goes through 
     	<50C, 1.0v>,  so 0.0v is  -50C.
      see https://www.analog.com/media/en/technical-documentation/data-sheets/TMP35_36_37.pdf
      so temp = (100 * 3.3 * ADCvalue / 4096 )  - 50 = 0.0805664 * ADCvalue - 50
    */

    // This compiles but the link fails because the bin is too big for flash on bluepill
    //   let adc_temp: f64 = (0.0805664 * adc_value as f64 ) - 50.0 ;	
    // these work
    //   let adc_temp:  i16 = ((0.0805664f32 * adc_value as f32 ) - 50.0f32) as i16 ;
    //   let adc_temp:  i16 = (0.0805664f32 * adc_value as f32 ) as i16 - 50  ;
    //   let adc_temp:  i16 = (adc_value as f32 / 12.412122 ) as i16 - 50  ;
    // and this works but the rounding is bad (a few degrees off)
    //   let adc_temp:  i16 = (adc_value / 12 ) as i16 - 50  ;

    loop {
        let mcu_value = mcutemp.read_tempC();
        hprintln!("MCU temp: {}", mcu_value).unwrap();

	let tmp36_value: u32 = tmp36.read_mv();
	let tmp36_temp:  i32 = tmp36.read_tempC();
	hprintln!("external TMP36 millivolts: {}, temperature: {} C.", tmp36_value, tmp36_temp).unwrap();
        }
}
