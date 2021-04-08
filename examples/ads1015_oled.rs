//! Using ads1x1x to read voltages measured by ads1015/1115 ADC on i2c
//! and  crate ssd1306 to print with i2c on a generic ssd1306 based OLED display.
//!
//! Uses the `embedded_graphics` crate to draw.
//! Wiring pin connections for scl and sda to display as in the setup sections below.
//! (not yet Tested on generic (cheap) ssd1306 OLED 0.91" 128x32 and 0.96" 128x64 displays. )
//! Note that the DisplaySize setting needs to be adjusted for 128x64 or 128x32 display
//!
//! This example based on
//!    https://github.com/jamwaffles/ssd1306/blob/master/examples/text_i2c.rs  and
//!    https://github.com/eldruin/driver-examples/stm32f1-bluepill/examples/ads1015-adc-display-bp.rs
//!  See https://blog.eldruin.com/ads1x1x-analog-to-digital-converter-driver-in-rust/
//!    for much more detailed description.

#![deny(unsafe_code)]
#![no_std]
#![no_main]

#[cfg(debug_assertions)]
use panic_semihosting as _;

#[cfg(not(debug_assertions))]
use panic_halt as _;

use cortex_m_rt::entry;

use ads1x1x::{channel as AdcChannel, Ads1x1x, FullScaleRange, SlaveAddr};

use core::fmt::Write;
use embedded_graphics::{
    fonts::{Font8x16, Text}, //Font6x8,
    pixelcolor::BinaryColor,
    prelude::*,
    style::TextStyleBuilder,
};

use ssd1306::{prelude::*, Builder, I2CDIBuilder};

use embedded_hal::digital::v2::OutputPin;
use nb::block;

// setup() does all  hal/MCU specific setup and returns generic hal device for use in main code.

#[cfg(feature = "stm32f0xx")] //  eg stm32f030xc
use stm32f0xx_hal::{
    delay::Delay,
    gpio::{
        gpiob::{PB7, PB8},
        gpioc::PC13,
        Alternate, Output, PushPull, AF1,
    },
    i2c::I2c,
    pac::{CorePeripherals, Peripherals, I2C1},
    prelude::*,
};

#[cfg(feature = "stm32f0xx")]
fn setup() -> (
    I2c<I2C1, PB8<Alternate<AF1>>, PB7<Alternate<AF1>>>,
    PC13<Output<PushPull>>,
    Delay,
) {
    let cp = CorePeripherals::take().unwrap();
    let mut p = Peripherals::take().unwrap();

    let mut rcc = p.RCC.configure().freeze(&mut p.FLASH);

    let gpiob = p.GPIOB.split(&mut rcc); // for i2c scl and sda

    let (scl, sda) = cortex_m::interrupt::free(move |cs| {
        (
            gpiob.pb8.into_alternate_af1(cs), // scl on PB8
            gpiob.pb7.into_alternate_af1(cs), // sda on PB7
        )
    });

    let i2c = I2c::i2c1(p.I2C1, (scl, sda), 400.khz(), &mut rcc);

    // led
    let gpioc = p.GPIOC.split(&mut rcc);
    let led = cortex_m::interrupt::free(move |cs| gpioc.pc13.into_push_pull_output(cs));

    (i2c, led, Delay::new(cp.SYST, &rcc)) // return tuple (i2c, led, delay)
}

#[cfg(feature = "stm32f1xx")] //  eg blue pill stm32f103
use stm32f1xx_hal::{
    delay::Delay,
    device::I2C1,
    gpio::{
        gpiob::{PB8, PB9},
        gpioc::PC13,
        Alternate, OpenDrain, Output, PushPull,
    },
    i2c::{BlockingI2c, DutyCycle, Mode},
    pac::{CorePeripherals, Peripherals},
    prelude::*,
};

#[cfg(feature = "stm32f1xx")]
fn setup() -> (
    BlockingI2c<I2C1, (PB8<Alternate<OpenDrain>>, PB9<Alternate<OpenDrain>>)>,
    PC13<Output<PushPull>>,
    Delay,
) {
    let cp = CorePeripherals::take().unwrap();
    let p = Peripherals::take().unwrap();

    let mut rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut p.FLASH.constrain().acr);

    let mut gpiob = p.GPIOB.split(&mut rcc.apb2); // for i2c scl and sda

    // can have (scl, sda) using I2C1  on (PB8, PB9 ) or on  (PB6, PB7)
    //     or   (scl, sda) using I2C2  on (PB10, PB11)

    let i2c = BlockingI2c::i2c1(
        p.I2C1,
        (
            gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh), // i2c scl on pb8
            gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh), // i2c sda on pb9
        ),
        &mut p.AFIO.constrain(&mut rcc.apb2).mapr,
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
    let mut gpioc = p.GPIOC.split(&mut rcc.apb2);
    let led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh); // led on pc13

    (i2c, led, Delay::new(cp.SYST, clocks)) // return tuple (i2c, led, delay)
}

#[cfg(feature = "stm32f3xx")] //  eg Discovery-stm32f303
use stm32f3xx_hal::{
    delay::Delay,
    gpio::{
        gpiob::{PB8, PB9},
        gpioe::PE15,
        Alternate, OpenDrain, Output, Pin, PushPull, AF4,
    },
    hal::blocking::i2c::{Read, WriteRead},
    i2c,
    pac::{CorePeripherals, Peripherals, I2C1},
    prelude::*,
};

#[cfg(feature = "stm32f3xx")]
fn setup() -> (impl WriteRead, PE15<Output<PushPull>>, Delay) {
    let cp = CorePeripherals::take().unwrap();
    let p = Peripherals::take().unwrap();

    let mut rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut p.FLASH.constrain().acr);

    let mut gpiob = p.GPIOB.split(&mut rcc.ahb); // for i2c

    let mut scl =
        gpiob
            .pb8
            .into_af4_open_drain(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrh); // scl on PB8
    let mut sda =
        gpiob
            .pb9
            .into_af4_open_drain(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrh); // sda on PB9

    // not sure if pull up is needed
    scl.internal_pull_up(&mut gpiob.pupdr, true);
    sda.internal_pull_up(&mut gpiob.pupdr, true);

    let i2c = i2c::I2c::new(
        p.I2C1,
        (scl, sda),
        //&mut afio.mapr,  need this for i2c1 but not i2c2
        400_000.Hz(),
        //100u32.kHz().try_into().unwrap(),
        clocks,
        &mut rcc.apb1,
    );

    // led
    let mut gpioe = p.GPIOE.split(&mut rcc.ahb);
    let led = gpioe
        .pe15
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper); // led on pe15

    (i2c, led, Delay::new(cp.SYST, clocks)) // return tuple (i2c, led, delay)
}

#[cfg(feature = "stm32f4xx")] // eg Nucleo-64  stm32f411
use stm32f4xx_hal::{
    delay::Delay,
    gpio::{
        gpiob::{PB10, PB3},
        gpioc::PC13,
        AlternateOD, Output, PushPull, AF4, AF9,
    },
    i2c::I2c,
    pac::{CorePeripherals, Peripherals, I2C2},
    prelude::*,
};

#[cfg(feature = "stm32f4xx")]
fn setup() -> (
    I2c<I2C2, (PB10<AlternateOD<AF4>>, PB3<AlternateOD<AF9>>)>,
    PC13<Output<PushPull>>,
    Delay,
) {
    let cp = CorePeripherals::take().unwrap();
    let p = Peripherals::take().unwrap();

    let rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.freeze();

    let gpiob = p.GPIOB.split(); // for i2c

    // can have (scl, sda) using I2C1  on (PB8  _af4, PB9 _af4) or on  (PB6 _af4, PB7 _af4)
    //     or   (scl, sda) using I2C2  on (PB10 _af4, PB3 _af9)

    let scl = gpiob.pb10.into_alternate_af4().set_open_drain(); // scl on PB10
    let sda = gpiob.pb3.into_alternate_af9().set_open_drain(); // sda on PB3

    let i2c = I2c::new(p.I2C2, (scl, sda), 400.khz(), clocks);

    // led
    let gpioc = p.GPIOC.split();
    let led = gpioc.pc13.into_push_pull_output();

    (i2c, led, Delay::new(cp.SYST, clocks)) // return tuple (i2c, led, delay)
}

#[cfg(feature = "stm32f7xx")]
use stm32f7xx_hal::{
    delay::Delay,
    gpio::{gpioc::PC13, Output, PushPull},
    i2c::{BlockingI2c, Mode, PinScl, PinSda},
    pac::{CorePeripherals, Peripherals, I2C1},
    prelude::*,
};

#[cfg(feature = "stm32f7xx")]
fn setup() -> (
    BlockingI2c<I2C1, impl PinScl<I2C1>, impl PinSda<I2C1>>,
    PC13<Output<PushPull>>,
    Delay,
) {
    let cp = CorePeripherals::take().unwrap();
    let p = Peripherals::take().unwrap();
    let mut rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.freeze();

    let gpiob = p.GPIOB.split();
    let gpioc = p.GPIOC.split();

    let scl = gpiob.pb8.into_alternate_af4().set_open_drain(); // scl on PB8
    let sda = gpiob.pb9.into_alternate_af4().set_open_drain(); // sda on PB9

    let i2c = BlockingI2c::i2c1(
        p.I2C1,
        (scl, sda),
        //400.khz(),
        Mode::Fast {
            frequency: 400_000.hz(),
        },
        clocks,
        &mut rcc.apb1,
        1000,
    );

    let led = gpioc.pc13.into_push_pull_output(); // led on pc13

    (i2c, led, Delay::new(cp.SYST, clocks)) // return tuple (i2c, led, delay)
}

#[cfg(feature = "stm32h7xx")]
use stm32h7xx_hal::{
    delay::Delay,
    gpio::{gpioc::PC13, Output, PushPull},
    i2c::I2c,
    //gpio::{gpiob::{PB8, PB9}, Alternate, AF4, }, really! builds without this
    pac::{CorePeripherals, Peripherals, I2C1},
    prelude::*,
};

#[cfg(feature = "stm32h7xx")]
fn setup() -> (I2c<I2C1>, PC13<Output<PushPull>>, Delay) {
    let cp = CorePeripherals::take().unwrap();
    let p = Peripherals::take().unwrap();
    let pwr = p.PWR.constrain();
    let vos = pwr.freeze();
    let rcc = p.RCC.constrain();
    let ccdr = rcc.sys_ck(160.mhz()).freeze(vos, &p.SYSCFG);
    let clocks = ccdr.clocks;

    let gpiob = p.GPIOB.split(ccdr.peripheral.GPIOB);
    let gpioc = p.GPIOC.split(ccdr.peripheral.GPIOC);

    let scl = gpiob.pb8.into_alternate_af4().set_open_drain(); // scl on PB8
    let sda = gpiob.pb9.into_alternate_af4().set_open_drain(); // sda on PB9

    let i2c = p
        .I2C1
        .i2c((scl, sda), 400.khz(), ccdr.peripheral.I2C1, &clocks);

    let led = gpioc.pc13.into_push_pull_output(); // led on pc13

    (i2c, led, Delay::new(cp.SYST, clocks)) // return tuple (i2c, led, delay)
}

#[cfg(feature = "stm32l0xx")]
use stm32l0xx_hal::{
    delay::Delay,
    gpio::{
        gpiob::{PB8, PB9},
        gpioc::PC13,
        OpenDrain, Output, PushPull,
    },
    i2c::I2c,
    pac::{CorePeripherals, Peripherals, I2C1},
    prelude::*,
    rcc, // for ::Config but note name conflict with serial
};

#[cfg(feature = "stm32l0xx")]
fn setup() -> (
    I2c<I2C1, PB9<Output<OpenDrain>>, PB8<Output<OpenDrain>>>,
    PC13<Output<PushPull>>,
    Delay,
) {
    let cp = CorePeripherals::take().unwrap();
    let p = Peripherals::take().unwrap();
    let mut rcc = p.RCC.freeze(rcc::Config::hsi16());

    let gpiob = p.GPIOB.split(&mut rcc);
    let gpioc = p.GPIOC.split(&mut rcc);

    // could also have scl on PB6, sda on PB7
    //BlockingI2c::i2c1(
    let scl = gpiob.pb8.into_open_drain_output(); // scl on PB8
    let sda = gpiob.pb9.into_open_drain_output(); // sda on PB9

    let i2c = p.I2C1.i2c(sda, scl, 400.khz(), &mut rcc);

    let led = gpioc.pc13.into_push_pull_output(); // led on pc13 with on/off

    (i2c, led, Delay::new(cp.SYST, rcc.clocks)) // return tuple (i2c, led, delay)
}

#[cfg(feature = "stm32l1xx")] // eg  Discovery STM32L100 and Heltec lora_node STM32L151CCU6
use stm32l1xx_hal::{
    delay::Delay,
    gpio::{gpiob::PB6, Output, PushPull},
    i2c::{I2c, Pins},
    prelude::*,
    rcc, // for ::Config but avoid name conflict with serial
    stm32::{CorePeripherals, Peripherals, I2C1},
    //gpio::{gpiob::{PB8, PB9}, Output, OpenDrain, },
};

#[cfg(feature = "stm32l1xx")]
fn setup() -> (I2c<I2C1, impl Pins<I2C1>>, PB6<Output<PushPull>>, Delay) {
    let cp = CorePeripherals::take().unwrap();
    let p = Peripherals::take().unwrap();
    let mut rcc = p.RCC.freeze(rcc::Config::hsi());

    let gpiob = p.GPIOB.split();

    // could also have scl,sda  on PB6,PB7 or on PB10,PB11
    let scl = gpiob.pb8.into_open_drain_output(); // scl on PB8
    let sda = gpiob.pb9.into_open_drain_output(); // sda on PB9

    let i2c = p.I2C1.i2c((scl, sda), 400.khz(), &mut rcc);

    let led = gpiob.pb6.into_push_pull_output(); // led on pb6

    (i2c, led, Delay::new(cp.SYST, rcc.clocks)) // return tuple (i2c, led, delay)
}

#[cfg(feature = "stm32l4xx")]
use stm32l4xx_hal::{
    delay::Delay,
    gpio::{
        gpiob::{PB10, PB11},
        gpioc::PC13,
        Alternate, OpenDrain, Output, PushPull, AF4,
    },
    i2c::I2c,
    pac::{CorePeripherals, Peripherals, I2C2},
    prelude::*,
};

#[cfg(feature = "stm32l4xx")]
fn setup() -> (
    I2c<
        I2C2,
        (
            PB10<Alternate<AF4, Output<OpenDrain>>>,
            PB11<Alternate<AF4, Output<OpenDrain>>>,
        ),
    >,
    PC13<Output<PushPull>>,
    Delay,
) {
    let cp = CorePeripherals::take().unwrap();
    let p = Peripherals::take().unwrap();
    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();
    let mut pwr = p.PWR.constrain(&mut rcc.apb1r1);
    let clocks = rcc
        .cfgr
        .sysclk(80.mhz())
        .pclk1(80.mhz())
        .pclk2(80.mhz())
        .freeze(&mut flash.acr, &mut pwr);

    let mut gpiob = p.GPIOB.split(&mut rcc.ahb2);

    // following ttps://github.com/stm32-rs/stm32l4xx-hal/blob/master/examples/i2c_write.rs
    let mut scl = gpiob
        .pb10
        .into_open_drain_output(&mut gpiob.moder, &mut gpiob.otyper); // scl on PB10
    scl.internal_pull_up(&mut gpiob.pupdr, true);
    let scl = scl.into_af4(&mut gpiob.moder, &mut gpiob.afrh);

    let mut sda = gpiob
        .pb11
        .into_open_drain_output(&mut gpiob.moder, &mut gpiob.otyper); // sda on PB11
    sda.internal_pull_up(&mut gpiob.pupdr, true);
    let sda = sda.into_af4(&mut gpiob.moder, &mut gpiob.afrh);

    let i2c = I2c::i2c2(p.I2C2, (scl, sda), 400.khz(), clocks, &mut rcc.apb1r1);

    let mut gpioc = p.GPIOC.split(&mut rcc.ahb2);
    let led = gpioc
        .pc13
        .into_push_pull_output(&mut gpioc.moder, &mut gpioc.otyper); // led on pc13

    (i2c, led, Delay::new(cp.SYST, clocks)) // return tuple (i2c, led, delay)
}

// End of hal/MCU specific setup. Following should be generic code.

//fn blinkOk() -> (){
//}

#[entry]
fn main() -> ! {
    let (i2c, mut led, mut delay) = setup();

    let manager = shared_bus::BusManager::<cortex_m::interrupt::Mutex<_>, _>::new(i2c);
    let interface = I2CDIBuilder::new().init(manager.acquire());
    let mut disp: GraphicsMode<_, _> = Builder::new()
        .size(DisplaySize128x64) // set display size 128x32, 128x64
        .connect(interface)
        .into();
    disp.init().unwrap();

    //    let text_style = TextStyleBuilder::new(Font6x8)
    //        .text_color(BinaryColor::On)
    //        .build();
    let text_style = TextStyleBuilder::new(Font8x16)
        .text_color(BinaryColor::On)
        .build();

    Text::new("Display initialized ...", Point::zero())
        .into_styled(text_style)
        .draw(&mut disp)
        .unwrap();
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

        disp.clear();
        // write some extra spaces after the number to clear up when the numbers get smaller
        for i in 0..values.len() {
            write!(lines[i], "Channel {}: {}    ", i, values[i]).unwrap();
            //disp.draw(
            //    Font6x8::render_str(&lines[i])
            //        .with_stroke(Some(1u8.into()))
            //        .translate(Coord::new(0, i as i32 * 16))
            //        .into_iter(),
            //);
            Text::new(&lines[i], Point::new(0, i as i32 * 16))
                .into_styled(text_style)
                .draw(&mut disp)
                .unwrap();
        }
        disp.flush().unwrap();
    }
}
