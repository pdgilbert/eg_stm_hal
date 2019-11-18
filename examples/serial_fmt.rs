//! Formatted string serial interface write with usart1 to serial-usb converter on pin pa9 (rx is pa10).
//!
//! Connect the Tx pin pa9 to the Rx pin of a serial-usb converter
//! so you can see the message in a serial console (e.g. minicom).
//! set for 9600bps, could be higher but needs serial console to be the same.

#![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_halt as _;
use cortex_m_rt::entry;
use core::fmt::Write;

//  eg blue pill stm32f103
#[cfg(feature = "stm32f103")]
use stm32f1xx_hal::{ prelude::*,  pac,  serial::{Config, Serial}, };

//  eg Discovery-stm32f303
//use alt_stm32f30x_hal::{  ??
#[cfg(feature = "stm32f303")]
use stm32f3xx_hal::{ prelude::*, pac, serial::{Config, Serial}, };

// eg Nucleo-64  stm32f411
#[cfg(feature = "stm32f411")]
use stm32f4xx_hal::{ prelude::*, pac, serial::{Config, Serial}, };

// eg  Discovery kit stm32l100 and Heltec lora_node STM32L151CCU6
#[cfg(any(feature = "stm32l100",   feature = "stnm32l151" )) ]
use stm32l1xx_hal::{ prelude::*, pac, serial::{Config, Serial}, };


#[entry]
fn main() -> ! {
    // Get access to the device specific peripherals from the peripheral access crate
    let p = pac::Peripherals::take().unwrap();

    // Take ownership of raw flash and rcc devices and convert to HAL structs
    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();

    // Freeze  all system clocks  and store the frozen frequencies in `clocks`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Prepare the alternate function I/O registers
    let mut afio = p.AFIO.constrain(&mut rcc.apb2);

    // Prepare the GPIOB peripheral
    let mut gpioa = p.GPIOA.split(&mut rcc.apb2);
    // let mut gpiob = p.GPIOB.split(&mut rcc.apb2);

    //see examples/serial_loopback_char_test.rs for more USART config notes.

    let serial = Serial::usart1(
        p.USART1,
        (gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh),  gpioa.pa10),
        &mut afio.mapr,
        Config::default().baudrate(9600.bps()),
        clocks,
        &mut rcc.apb2,
    );

    // Split the serial struct into a receiving and a transmitting part
    let (mut tx, _rx) = serial.split();

    let number = 42;
    writeln!(tx, "\r\nHello {}. Converted number set to 42.\r\n", number).unwrap();

    loop {}
}
