// see https://github.com/stm32-rs/stm32f1xx-hal/blob/master/examples/adc.rs
// for stm32f4xx see examples in
//     https://docs.rs/stm32f4xx-hal/0.8.3/stm32f4xx_hal/adc/struct.Adc.html
// http://ctms.engin.umich.edu/CTMS/Content/Activities/TMP35_36_37.pdf
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_semihosting as _;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

#[cfg(feature = "stm32f1xx")]  //  eg blue pill stm32f103
use stm32f1xx_hal::{prelude::*, 
                    pac::Peripherals, 
                    adc::Adc,
		    gpio::{gpiob::{PB1}, Analog},  
		    device::{ADC1, ADC2},
                    };

#[cfg(feature = "stm32f3xx")]  //  eg Discovery-stm32f303
use stm32f3xx_hal::{prelude::*, 
                    stm32::Peripherals, 
                    adc::Adc,
		    gpio::{gpiob::{PB1}, Analog},  
		    pac::{ADC1, ADC2},
                    };

#[cfg(feature = "stm32f4xx")] // eg Nucleo-64  stm32f411
use stm32f4xx_hal::{prelude::*, 
                    pac::Peripherals, 
                    adc::Adc,
		    gpio::{gpiob::{PB1}, Analog},  
		    adc::Adc::{ADC1, ADC2},
                    };


//use stm32f4xx_hal::{ gpio::gpioa, adc::{ Adc, config::AdcConfig, config::SampleTime, }, };
//use stm32f4xx_hal::{ gpio::gpioa, adc::{ Adc, config::AdcConfig, config::{SampleTime,
//           Sequence, Eoc, Scan, Clock}, }, };


#[cfg(feature = "stm32l1xx") ] // eg  Discovery kit stm32l100 and Heltec lora_node STM32L151CCU6
use stm32l1xx_hal::{prelude::*, 
                    stm32::Peripherals, 
                    adc::Adc,
		    gpio::{gpiob::{PB1}, Analog},  
		    pac::{ADC1, ADC2},
                    };


#[entry]
fn main() -> ! {

    #[cfg(feature = "stm32f1xx")]
    fn setup() ->  (Adc<ADC1>, Adc<ADC2>, PB1<Analog>)  {
    
       let p = Peripherals::take().unwrap();
       let mut flash = p.FLASH.constrain();
       let mut rcc = p.RCC.constrain();

       // Configure ADC clocks
       // Default value is the slowest possible ADC clock: PCLK2 / 8  ?
       // ADC clock is configurable, so its frequency can be changed.
       // User specified value is be approximated using supported  prescaler values 2/4/6/8  ?

       let clocks = rcc.cfgr.adcclk(2.mhz()).freeze(&mut flash.acr);
    
       // with above on bluepill  clocks.sysclk() is  8 Mhz and  clocks.adcclk() is  2 Mhz
       // with below on bluepill  clocks.sysclk() is 56 Mhz and  clocks.adcclk() is 14 Mhz
       // The mcu temp does not seem to be affected by this difference
       // but the external analog temperature (adc_temp) is high by 6-7deg C with clock below.
    
       //let clocks = rcc.cfgr.use_hse(8.mhz()).sysclk(56.mhz())
       //    .pclk1(28.mhz()).adcclk(14.mhz()).freeze(&mut flash.acr);

       hprintln!("sysclk freq: {}", clocks.sysclk().0).unwrap();  
       hprintln!("adc freq: {}", clocks.adcclk().0).unwrap();    


       // Setup ADC1 for internal MCU temperature

       let mcuadc = Adc::adc1(p.ADC1, &mut rcc.apb2, clocks);

       let mut gpiob = p.GPIOB.split(&mut rcc.apb2);

       // Configure analog input on pb0 
       //let mut adc1 = adc::Adc::adc1(p.ADC1, &mut rcc.apb2, clocks);
       //let mut ch0 = gpiob.pb0.into_analog(&mut gpiob.crl);

       // Setup external analog temperature sensor (eg TMP36) input on pin pb1 using ADC2
       let adc2 = Adc::adc2(p.ADC2, &mut rcc.apb2, clocks);
       let ch1 = gpiob.pb1.into_analog(&mut gpiob.crl);
       
       (mcuadc, adc2, ch1)
       };

//found tuple `(stm32f4xx_hal::adc::Adc<stm32f4::stm32f411::ADC1>, _, //stm32f4xx_hal::gpio::gpiob::PB1<stm32f4xx_hal::gpio::Analog>)`


    #[cfg(feature = "stm32f4xx")]
    fn setup() ->  (Adc<ADC1>, _, PB1<Analog>) {
       
       let p = Peripherals::take().unwrap();
       let mut flash = p.FLASH.constrain();
       let mut rcc = p.RCC.constrain();

       let clocks = rcc.cfgr.adcclk(2.mhz()).freeze(&mut flash.acr);
        
       //let clocks = rcc.cfgr.use_hse(8.mhz()).sysclk(56.mhz())
       //    .pclk1(28.mhz()).adcclk(14.mhz()).freeze(&mut flash.acr);

       hprintln!("sysclk freq: {}", clocks.sysclk().0).unwrap();  
       hprintln!("adc freq: {}", clocks.adcclk().0).unwrap();    


       // Setup ADC1 for internal MCU temperature
      
       let mut mcuadc = Adc::adc1(device.ADC1, true, AdcConfig::default());

       let mut gpiob = p.GPIOB.split();

       // Configure analog input on pb0 
       //let mut adc1 = adc::Adc::adc1(p.ADC1, &mut rcc.apb2, clocks);
       //let mut ch0 = gpiob.pb0.into_analog(&mut gpiob.crl);

       // Setup external analog temperature sensor (eg TMP36) input on pin pb1 using ADC2
       let mut adc2 = Adc::adc2(p.ADC2, &mut rcc.apb2, clocks);
       let mut ch1 = gpiob.pb1.into_analog();
       
       (mcuadc, adc2, ch1)
       };


    // End of hal/MCU specific setup. Following should be generic code.


    let (mut mcuadc,   mut adc2,   mut ch1) = setup();  


    // Vin = 3.3v * ADCvalue / 4096     (12 bit adc has  2**12 = 4096 steps)
    // TMP35 scale is 100 deg C per 1.0v (slope 10mV/deg C) and goes through 
    //   <50C, 1.0v>,  so 0.0v is  -50C.
    // see https://www.analog.com/media/en/technical-documentation/data-sheets/TMP35_36_37.pdf
    // so ADCtemp = (100 * 3.3 * ADCvalue / 4096 )  - 50 = 0.0805664 * ADCvalue - 50

    // This compiles but the link fails because the bin is too big for flash on bluepill
    //   let adc_temp: f64 = (0.0805664 * adc_value as f64 ) - 50.0 ;	
    // these work
    //   let adc_temp:  i16 = ((0.0805664f32 * adc_value as f32 ) - 50.0f32) as i16 ;
    //   let adc_temp:  i16 = (0.0805664f32 * adc_value as f32 ) as i16 - 50  ;
    //   let adc_temp:  i16 = (adc_value as f32 / 12.412122 ) as i16 - 50  ;
    // and this works but the rounding is bad (a few degrees off)
    //   let adc_temp:  i16 = (adc_value / 12 ) as i16 - 50  ;

    loop {
        let mcutemp = mcuadc.read_temp();
        hprintln!("MCU temp: {}", mcutemp).unwrap();

	let adc_value: u16 = adc2.read(&mut ch1).unwrap();
	let adc_temp:  i16 = (adc_value as f32 / 12.412122 ) as i16 - 50  ;
        hprintln!("adc2: {}", adc_temp).unwrap();
        }
}
