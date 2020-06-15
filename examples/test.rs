//! Just misc testing

#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[cfg(debug_assertions)]
extern crate panic_semihosting;

#[cfg(not(debug_assertions))]
extern crate panic_halt;

//use cortex_m::asm;
use cortex_m_rt::entry;
//use core::fmt::Write;
use cortex_m_semihosting::hprintln;
use core::str;
//use nb::block;

use cortex_m::singleton;

use heapless::{consts, Vec};

use eg_stm_hal::to_str;

#[cfg(feature = "stm32f1xx")]  //  eg blue pill stm32f103
use stm32f1xx_hal::{prelude::*,   pac::Peripherals, serial::{Config, Serial, StopBits}, };

#[cfg(feature = "stm32f3xx")]  //  eg Discovery-stm32f303
use stm32f3xx_hal::{prelude::*, stm32::Peripherals, serial::{ Serial}, };

#[cfg(feature = "stm32f4xx")] // eg Nucleo-64  stm32f411
use stm32f4xx_hal::{prelude::*, pac::Peripherals, serial::{config::Config, Serial }};

#[cfg(feature = "stm32l1xx") ] // eg  Discovery kit stm32l100 and Heltec lora_node STM32L151CCU6
use stm32l1xx_hal::{prelude::*, stm32::Peripherals, serial::{config::Config, Serial }};

// see https://doc.rust-lang.org/std/string/struct.String.html#method.as_bytes
//     https://doc.rust-lang.org/std/str/fn.from_utf8.html
// A string slice (&str) is made of bytes (u8), and a byte slice (&[u8]) is made of bytes

#[entry]
fn main() -> ! {

    //  Various attempts to convert  str to byte to str.
    
    let r = b'X';
    hprintln!("{}", r ).unwrap();             //88
    hprintln!("{}", u8::from(r) ).unwrap();   //88
    hprintln!("{}", r as u8 ).unwrap();       //88
    hprintln!("{}", r as u16 ).unwrap();      //88
    hprintln!("{}", str::from_utf8(&[r]).unwrap()).unwrap();  //X

    let r = b"Y";
    hprintln!("{:?}", r ).unwrap();  //[89]
    //hprintln!("{}", r ).unwrap();            [u8; 1]` cannot be formatted with the default formatter
    //hprintln!("{}", u8::from(r) );  trait bound ...not satisfied
    //hprintln!("{}", r as u8 );      casting `&[u8; 1]` as `u8` is invalid
    //hprintln!("{}", r as u16 );     casting `&[u8; 1]` as `u16` is invalid
    //hprintln!("{}", str::from_utf8(&[r]).unwrap()).unwrap();  // expected u8, found &[u8; 1]
    for byte in r.iter() {
        //hprintln!("{}", str::from_utf8(&[byte]).unwrap()).unwrap(); // expected u8, found &u8
        //hprintln!("{}", str::from_utf8([byte]).unwrap()).unwrap();  // expected &[u8], found array of 1 elements
        //hprintln!("{}", str::from_utf8(*byte).unwrap()).unwrap();   // expected &[u8], found u8
        //hprintln!("{}", str::from_utf8(byte).unwrap()).unwrap();    // expected slice, found u8
        //hprintln!("{}", str::from_utf8(u8::from(*byte)).unwrap()).unwrap(); // expected &[u8], found u8
        //usart1.tdr.write(|w| w.tdr().bits(u16::from(*byte)));
        hprintln!("{}", str::from_utf8(&[u8::from(*byte)]).unwrap()).unwrap(); // Y
    }

    let txt ="XYZ";
    let r = txt.as_bytes();
    assert_eq!( r, b"XYZ");
    //assert_eq!( "XYZ", str::from_utf8(r) ); //no implementation 

    hprintln!("{}", txt ).unwrap();  //XYZ
    hprintln!("{}", txt.len() ).unwrap();  //3

    hprintln!("{:?}", r ).unwrap();  //[88, 89, 90]
    hprintln!("{}", r.len() ).unwrap();  //3

    let rr = str::from_utf8(r); 
    hprintln!("{:?}", rr ).unwrap();  // Ok("XYZ")  // needs {:?}
    
    let rr = match str::from_utf8(r) {  
       Ok(u8)     => u8,          // works here but need to be done differently in function below
       Err(error) => panic!("problem converting u8 to str {:?}", error),
       };

    hprintln!("{}", rr ).unwrap();  // XYZ

    for byte in r.iter() {
        hprintln!("{}", str::from_utf8(&[u8::from(*byte)]).unwrap()).unwrap(); //X Y Z on separate lines
    }

    let mut buffer: Vec<u8, consts::U32> = Vec::new();
    hprintln!("buffer at {} of {}", buffer.len(), buffer.capacity()).unwrap();  //0 of 32
    buffer.clear();
    for byte in r.iter() {
            if buffer.push(*byte).is_err() { // buffer full
                hprintln!("buffer full").unwrap(); 
                }
            }
    hprintln!("{:?}", buffer ).unwrap();  //[88, 89, 90]
    hprintln!("{:#?}", buffer ).unwrap();  //[88, 89, 90] on separate lines
    hprintln!("{}", to_str(&buffer) ).unwrap();  //XYZ

    //asm::bkpt();

    // check buffer full error

    let mut buffer: Vec<u8, consts::U8> = Vec::new();
    hprintln!("buffer at {} of {}", buffer.len(), buffer.capacity()).unwrap();  //0 of 8
    buffer.clear();
    for byte in "1234567890".as_bytes().iter() {
            if buffer.push(*byte).is_err() { // buffer full
                hprintln!("buffer full").unwrap(); 
                }
            }
    hprintln!("{}", to_str(&buffer) ).unwrap();  //XYZ

    let mut buffer : Vec<u8, consts::U8> = Vec::new();
    //no let mut buffer = singleton!(: [u8; 8] = [0; 8]).unwrap();
    hprintln!("buffer at {} of {}", buffer.len(), buffer.capacity()).unwrap();  //0 of 8
    buffer.clear();
    let r = "1234567890".as_bytes();
    let mut i = 0;
    while !buffer.push(r[i]).is_err() {
        i += 1 ;
	}
    hprintln!("buffer at {} of {}", buffer.len(), buffer.capacity()).unwrap();  //8 of 8
    hprintln!("{}", to_str(&buffer) ).unwrap();  //XYZ

    let mut buffer : Vec<u8, consts::U8> = Vec::new();
    //no let mut buffer = singleton!(: [u8; 8] = [0; 8]).unwrap();
    hprintln!("buffer at {} of {}", buffer.len(), buffer.capacity()).unwrap();  //0 of 8
    buffer.clear();
    let r = "$234\n\r7".as_bytes();
    let mut i = 0;
    // note this is < not <= since && does both sides , so last r[i] gets pushed
    // so this puts \n\r on the end of buffer.
    while (i < r.len()) && !buffer.push(r[i]).is_err() {
        if r[i] == 13  { break; }   // "\r".as_bytes() is 13, \n is 10
	i += 1 ;
	}
    hprintln!("{:?}", buffer ).unwrap();  
    hprintln!("buffer at {} of {}", buffer.len(), buffer.capacity()).unwrap();  //6 of 8
    hprintln!("{}", to_str(&buffer) ).unwrap();  //XYZ
   
    //for byte in b"error: buffer full\n\r" {
    //    while usart1.isr.read().txe().bit_is_clear() {}
    //    usart1.tdr.write(|w| w.tdr().bits(u16::from(*byte)));

    loop {}
}
