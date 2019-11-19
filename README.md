# eg_stm_hal
Newbie notes - examples using embedded Rust HAL

(My development environment is Linux, so these notes are specific to that 
in many places. However, the examples should work in other development 
environments. Just the setup may change.)

##  Status
work in progress ...

The Tracis CI is at (https://travis-ci.org/pdgilbert/eg_stm_hal)

|      HAL       |      Board          | Build Status     |
| -------------- |:-------------------:|:---------------- |
| stm32f1xx-hal  |     bluepill        | [![Build Status](https://travis-ci.org/pdgilbert/eg_stm_hal.svg?branch=master)](https://travis-ci.org/pdgilbert/eg_stm_hal) |

| stm32f3xx-hal  | STM32F303 Discovery | [![Build Status](https://travis-ci.org/pdgilbert/eg_stm_hal.svg?branch=master)](https://travis-ci.org/pdgilbert/eg_stm_hal) |


##  This Package Setup
I am trying to have a common code base of examples that run on different boards.
(This may be wishful thinking.) I have still not decided the best way to 
organize this for Cargo. Workspaces do not seem to be intended for this.
My current setup is to have common files src/, examples/, ..., at the top level.
Then, under boards/,  use soft links to the common files.

##  Notes on ...

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
