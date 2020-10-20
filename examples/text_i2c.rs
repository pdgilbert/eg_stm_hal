//! Using crate ssd1306 to print with i2c on a generic ssd1306 based OLED display.
//!
//! Print "Hello world!" then "Hello rust!". Uses the `embedded_graphics` crate to draw.
//! Wiring pin connections for scl and sda to display as in the setup sections below.
//! Tested on generic (cheap) ssd1306 OLED 0.91" 128x32 and 0.96" 128x64 displays.
//! Note that the DisplaySize setting needs to be adjusted for 128x64 or 128x32 display
//!
//! This example based on 
//!    https://github.com/jamwaffles/ssd1306/blob/master/examples/text_i2c.rs
//! with stm32f4xx_hal setup following 
//!    https://github.com/stm32-rs/stm32f4xx-hal/blob/master/examples/ssd1306-image.rs
//!
//! Compare this example with oled_gps.


#![no_std]
#![no_main]

use cortex_m_rt::{entry, exception, ExceptionFrame};

//builtin include Font6x6, Font6x8, Font6x12, Font8x16, Font12x16, Font24x32
use embedded_graphics::{
    fonts::{Font8x16, Text}, 
    pixelcolor::BinaryColor,
    prelude::*,
    style::TextStyleBuilder,
   };
use panic_halt as _;

use ssd1306::{prelude::*, Builder, I2CDIBuilder};


// setup() does all  hal/MCU specific setup and returns generic hal device for use in main code.

#[cfg(feature = "stm32f0xx")]  //  eg stm32f030xc
use stm32f0xx_hal::{prelude::*,
                    pac::Peripherals, 
		    i2c::{I2c, },
		    gpio::{gpiob::{PB7, PB8}, Alternate, AF1, },
		    pac::I2C1,
		    };

    #[cfg(feature = "stm32f0xx")]
    fn setup() ->  I2c<I2C1,  PB8<Alternate<AF1>>, PB7<Alternate<AF1>> > {

       let mut p   = Peripherals::take().unwrap();
       let mut rcc = p.RCC.configure().freeze(&mut p.FLASH);

       let gpiob = p.GPIOB.split(&mut rcc);
       
       let  (scl,  sda) = cortex_m::interrupt::free(move |cs| {
            (gpiob.pb8.into_alternate_af1(cs),   // scl on PB8
             gpiob.pb7.into_alternate_af1(cs),   // sda on PB7
             )
            });
 
       // return i2c
       I2c::i2c1(p.I2C1, (scl,  sda), 400.khz(), &mut rcc, )
       }



#[cfg(feature = "stm32f1xx")]  //  eg blue pill stm32f103
use stm32f1xx_hal::{prelude::*,
                    pac::Peripherals, 
		    i2c::{BlockingI2c, DutyCycle, Mode},
		    gpio::{gpiob::{PB8, PB9}, Alternate, OpenDrain, },
		    device::I2C1,
		    };

    #[cfg(feature = "stm32f1xx")]
    fn setup() ->  BlockingI2c<I2C1,  (PB8<Alternate<OpenDrain>>, PB9<Alternate<OpenDrain>>) > {
  
       let p = Peripherals::take().unwrap();
       let mut rcc = p.RCC.constrain();

       let clocks = rcc.cfgr.freeze(&mut  p.FLASH.constrain().acr);
       let mut afio = p.AFIO.constrain(&mut rcc.apb2);

       let mut gpiob = p.GPIOB.split(&mut rcc.apb2);
       
       // return i2c
       BlockingI2c::i2c1(
   	   p.I2C1,
   	   (gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh),   // scl on PB8
	    gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh)),  // sda on PB9
   	   &mut afio.mapr,
   	   Mode::Fast {
   	       frequency: 400_000.hz(),
   	       duty_cycle: DutyCycle::Ratio2to1,
   	   },
   	   clocks,
   	   &mut rcc.apb1,
   	   1000,
   	   10,
   	   1000,
   	   1000,
           )
       }



#[cfg(feature = "stm32f3xx")]  //  eg Discovery-stm32f303
use stm32f3xx_hal::{prelude::*, 
                    pac::Peripherals,
                    i2c::{I2c, },  
		    gpio::{gpiob::{PB8, PB9}, AF4, },
		    pac::I2C1,
		    };
	   
    #[cfg(feature = "stm32f3xx")]
    fn setup() ->  I2c<I2C1, (PB8<AF4>, PB9<AF4>)> {
  
       let p = Peripherals::take().unwrap();
       let mut rcc = p.RCC.constrain();
       let clocks = rcc.cfgr.freeze(&mut  p.FLASH.constrain().acr);
       let mut gpiob = p.GPIOB.split(&mut rcc.ahb);

       let scl = gpiob.pb8.into_af4(&mut gpiob.moder, &mut gpiob.afrh);   // scl on PB8
       let sda = gpiob.pb9.into_af4(&mut gpiob.moder, &mut gpiob.afrh);   // sda on PB9
      
       // return i2c
       I2c::i2c1(p.I2C1, (scl, sda), 400_000.hz(), clocks, &mut rcc.apb1 )
       }


#[cfg(feature = "stm32f4xx")] // eg Nucleo-64, blackpills stm32f401 and stm32f411
use stm32f4xx_hal::{prelude::*,  
                    pac::Peripherals, 
                    i2c::{I2c, },  
		    gpio::{gpiob::{PB8, PB9}, AlternateOD, AF4, },
                    pac::I2C1,
		    }; 

    #[cfg(feature = "stm32f4xx")]
    fn setup() ->  I2c<I2C1, (PB8<AlternateOD<AF4>>, PB9<AlternateOD<AF4>>)> {
  
       let  p  = Peripherals::take().unwrap();
       let rcc = p.RCC.constrain();
       let clocks = rcc.cfgr.freeze();
       let gpiob  = p.GPIOB.split();
       
       // could also have scl on PB6, sda on PB7
       //BlockingI2c::i2c1(
       let scl = gpiob.pb8.into_alternate_af4().set_open_drain();   // scl on PB8
       let sda = gpiob.pb9.into_alternate_af4().set_open_drain();   // sda on PB9
       
       // return i2c
       I2c::i2c1(p.I2C1, (scl, sda), 400.khz(), clocks)
       }


#[cfg(feature = "stm32f7xx")] 
use stm32f7xx_hal::{prelude::*,  
                    pac::Peripherals, 
                    i2c::{BlockingI2c, Mode, PinScl, PinSda},  
		    pac::I2C1,
		    }; 

    //fn setup() ->  BlockingI2c<I2C1, PB8<Alternate<AF4>>, PB9<Alternate<AF4>>> {
    //fn setup() ->  BlockingI2c<I2C1, impl PinScl<AF4>, impl PinSda<AF4>> {
    //fn setup() ->  BlockingI2c<I2C1, impl PinScl<I2C1>, impl PinSda<I2C1>> {
    //fn setup() ->  BlockingI2c<I2C1, impl PinScl<I2C1>, impl PinSda<I2C1>> {
    #[cfg(feature = "stm32f7xx")]
    fn setup() ->  BlockingI2c<I2C1, impl PinScl<I2C1>, impl PinSda<I2C1>> {

       let  p  = Peripherals::take().unwrap();
       let mut rcc = p.RCC.constrain();
       let clocks = rcc.cfgr.freeze();
       
       let gpiob  = p.GPIOB.split();
       
       let scl = gpiob.pb8.into_alternate_af4().set_open_drain();   // scl on PB8
       let sda = gpiob.pb9.into_alternate_af4().set_open_drain();   // sda on PB9
       
       // return i2c
       BlockingI2c::i2c1(
           p.I2C1, 
	   (scl, sda), 
	   //400.khz(), 
   	   Mode::Fast { frequency: 400_000.hz(), },
	   clocks, 
	   &mut rcc.apb1,
	   1000)
       }


#[cfg(feature = "stm32h7xx")] 
use stm32h7xx_hal::{prelude::*,  
                    pac::Peripherals, 
                    i2c::{I2c,},  
		    //gpio::{gpiob::{PB8, PB9}, Alternate, AF4, }, really! builds without this
                    pac::I2C1,
		    }; 

    #[cfg(feature = "stm32h7xx")]
    fn setup() ->  I2c<I2C1> {
  
       let p      = Peripherals::take().unwrap();
       let pwr    = p.PWR.constrain();
       let vos    = pwr.freeze();
       let rcc    = p.RCC.constrain();
       let ccdr   = rcc.sys_ck(160.mhz()).freeze(vos, &p.SYSCFG);
       let clocks = ccdr.clocks;
       
       let gpiob  = p.GPIOB.split(ccdr.peripheral.GPIOB);
 
       let scl = gpiob.pb8.into_alternate_af4().set_open_drain();   // scl on PB8
       let sda = gpiob.pb9.into_alternate_af4().set_open_drain();   // sda on PB9
       
       // return i2c
       // I2c::i2c1(p.I2C1, (scl, sda), 400.khz(), clocks)
       p.I2C1 .i2c((scl, sda), 400.khz(), ccdr.peripheral.I2C1, &clocks)
       }


#[cfg(feature = "stm32l0xx")] 
use stm32l0xx_hal::{prelude::*,  
                    pac::Peripherals, 
		    rcc,   // for ::Config but note name conflict with serial
                    i2c::{I2c, },  
		    gpio::{gpiob::{PB8, PB9}, Output, OpenDrain},
                    pac::I2C1,
		    }; 

    #[cfg(feature = "stm32l0xx")]
    fn setup() ->  I2c<I2C1, PB9<Output<OpenDrain>>, PB8<Output<OpenDrain>>> {
  
       let  p  = Peripherals::take().unwrap();
       let mut rcc = p.RCC.freeze(rcc::Config::hsi16());
       let gpiob  = p.GPIOB.split(&mut rcc);
       
       // could also have scl on PB6, sda on PB7
       //BlockingI2c::i2c1(
       let scl = gpiob.pb8.into_open_drain_output();   // scl on PB8
       let sda = gpiob.pb9.into_open_drain_output();   // sda on PB9
       
       // return i2c
       p.I2C1.i2c(sda, scl, 400.khz(), &mut rcc)
       }



#[cfg(feature = "stm32l1xx") ] // eg  Discovery STM32L100 and Heltec lora_node STM32L151CCU6
use stm32l1xx_hal::{prelude::*, 
                    stm32::Peripherals,
		    rcc,   // for ::Config but avoid name conflict with serial
                    i2c::{I2c, Pins, },  
		    //gpio::{gpiob::{PB8, PB9}, Output, OpenDrain, },
                    stm32::I2C1,
                    };
   
    #[cfg(feature = "stm32l1xx")]
    fn setup() -> I2c<I2C1, impl Pins<I2C1>> {

       //above can use I2c<I2C1, (PB8<Output<OpenDrain>>, PB9<Output<OpenDrain>>)>
       // that requires also   gpio::{gpiob::{PB8, PB9}, Output, OpenDrain, },

       let  p  = Peripherals::take().unwrap();
       let mut rcc = p.RCC.freeze(rcc::Config::hsi());

       let gpiob  = p.GPIOB.split();
       
       // could also have scl,sda  on PB6,PB7 or on PB10,PB11
       let scl = gpiob.pb8.into_open_drain_output();   // scl on PB8
       let sda = gpiob.pb9.into_open_drain_output();   // sda on PB9
       
       //BlockingI2c::i2c1( ??

       // return i2c
       p.I2C1.i2c((scl, sda), 400.khz(), &mut rcc) 
       }


#[cfg(feature = "stm32l4xx")] 
use stm32l4xx_hal::{prelude::*,  
                    pac::Peripherals, 
                    i2c::{I2c, },  
		    gpio::{gpiob::{PB10, PB11}, Alternate, AF4, Output, OpenDrain, },
                    pac::I2C2,
		    }; 

    #[cfg(feature = "stm32l4xx")]
    fn setup() ->  I2c<I2C2, (PB10<Alternate<AF4, Output<OpenDrain>>>, PB11<Alternate<AF4, Output<OpenDrain>>>)> {
  
       let  p     = Peripherals::take().unwrap();
       let mut flash = p.FLASH.constrain();
       let mut rcc = p.RCC.constrain();
       let mut pwr = p.PWR.constrain(&mut rcc.apb1r1);
       let clocks = rcc.cfgr .sysclk(80.mhz()) .pclk1(80.mhz()) 
                             .pclk2(80.mhz()) .freeze(&mut flash.acr, &mut pwr);

       let mut gpiob  = p.GPIOB.split(&mut rcc.ahb2);
       
       
       // following ttps://github.com/stm32-rs/stm32l4xx-hal/blob/master/examples/i2c_write.rs
       let mut scl = gpiob.pb10.into_open_drain_output(&mut gpiob.moder, &mut gpiob.otyper);  // scl on PB10
       scl.internal_pull_up(&mut gpiob.pupdr, true);
       let scl = scl.into_af4(&mut gpiob.moder, &mut gpiob.afrh);

       let mut sda = gpiob.pb11.into_open_drain_output(&mut gpiob.moder, &mut gpiob.otyper);  // sda on PB11
       sda.internal_pull_up(&mut gpiob.pupdr, true);
       let sda = sda.into_af4(&mut gpiob.moder, &mut gpiob.afrh);
    
       // return i2c
       I2c::i2c2(p.I2C2, (scl, sda), 400.khz(), clocks, &mut rcc.apb1r1)
       }


    // End of hal/MCU specific setup. Following should be generic code.


#[entry]
fn main() -> ! {

    let i2c = setup();
    
    let interface = I2CDIBuilder::new().init(i2c);
    let mut disp: GraphicsMode<_, _> = Builder::new()
                    .size(DisplaySize128x64)        // set display size 128x32, 128x64
		    .connect(interface)
		    .into();
    disp.init().unwrap();

    //builtin include Font6x6, Font6x8, Font6x12, Font8x16, Font12x16, Font24x32
    let text_style = TextStyleBuilder::new(Font8x16) 
        .text_color(BinaryColor::On)
        .build();

    Text::new("Hello world!", Point::zero())
        .into_styled(text_style)
        .draw(&mut disp)
        .unwrap();

    Text::new("Hello Rust!", Point::new(0, 20))
        .into_styled(text_style)
        .draw(&mut disp)
        .unwrap();

    disp.flush().unwrap();

    loop {}
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
