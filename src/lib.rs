//! misc small utilities
//! eg to_str()
//!   hprintln!("Check 1 string:{}", to_str("my test string".as_bytes())).unwrap(); 
//!   hprintln!("Check 2 bytes:{:?}",       "myS test string".as_bytes()).unwrap(); 

#![deny(unsafe_code)]
#![no_std]

// extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
// extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger


//fn from_utf8_lossy...

pub fn to_str( x:&[u8] ) -> &str {
   match core::str::from_utf8(x) {
      Ok(str)     => &str,
      Err(_error) => "problem converting u8 to str ",
   }
}

//      Err(error) => panic!("problem converting u8 to str {:?}", error),

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
      assert_eq!("my first test string", to_str("my first test string".as_bytes()));
    }
}
//    #[test]
//    hprintln!("Check 3 [u8; 8] bytes:{:?}", singleton!(: [u8; 8] = [44, 186, 53, 19, 114, 54, 76, 202]).unwrap()).unwrap(); 
//    hprintln!("Check 4 [u8; 8] string:{}", to_str(singleton!(: [u8; 8] = [106, 117, 115, 116, 32, 99, 104, 101]).unwrap())).unwrap(); 
    
    // these are failing, string that cannot be converted
    //hprintln!("{}", to_str(singleton!(: [u8; 8] = [44, 186, 53, 19, 114, 54, 76, 202]).unwrap())); 
    //hprintln!("{}", to_str(singleton!(: [u8; 4] = [44, 186, 53, 19]).unwrap())); 
    //hprintln!("{}", to_str(singleton!(: [u8; 4] = [114, 54, 76, 202]).unwrap())); 
