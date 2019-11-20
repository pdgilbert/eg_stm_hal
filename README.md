# eg_stm_hal
Newbie notes - examples using embedded Rust HAL

(My development environment is Linux, so these notes are specific to that 
in many places. However, the examples should work in other development 
environments. Just the setup may change.)

##  Status
work in progress ...

The overall Tracis CI build status is [![Build Status](https://travis-ci.org/pdgilbert/eg_stm_hal.svg?branch=master)](https://travis-ci.org/pdgilbert/eg_stm_hal)
Status for [individual boards can be seen the Travis CI.](https://travis-ci.org/pdgilbert/eg_stm_hal)
Testing if the code runs and does something resembling what it is supposed to do requires hardware and
is not as automatic as CI. This is my summary as of November 2019.

|      HAL       |    MCU    |      Board          |   Builds   |  Runs  |          Notes                            |
| -------------- |:---------:|:-------------------:|:----------:|:------:| :---------------------------------------- |
| stm32f1xx-hal  | stm32f103 |      bluepill       |   mostly   |  some  | Problems using serial.                    |
| stm32f3xx-hal  | stm32f303 | discovery-stm32f303 |    no      |   no   | Hal differences. no `pac` in the root, ...|
| stm32f4xx-hal  | stm32f411 |      nucleo-64      |    no      |   no   | Hal differences. no `pac` in the root, ...|
| stm32l1xx-hal  | stm32l100 | discovery-stm32l100 |    no      |   no   | Hal does not build.                       |
| stm32l1xx-hal  | stm32l151 | heltec-lora-node151 |    no      |   no   | Hal does not build.                       |


|   HAL git       |      HAL Travis CI  Status         | 
| -----------------------  |:---------------------------:|
| [stm32f1xx-hal](https://github.com/stm32-rs/stm32f1xx-hal) | Build Status |
| [stm32f3xx-hal](https://github.com/stm32-rs/stm32f3xx-hal) | Build Status |
| [stm32f4xx-hal](https://github.com/stm32-rs/stm32f4xx-hal) 
   | [![Build Status](https://api.travis-ci.org/stm32-rs/stm32f4xx-hal.svg?branch=master)]
       (https://api.travis-ci.org/stm32-rs/stm32f4xx-hal) |
| [stm32l1xx-hal](https://github.com/stm32-rs/stm32l1xx-hal) | Build Status |

##  This Package Setup
I am trying to have a common code base of examples that run on different boards.
(This may be wishful thinking.) I have still not decided the best way to 
organize this for Cargo. Workspaces do not seem to be intended for this.
My current setup is to have common files src/, examples/, ..., at the top level.
Then, under boards/,  use soft links to the common files.

##  Notes on ...

##  Links
The CI for several rust embedded projects is [here.](https://travis-ci.org/rust-embedded)

[HALs on Github](https://github.com/stm32-rs) and on [Travis CI](https://travis-ci.org/stm32-rs)
.
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
