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

// setup() does all  hal/MCU specific setup and returns generic hal device for use in main code.

#[cfg(feature = "stm32f0xx")] //  eg stm32f030xc
use stm32f0xx_hal::{
    delay::Delay,
    gpio::{
        gpioa::PA1,
        gpioa::{PA5, PA6, PA7},
        gpiob::PB1,
        Alternate, Output, PushPull, AF0,
    },
    pac::Peripherals,
    pac::SPI1,
    pac::USART3,
    prelude::*,
    serial::{Rx, Serial, Tx},
    spi::{EightBit, Spi},
};

#[cfg(feature = "stm32f0xx")]
fn setup() -> (
    Tx<USART3>,
    Rx<USART3>,
    sx127x_lora::LoRa<
        Spi<SPI1, PA5<Alternate<AF0>>, PA6<Alternate<AF0>>, PA7<Alternate<AF0>>, EightBit>,
        PA1<Output<PushPull>>,
        PB1<Output<PushPull>>,
    >,
    Delay,
) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut p = Peripherals::take().unwrap();
    let mut rcc = p.RCC.configure().freeze(&mut p.FLASH);

    let gpioa = p.GPIOA.split(&mut rcc);
    let gpiob = p.GPIOB.split(&mut rcc);

    let (tx, rx, sck, miso, mosi, cs, rst) = cortex_m::interrupt::free(move |cs| {
        (
            gpiob.pb10.into_alternate_af4(cs),   //tx pb10  for GPS
            gpiob.pb11.into_alternate_af4(cs),   //rx pb11  for GPS
            gpioa.pa5.into_alternate_af0(cs),    //   sck   on PA5
            gpioa.pa6.into_alternate_af0(cs),    //   miso  on PA6
            gpioa.pa7.into_alternate_af0(cs),    //   mosi  on PA7
            gpioa.pa1.into_push_pull_output(cs), //  cs   on PA1
            gpiob.pb1.into_push_pull_output(cs), // reset on PB1
        )
    });

    let (tx, rx) = Serial::usart3(p.USART3, (tx, rx), 9600.bps(), &mut rcc).split();

    let spi = Spi::spi1(
        p.SPI1,
        (sck, miso, mosi),
        sx127x_lora::MODE,
        8.mhz(),
        &mut rcc,
    );

    let mut delay = Delay::new(cp.SYST, &rcc);

    let lora = sx127x_lora::LoRa::new(spi, cs, rst, FREQUENCY, &mut delay).unwrap();

    (tx, rx, lora, delay)
}

#[cfg(feature = "stm32f1xx")] //  eg blue pill stm32f103
use stm32f1xx_hal::{
    delay::Delay,
    device::SPI1,
    device::USART3,
    gpio::{
        gpioa::{PA0, PA1},
        gpioa::{PA5, PA6, PA7},
        Alternate, Floating, Input, Output, PushPull,
    },
    pac::Peripherals,
    prelude::*,
    serial::{Config, Rx, Serial, Tx}, //, StopBits
    spi::{Spi, Spi1NoRemap},
};

#[cfg(feature = "stm32f1xx")]
fn setup() -> (
    Tx<USART3>,
    Rx<USART3>,
    sx127x_lora::LoRa<
        Spi<
            SPI1,
            Spi1NoRemap,
            (
                PA5<Alternate<PushPull>>,
                PA6<Input<Floating>>,
                PA7<Alternate<PushPull>>,
            ),
            u8,
        >,
        PA1<Output<PushPull>>,
        PA0<Output<PushPull>>,
    >,
    Delay,
) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let p = Peripherals::take().unwrap();
    let mut rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut p.FLASH.constrain().acr);
    let mut afio = p.AFIO.constrain(&mut rcc.apb2);

    let mut gpiob = p.GPIOB.split(&mut rcc.apb2);
    let (tx, rx) = Serial::usart3(
        p.USART3,
        (
            gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh), //tx pb10  for GPS
            gpiob.pb11,
        ), //rx pb11  for GPS
        &mut afio.mapr,
        Config::default().baudrate(9_600.bps()),
        clocks,
        &mut rcc.apb1,
    )
    .split();

    let mut gpioa = p.GPIOA.split(&mut rcc.apb2);
    let spi = Spi::spi1(
        p.SPI1,
        (
            gpioa.pa5.into_alternate_push_pull(&mut gpioa.crl), //   sck   on PA5
            gpioa.pa6.into_floating_input(&mut gpioa.crl),      //   miso  on PA6
            gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl), //   mosi  on PA7
        ),
        &mut afio.mapr,
        sx127x_lora::MODE,
        8.mhz(),
        clocks,
        &mut rcc.apb2,
    );

    let mut delay = Delay::new(cp.SYST, clocks);

    let lora = sx127x_lora::LoRa::new(
        spi,
        gpioa.pa1.into_push_pull_output(&mut gpioa.crl), //  cs   on PA1
        gpioa.pa0.into_push_pull_output(&mut gpioa.crl), // reset on PA0
        FREQUENCY,
        &mut delay,
    )
    .unwrap(); // delay
               // .expect("Failed to communicate with radio module!")

    (tx, rx, lora, delay)
}

#[cfg(feature = "stm32f3xx")] //  eg Discovery-stm32f303
use stm32f3xx_hal::{
    delay::Delay,
    gpio::{
        gpioa::{PA0, PA1},
        gpioa::{PA5, PA6, PA7},
        Output, PushPull, AF5,
    },
    prelude::*,
    serial::{Rx, Serial, Tx},
    spi::Spi,
    stm32::Peripherals,
    stm32::SPI1,
    stm32::USART2,
};

#[cfg(feature = "stm32f3xx")]
fn setup() -> (
    Tx<USART2>,
    Rx<USART2>,
    sx127x_lora::LoRa<
        Spi<SPI1, (PA5<AF5>, PA6<AF5>, PA7<AF5>)>,
        PA1<Output<PushPull>>,
        PA0<Output<PushPull>>,
    >,
    Delay,
) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let p = Peripherals::take().unwrap();
    let mut rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut p.FLASH.constrain().acr);
    let mut gpioa = p.GPIOA.split(&mut rcc.ahb);

    let (tx, rx) = Serial::usart2(
        p.USART2,
        (
            gpioa.pa2.into_af7(&mut gpioa.moder, &mut gpioa.afrl), //tx pa2  for GPS
            gpioa.pa3.into_af7(&mut gpioa.moder, &mut gpioa.afrl),
        ), //rx pa3  for GPS
        9600.bps(), // 115_200.bps(),
        clocks,
        &mut rcc.apb1,
    )
    .split();

    let spi = Spi::spi1(
        p.SPI1,
        (
            gpioa.pa5.into_af5(&mut gpioa.moder, &mut gpioa.afrl), // sck   on PA5
            gpioa.pa6.into_af5(&mut gpioa.moder, &mut gpioa.afrl), // miso  on PA6
            gpioa.pa7.into_af5(&mut gpioa.moder, &mut gpioa.afrl), // mosi  on PA7
        ),
        sx127x_lora::MODE,
        8.mhz(),
        clocks,
        &mut rcc.apb2,
    );

    let mut delay = Delay::new(cp.SYST, clocks);

    let lora = sx127x_lora::LoRa::new(
        spi,
        gpioa
            .pa1
            .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper), //  cs  on PA1
        gpioa
            .pa0
            .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper), //reset on PA0
        FREQUENCY,
        &mut delay,
    )
    .unwrap(); // delay
               // .expect("Failed to communicate with radio module!")

    (tx, rx, lora, delay)
}

#[cfg(feature = "stm32f4xx")] // eg Nucleo-64  stm32f411
use stm32f4xx_hal::{
    delay::Delay,
    gpio::{
        gpioa::{PA0, PA1},
        gpioa::{PA5, PA6, PA7},
        Alternate, Output, PushPull, AF5,
    },
    pac::Peripherals,
    pac::SPI1,
    pac::USART2,
    prelude::*,
    serial::{config::Config, Rx, Serial, Tx},
    spi::Spi,
    time::MegaHertz,
};

#[cfg(feature = "stm32f4xx")]
fn setup() -> (
    Tx<USART2>,
    Rx<USART2>,
    sx127x_lora::LoRa<
        Spi<
            SPI1,
            (
                PA5<Alternate<AF5>>,
                PA6<Alternate<AF5>>,
                PA7<Alternate<AF5>>,
            ),
        >,
        PA1<Output<PushPull>>,
        PA0<Output<PushPull>>,
    >,
    Delay,
) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let p = Peripherals::take().unwrap();
    let clocks = p.RCC.constrain().cfgr.freeze();
    let gpioa = p.GPIOA.split();

    let (tx, rx) = Serial::usart2(
        p.USART2,
        (
            gpioa.pa2.into_alternate_af7(), //tx pa2  for GPS
            gpioa.pa3.into_alternate_af7(),
        ), //rx pa3  for GPS
        Config::default().baudrate(9600.bps()),
        clocks,
    )
    .unwrap()
    .split();

    let spi = Spi::spi1(
        p.SPI1,
        (
            gpioa.pa5.into_alternate_af5(), // sck   on PA5
            gpioa.pa6.into_alternate_af5(), // miso  on PA6
            gpioa.pa7.into_alternate_af5(), // mosi  on PA7
        ),
        sx127x_lora::MODE,
        MegaHertz(8).into(),
        clocks,
    );

    let mut delay = Delay::new(cp.SYST, clocks);

    let lora = sx127x_lora::LoRa::new(
        spi,
        gpioa.pa1.into_push_pull_output(), //  cs   on PA1
        gpioa.pa0.into_push_pull_output(), // reset on PA0
        FREQUENCY,
        &mut delay,
    )
    .unwrap(); // delay

    (tx, rx, lora, delay)
}

#[cfg(feature = "stm32f7xx")]
use stm32f7xx_hal::{
    delay::Delay,
    gpio::{
        gpioa::{PA0, PA1},
        Output, PushPull,
    },
    pac::Peripherals,
    pac::SPI1,
    pac::USART2,
    prelude::*,
    serial::{Config, Oversampling, Rx, Serial, Tx},
    spi::{ClockDivider, Enabled, Pins, Spi},
};

#[cfg(feature = "stm32f7xx")]
fn setup() -> (
    Tx<USART2>,
    Rx<USART2>,
    sx127x_lora::LoRa<
        Spi<SPI1, impl Pins<SPI1>, Enabled<u8>>,
        PA1<Output<PushPull>>,
        PA0<Output<PushPull>>,
    >,
    Delay,
) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let p = Peripherals::take().unwrap();
    let mut rcc = p.RCC.constrain();

    let gpioa = p.GPIOA.split();

    //let mut ncs = gpioc.pa4.into_push_pull_output();
    let sck = gpioa.pa5.into_alternate_af5(); // sck   on PA5
    let miso = gpioa.pa6.into_alternate_af5(); // miso  on PA6
    let mosi = gpioa.pa7.into_alternate_af5(); // mosi  on PA7

    //   somewhere 8.mhz needs to be set in spi

    let spi = Spi::new(p.SPI1, (sck, miso, mosi)).enable::<u8>(
        &mut rcc,
        ClockDivider::DIV32,
        sx127x_lora::MODE,
    );

    let clocks = rcc.cfgr.sysclk(216.mhz()).freeze();

    let mut delay = Delay::new(cp.SYST, clocks);

    let lora = sx127x_lora::LoRa::new(
        spi,
        gpioa.pa1.into_push_pull_output(), //  cs   on PA1
        gpioa.pa0.into_push_pull_output(), // reset on PA0
        FREQUENCY,
        &mut delay,
    )
    .unwrap(); // delay

    let (tx, rx) = Serial::new(
        p.USART2,
        (
            gpioa.pa2.into_alternate_af7(), //tx pa2  for GPS
            gpioa.pa3.into_alternate_af7(),
        ), //rx pa3  for GPS
        clocks,
        Config {
            baud_rate: 9600.bps(),
            oversampling: Oversampling::By16,
            character_match: None,
        },
    )
    .split();

    (tx, rx, lora, delay)
}

#[cfg(feature = "stm32h7xx")]
use stm32h7xx_hal::{
    delay::Delay,
    gpio::{
        gpioa::{PA0, PA1},
        Output, PushPull,
    },
    pac::Peripherals,
    pac::SPI1,
    pac::USART2,
    prelude::*,
    serial::{Rx, Tx},
    spi::{Enabled, Spi},
};

#[cfg(feature = "stm32h7xx")]
fn setup() -> (
    Tx<USART2>,
    Rx<USART2>,
    sx127x_lora::LoRa<Spi<SPI1, Enabled>, PA1<Output<PushPull>>, PA0<Output<PushPull>>>,
    Delay,
) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let p = Peripherals::take().unwrap();
    let pwr = p.PWR.constrain();
    let vos = pwr.freeze();
    let rcc = p.RCC.constrain();
    let ccdr = rcc.sys_ck(160.mhz()).freeze(vos, &p.SYSCFG);
    let clocks = ccdr.clocks;

    let gpioa = p.GPIOA.split(ccdr.peripheral.GPIOA);

    let (tx, rx) = p
        .USART2
        .serial(
            (
                gpioa.pa2.into_alternate_af7(), //tx pa2 for GPS rx
                gpioa.pa3.into_alternate_af7(),
            ), //rx pa3 for GPS tx
            9600.bps(),
            ccdr.peripheral.USART2,
            &clocks,
        )
        .unwrap()
        .split();

    // following github.com/stm32-rs/stm32h7xx-hal/blob/master/examples/spi.rs
    let spi = p.SPI1.spi(
        (
            gpioa.pa5.into_alternate_af5(), // sck   on PA5
            gpioa.pa6.into_alternate_af5(), // miso  on PA6
            gpioa.pa7.into_alternate_af5(), // mosi  on PA7
        ),
        sx127x_lora::MODE,
        8.mhz(),
        ccdr.peripheral.SPI1,
        &clocks,
    );

    let mut delay = Delay::new(cp.SYST, clocks);

    let lora = sx127x_lora::LoRa::new(
        spi,
        gpioa.pa1.into_push_pull_output(), //  cs   on PA1
        gpioa.pa0.into_push_pull_output(), // reset on PA0
        FREQUENCY,
        &mut delay,
    )
    .unwrap(); // delay

    (tx, rx, lora, delay) // delay again
}

#[cfg(feature = "stm32l0xx")]
use stm32l0xx_hal::{
    delay::Delay,
    gpio::{
        gpioa::{PA0, PA1},
        gpioa::{PA5, PA6, PA7},
        Analog, Output, PushPull,
    },
    pac::Peripherals,
    pac::SPI1,
    pac::USART2,
    prelude::*,
    rcc, // for ::Config but note name conflict with serial
    serial::{Config, Rx, Serial2Ext, Tx},
    spi::Spi,
};

#[cfg(feature = "stm32l0xx")]
fn setup() -> (
    Tx<USART2>,
    Rx<USART2>,
    sx127x_lora::LoRa<
        Spi<SPI1, (PA5<Analog>, PA6<Analog>, PA7<Analog>)>,
        PA1<Output<PushPull>>,
        PA0<Output<PushPull>>,
    >,
    Delay,
) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let p = Peripherals::take().unwrap();
    let mut rcc = p.RCC.freeze(rcc::Config::hsi16());
    //let clocks    =  p.RCC.constrain().cfgr.freeze();
    //let clocks    =  p.RCC.constrain().cfgr.freeze();
    let gpioa = p.GPIOA.split(&mut rcc);

    let (tx, rx) = p
        .USART2
        .usart(
            gpioa.pa2, //tx pa2  for GPS
            gpioa.pa3, //rx pa3  for GPS
            Config::default().baudrate(9600.bps()),
            &mut rcc,
        )
        .unwrap()
        .split();

    // consider
    //let mut nss = gpioa.pa4.into_push_pull_output();
    //loop {
    //    nss.set_low().unwrap();
    //    spi.write(&[0, 1]).unwrap();
    //    nss.set_high().unwrap();
    //    }

    let spi = p.SPI1.spi(
        (
            gpioa.pa5, // sck   on PA5
            gpioa.pa6, // miso  on PA6
            gpioa.pa7, // mosi  on PA7
        ),
        sx127x_lora::MODE,
        8.mhz(),
        &mut rcc,
    );

    let mut delay = Delay::new(cp.SYST, rcc.clocks);

    let lora = sx127x_lora::LoRa::new(
        spi,
        gpioa.pa1.into_push_pull_output(), //  cs   on PA1
        gpioa.pa0.into_push_pull_output(), // reset on PA0
        FREQUENCY,
        &mut delay,
    )
    .unwrap(); // delay

    (tx, rx, lora, delay)
}

#[cfg(feature = "stm32l1xx")] // eg  Discovery kit stm32l100 and Heltec lora_node STM32L151CCU6
use stm32l1xx_hal::{
    delay::Delay,
    gpio::{
        gpioa::{PA0, PA1},
        Output, PushPull,
    },
    prelude::*,
    rcc, // for ::Config but note name conflict with serial
    serial::{Config, Rx, SerialExt, Tx},
    spi::{Pins, Spi},
    stm32::Peripherals,
    stm32::SPI1,
    stm32::USART1,
};

/*
The Heltec lora_node 151 uses USART2 and USART3 pins for on board LoRa connections and power detection.
See https://resource.heltec.cn/download/LoRa_Node_151/LoRa_Node_151_Pinout_Diagram.pdf.
So only USART1 is available. It is used for the GPS.
For simplicity of this example the same setup is used on the Discovery kit stm32l100.
*/

#[cfg(feature = "stm32l1xx")]
fn setup() -> (
    Tx<USART1>,
    Rx<USART1>,
    sx127x_lora::LoRa<Spi<SPI1, impl Pins<SPI1>>, PA1<Output<PushPull>>, PA0<Output<PushPull>>>,
    Delay,
) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let p = Peripherals::take().unwrap();
    let mut rcc = p.RCC.freeze(rcc::Config::hsi());

    let gpioa = p.GPIOA.split();

    let (tx, rx) = p
        .USART1
        .usart(
            (
                gpioa.pa9, //tx pa9   for GPS rx
                gpioa.pa10,
            ), //rx pa10  for GPS tx
            Config::default().baudrate(9600.bps()),
            &mut rcc,
        )
        .unwrap()
        .split();

    let spi = p.SPI1.spi(
        (
            gpioa.pa5, // sck   on PA5
            gpioa.pa6, // miso  on PA6
            gpioa.pa7, // mosi  on PA7
        ),
        sx127x_lora::MODE,
        8.mhz(),
        &mut rcc,
    );

    let mut delay = cp.SYST.delay(rcc.clocks);

    let lora = sx127x_lora::LoRa::new(
        spi,
        gpioa.pa1.into_push_pull_output(), //  cs   on PA1
        gpioa.pa0.into_push_pull_output(), // reset on PA0
        FREQUENCY,
        &mut delay,
    )
    .unwrap(); // delay

    (tx, rx, lora, delay)
}

#[cfg(feature = "stm32l4xx")]
use stm32l4xx_hal::{
    delay::Delay,
    gpio::{
        gpioa::{PA0, PA1},
        gpioa::{PA5, PA6, PA7},
        Alternate, Floating, Input, Output, PushPull, AF5,
    },
    pac::Peripherals,
    pac::SPI1,
    pac::USART2,
    prelude::*,
    serial::{Config, Rx, Serial, Tx},
    spi::Spi,
};

#[cfg(feature = "stm32l4xx")]
fn setup() -> (
    Tx<USART2>,
    Rx<USART2>,
    sx127x_lora::LoRa<
        Spi<
            SPI1,
            (
                PA5<Alternate<AF5, Input<Floating>>>,
                PA6<Alternate<AF5, Input<Floating>>>,
                PA7<Alternate<AF5, Input<Floating>>>,
            ),
        >,
        PA1<Output<PushPull>>,
        PA0<Output<PushPull>>,
    >,
    Delay,
) {
    let cp = cortex_m::Peripherals::take().unwrap();
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

    let mut gpioa = p.GPIOA.split(&mut rcc.ahb2);

    let (tx, rx) = Serial::usart2(
        p.USART2,
        (
            gpioa.pa2.into_af7(&mut gpioa.moder, &mut gpioa.afrl), //tx pa2  for GPS
            gpioa.pa3.into_af7(&mut gpioa.moder, &mut gpioa.afrl),
        ), //rx pa3  for GPS
        Config::default().baudrate(9600.bps()),
        clocks,
        &mut rcc.apb1r1,
    )
    .split();

    let spi = Spi::spi1(
        p.SPI1,
        (
            gpioa.pa5.into_af5(&mut gpioa.moder, &mut gpioa.afrl), // sck   on PA5
            gpioa.pa6.into_af5(&mut gpioa.moder, &mut gpioa.afrl), // miso  on PA6
            gpioa.pa7.into_af5(&mut gpioa.moder, &mut gpioa.afrl), // mosi  on PA7
        ),
        sx127x_lora::MODE,
        8.mhz(),
        clocks,
        &mut rcc.apb2,
    );

    let mut delay = Delay::new(cp.SYST, clocks);

    let lora = sx127x_lora::LoRa::new(
        spi,
        gpioa
            .pa1
            .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper), //  cs   on PA1
        gpioa
            .pa0
            .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper), // reset on PA0
        FREQUENCY,
        &mut delay,
    )
    .unwrap(); // delay

    (tx, rx, lora, delay)
}

// End of hal/MCU specific setup. Following should be generic code.

#[entry]

fn main() -> ! {
    let (mut _tx_gps, mut rx_gps, mut lora, _delay) = setup(); //  GPS, lora, delay

    // stm32f7xx_hal problem   lora.set_tx_power(17,1).unwrap(); //Using PA_BOOST. See your board for correct pin.

    // byte buffer length 80
    let mut buffer: Vec<u8, consts::U80> = Vec::new();
    let mut buffer2 = [0; 255]; //lora.transmit_payload() WANTS THIS SIZE, much bigger than 80 needed!

    //hprintln!("buffer at {} of {}", buffer.len(), buffer.capacity()).unwrap();  //0 of 80
    buffer.clear();

    //hprintln!("going into write/read loop ^C to exit ...").unwrap();

    let e: u8 = 9;
    let mut good = false;
    let mut size: usize;

    loop {
        let byte = match block!(rx_gps.read()) {
            Ok(byt) => byt,
            Err(_error) => e,
        };
        if byte == 36 {
            //  $ is 36. start of a line
            buffer.clear();
            good = true; //start capturing line
        };
        if good {
            if buffer.push(byte).is_err() || byte == 13 {
                //transmit if end of line. \r is 13, \n is 10

                size = buffer.len(); //packet size
                hprintln!("read buffer {} of {}", size, buffer.capacity()).unwrap();
                //hprintln!("read buffer {:?}", buffer).unwrap();

                // seems this should be unnecessary, but ...
                for i in 0..size {
                    //.chars().enumerate() {
                    buffer2[i] = buffer[i] as u8;
                }

                //hprintln!("transmit buffer2 {:?}", to_str(&buffer2)).unwrap();

                let transmit = lora.transmit_payload(&buffer2);
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
