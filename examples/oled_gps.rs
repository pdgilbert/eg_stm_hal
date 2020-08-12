//! Read GPS on usart serial interface and display on OLED with i2c.
//! Compare this example with gps_rw, lora_gps and text_i2c.

#![deny(unsafe_code)]
#![no_main]
#![no_std]


#[cfg(debug_assertions)]
extern crate panic_semihosting;

#[cfg(not(debug_assertions))]
extern crate panic_halt;

//use cortex_m::asm;

use heapless::{consts, Vec};

use cortex_m_rt::entry;
//use core::fmt::Write;  // for writeln
use cortex_m_semihosting::hprintln;
//use core::str;
//use core::ascii;
use nb::block;

use eg_stm_hal::to_str;

use embedded_graphics::{
    fonts::{Font8x16, Text},   // Font6x8, Font12x16, Font6x12, Font8x16
    pixelcolor::BinaryColor,
    prelude::*,
    style::TextStyleBuilder,
    };

use ssd1306::{prelude::*, Builder, I2CDIBuilder};


#[cfg(feature = "stm32f1xx")]  //  eg blue pill stm32f103
use stm32f1xx_hal::{prelude::*,   
                    pac::Peripherals, 
                    serial::{Config, Serial, Tx, Rx},  //, StopBits
		    device::{USART3},  
                    delay::Delay,
		    i2c::{BlockingI2c, DutyCycle, Mode},
		    gpio::{gpiob::{PB8, PB9}, Alternate, OpenDrain, },
		    device::I2C1,
		    }; 

#[cfg(feature = "stm32f3xx")]  //  eg Discovery-stm32f303
use stm32f3xx_hal::{prelude::*, 
                    stm32::Peripherals,
                    serial::{ Serial, Tx, Rx},
		    stm32::{USART2}, 
                    delay::Delay,
		    i2c::{I2c, },  
		    gpio::{gpiob::{PB8, PB9}, AF4, },
		    pac::I2C1,
		    };

#[cfg(feature = "stm32f4xx")] // eg Nucleo-64, blackpills stm32f401 and stm32f411
use stm32f4xx_hal::{prelude::*,  
                    pac::Peripherals, 
                    serial::{config::Config, Serial, Tx, Rx},
		    pac::{USART2}, 
                    delay::Delay,
		    i2c::{I2c, },  
		    gpio::{gpiob::{PB8, PB9}, AlternateOD, AF4, },
                    pac::I2C1,
		    };

#[cfg(feature = "stm32l1xx") ] // eg  Discovery kit stm32l100 and Heltec lora_node STM32L151CCU6
use stm32l1xx_hal::{prelude::*, 
		    stm32::Peripherals, 
		    serial::{Config, Serial, Tx, Rx},
		    stm32::{USART2},
                    delay::Delay,
		    gpio::{gpiob::{PB8, PB9}, AlternateOD, AF4, },
                    stm32::I2C1,
		    };



#[entry]

fn main() -> ! {

    #[cfg(feature = "stm32f1xx")]
    fn setup() ->  (Tx<USART3>, Rx<USART3>,
                    BlockingI2c<I2C1,  (PB8<Alternate<OpenDrain>>, PB9<Alternate<OpenDrain>>) >, 
		    Delay )  {

       let cp = cortex_m::Peripherals::take().unwrap();
       let p = Peripherals::take().unwrap();
       let mut rcc = p.RCC.constrain();  
       let clocks = rcc.cfgr.freeze(&mut p.FLASH.constrain().acr); 
       let mut afio = p.AFIO.constrain(&mut rcc.apb2);

       let mut gpiob = p.GPIOB.split(&mut rcc.apb2);
       let (tx3, rx3) = Serial::usart3(
           p.USART3,
           (gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh),    //tx pb10  for GPS rx
            gpiob.pb11),					    //rx pb11  for GPS tx
           &mut afio.mapr,
           Config::default() .baudrate(9_600.bps()), 
           clocks,
           &mut rcc.apb1,
       ).split();

       let i2c = BlockingI2c::i2c1(
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
          );

       (tx3, rx3,   i2c,
        Delay::new(cp.SYST, clocks))
       };



    #[cfg(feature = "stm32f3xx")]
    fn setup() ->  (Tx<USART2>, Rx<USART2>, 
                   I2c<I2C1, (PB8<AF4>, PB9<AF4>)>, 
                   Delay ) {

       let cp = cortex_m::Peripherals::take().unwrap();
       let p = Peripherals::take().unwrap();
       let mut rcc = p.RCC.constrain();  
       let clocks  = rcc.cfgr.freeze(&mut p.FLASH.constrain().acr); 
       let mut gpioa = p.GPIOA.split(&mut rcc.ahb); 

       let (tx2, rx2) = Serial::usart2(
           p.USART2,
           (gpioa.pa2.into_af7(&mut gpioa.moder, &mut gpioa.afrl),    //tx pa2  for GPS rx
            gpioa.pa3.into_af7(&mut gpioa.moder, &mut gpioa.afrl)),   //rx pa3  for GPS tx
           9600.bps(),    // 115_200.bps(),
           clocks,
           &mut rcc.apb1,
           ).split();

       let mut gpiob = p.GPIOB.split(&mut rcc.ahb);

       let scl = gpiob.pb8.into_af4(&mut gpiob.moder, &mut gpiob.afrh);   // scl on PB8
       let sda = gpiob.pb9.into_af4(&mut gpiob.moder, &mut gpiob.afrh);   // sda on PB9
      
       (tx2, rx2,   
        I2c::i2c1(p.I2C1, (scl, sda), 400_000.hz(), clocks, &mut rcc.apb1 ), // i2c
        Delay::new(cp.SYST, clocks))
       };




    #[cfg(feature = "stm32f4xx")]
    fn setup() ->  (Tx<USART2>, Rx<USART2>,
                    I2c<I2C1, (PB8<AlternateOD<AF4>>, PB9<AlternateOD<AF4>>)>, 
                    Delay ) {

        let cp = cortex_m::Peripherals::take().unwrap();
        let p = Peripherals::take().unwrap();
        let clocks    =  p.RCC.constrain().cfgr.freeze();
        let gpioa = p.GPIOA.split();

        let (tx2, rx2) = Serial::usart2(
           p.USART2,
           (gpioa.pa2.into_alternate_af7(),            //tx pa2  for GPS rx
	    gpioa.pa3.into_alternate_af7()),           //rx pa3  for GPS tx
           Config::default() .baudrate(9600.bps()), 
           clocks,
           ).unwrap().split();

        let gpiob  = p.GPIOB.split();
       
       // could also have scl on PB6, sda on PB7
       //BlockingI2c::i2c1(
       let scl = gpiob.pb8.into_alternate_af4().set_open_drain();   // scl on PB8
       let sda = gpiob.pb9.into_alternate_af4().set_open_drain();   // sda on PB9
       
       (tx2, rx2,   
	I2c::i2c1(p.I2C1, (scl, sda), 400.khz(), clocks), // i2c
        Delay::new(cp.SYST, clocks))
       };




    #[cfg(feature = "stm32l1xx")]
    fn setup() ->  (Tx<USART2>, Rx<USART2>,I2c<I2C1, 
                    (PB8<AlternateOD<AF4>>, PB9<AlternateOD<AF4>>)>, 
		    Delay )  {

       let cp = cortex_m::Peripherals::take().unwrap();
       let p = Peripherals::take().unwrap();
       let clocks    =  p.RCC.constrain().cfgr.freeze();
       let gpioa = p.GPIOA.split();

       let (tx2, rx2) = Serial::usart2(
           p.USART2,
           (gpioa.pa2.into_alternate_af7(),	      //tx pa2  for GPS rx
            gpioa.pa3.into_alternate_af7()),	      //rx pa3  for GPS tx
           Config::default() .baudrate(9600.bps()), 
           clocks,
           ).unwrap().split();

       let gpiob  = p.GPIOB.split();
       
       let scl = gpiob.pb8.into_alternate_af4().set_open_drain();   // scl on PB8
       let sda = gpiob.pb9.into_alternate_af4().set_open_drain();   // sda on PB9
       
       // return i2c
       I2c::i2c1(p.I2C1, (scl, sda), 400.khz(), clocks)

       (tx2, rx2,  
        I2c::i2c1(p.I2C1, (scl, sda), 400.khz(), clocks), // i2c
        Delay::new(cp.SYST, clocks))
       };


    // End of hal/MCU specific setup. Following should be generic code.

    let (mut _tx_gps, mut rx_gps,   i2c,  mut delay) = setup();  //  GPS, i2c, delay

    let interface = I2CDIBuilder::new().init(i2c);
    let mut disp: GraphicsMode<_> = Builder::new().connect(interface).into();
    disp.init().unwrap();

    let text_style = TextStyleBuilder::new(Font8x16)
        .text_color(BinaryColor::On)
        .background_color(BinaryColor::Off)
        .build();

    //Text::new("----", Point::zero())
    //	.into_styled(text_style)
    //	.draw(&mut disp)
    //	.unwrap();
    //
    //delay.delay_ms(4000_u16);


    //disp.flush().unwrap();

    // byte buffer length 80
    let mut buffer: Vec<u8, consts::U80> = Vec::new();
    //hprintln!("buffer at {} of {}", buffer.len(), buffer.capacity()).unwrap();  //0 of 80
    buffer.clear();

    let e: u8 = 9;
    let mut good = false;
    //let mut size: usize = 0;
    
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
	   if buffer.push(byte).is_err() ||  byte == 13  { //print if end of line. \r is 13, \n is 10
              
              //hprintln!("buffer at {} of {}", buffer.len(), buffer.capacity()).unwrap();
              //hprintln!("read buffer {:?}", to_str(&buffer)).unwrap();
              
	      //hprintln!("buffer[0..6] {:?}", &buffer[0..6]).unwrap();
	      //hprintln!("buffer[0..6] {}", to_str(&buffer)).unwrap();  // message id
	      // hprintln!("buffer[0..6] {}", to_str(&buffer[0..6])).unwrap();  // message
 
              
	      //if buffer[0..6] == [36, 71, 80, 84, 88, 84] {  // "$GPTXT"
	      //if buffer[0..6] == [36, 71, 80, 82, 77, 67] {  //$GPRMC
	      
	      //$GPGLL north ~ to_str(&buffer[7..19]) east ~ to_str(&buffer[19..33])
	      //$GPRMC north = to_str(&buffer[19..31]) east = to_str(&buffer[32..45])
	      
	      if to_str(&buffer[0..6]) == "$GPRMC" {   // message id
	          let north = to_str(&buffer[19..31]);
	          hprintln!("north {}", north).unwrap();
	          let east  = to_str(&buffer[32..45]);
	          Text::new(north, Point::new(0, 0))
                      .into_styled(text_style)
                      .draw(&mut disp)
                      .unwrap();
	          Text::new(east, Point::new(0, 20))
                      .into_styled(text_style)
                      .draw(&mut disp)
                      .unwrap();
	          disp.flush().unwrap();
		  };
		  	      
              buffer.clear();
	      good = false;
              delay.delay_ms(4000_u16);
	      };
	   };
	}
}