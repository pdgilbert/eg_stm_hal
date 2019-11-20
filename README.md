# Examples Using embedded Rust HAL [![Build Status](https://travis-ci.org/pdgilbert/eg_stm_hal.svg?branch=master)](https://travis-ci.org/pdgilbert/eg_stm_hal)

This project's overall Travis CI build status is above.
[Status for individual boards described below can be 
seen at Travis CI.](https://travis-ci.org/pdgilbert/eg_stm_hal)

## Preamble
These are newbie notes. I really am a newbie to embedded programming and to Rust. 
This is an attempt to organize some notes made while trying to figure things out.
I have put them here in the hope they will be useful to others. More importantly for me,
I am also trying to keep track of what works and what does not, in a way that can be
kept up to date relatively easily. I have found a lot of confusing out-of-date information
on the web, so my hope is that the CI links here will warn readers when this project 
becomes old and broken.

Rust seems to have many attractive features compared to more mature languages.
It has a modern packaging system which encourages documentation and testing.
The language elements are designed so the compiler can catch many errors,
which is frustrating while learning the language but will be a substantial time saving
compared to run time debugging. The downside is that Rust is newer than many alternatives,
embedded Rust is even newer, and the hardware abstraction library (HAL) project is in 
active development.

So, this is not yet easy territory for faint of heart newbies, or anyone on a strict timeline.
For me, having spent many years dealing with cross platform desktop problems, 
HAL just makes a lot of sense.

After awhile things that were a time consuming stumbling block become so obvious that
it no longer seems necessary to mention them. That is why it is so hard to write beginner
documention, not to mention programmers' general reluctance to document anything.
Please enter an issue if you think there is something that really needs to be clarified or added.

(My development environment is Linux, so these notes are specific to that 
in many places. However, the examples should work in other development 
environments. Just the setup may change.)

##  Contents
- [Status Summary](#status-summary)
- [This Package Setup](#this-package-setup)
- [Notes on the Examples](#notes-on-the-examples)
- [Hardware Notes](#hardware-notes)
- [Misc Notes on STlink and OpenOCD](#misc-notes-on-stlink-and-openocd)
- [Misc Install Notes](#misc-install-notes)
- [Links](#Links)
- [License](#License)
- [Contribution](#Contribution)
- [](## )
- [](## )


##  Status Summary
 (November 19, 2019) work in progress ...

The overall Travis CI build status and the link for individual boards is given above.
Testing if the code runs and does something resembling what it is supposed to do 
requires hardware and is not as automatic as CI. 
This is my summary as of November 2019. Boards indicates as in 'none-' mean that I do not
have hardware to check this MCU. If you check the examples using one of these MCUs 
then please provide details 
using [issues](https://github.com/pdgilbert/eg_stm_hal/issues) on the git project page.

|      HAL       |    MCU    |      Board          |   Builds   |  Runs  |          Notes                            |
| -------------- |:---------:|:-------------------:|:----------:|:------:| :---------------------------------------- |
| stm32f1xx-hal  | stm32f103 |      bluepill       |   mostly   |  some  | Problems using serial.                    |
| stm32f1xx-hal  | stm32f100 |   none-stm32f100    |   mostly   |   NA   |                                           |
| stm32f3xx-hal  | stm32f303 | discovery-stm32f303 |    no      |   no   | Hal differences. no `pac` in the root, ...|
| stm32f4xx-hal  | stm32f411 |      nucleo-64      |    no      |   no   | Hal differences. no `pac` in the root, ...|
| stm32l1xx-hal  | stm32l100 | discovery-stm32l100 |    no      |   no   | Hal does not build.                       |
| stm32l1xx-hal  | stm32l151 | heltec-lora-node151 |    no      |   no   | Hal does not build.                       |


This projects examples depend on these HALs. 
See [HALs on Github](https://github.com/stm32-rs) and on [Travis CI.](https://travis-ci.org/stm32-rs)

|   HAL git                         |       HAL Travis CI  Status           | 
|:---------------------------------:|:-------------------------------------:|
| [stm32f1xx-hal](https://github.com/stm32-rs/stm32f1xx-hal) | [![Build Status](https://travis-ci.com/stm32-rs/stm32f1xx-hal.svg?branch=master)](https://travis-ci.com/stm32-rs/stm32f1xx-hal) |
| [stm32f3xx-hal](https://github.com/stm32-rs/stm32f3xx-hal) | [![Build Status](https://travis-ci.com/stm32-rs/stm32f3xx-hal.svg?branch=master)](https://travis-ci.com/stm32-rs/stm32f3xx-hal) |
| [stm32f4xx-hal](https://github.com/stm32-rs/stm32f4xx-hal) | [![Build Status](https://travis-ci.com/stm32-rs/stm32f4xx-hal.svg?branch=master)](https://travis-ci.com/stm32-rs/stm32f4xx-hal) |
| [stm32l1xx-hal](https://github.com/stm32-rs/stm32l1xx-hal) | [![Build Status](https://travis-ci.com/stm32-rs/stm32l1xx-hal.svg?branch=master)](https://travis-ci.com/stm32-rs/stm32l1xx-hal) |


##  This Package Setup

I am trying to have a common code base of examples that run on different boards.
(This may be wishful thinking.) I have still not decided the best way to 
organize this for Cargo. Workspaces do not seem to be intended for this.
My current setup is to have common files src/, examples/, ..., at the top level.
Then, under boards/,  use soft links to the common files.

You can get this package from Github with 
```
git clone https://github.com/pdgilbert/eg_stm_hal.git
```
This package is mostly examples in directory examples/, but the
build fails unless there are targets so there needs to be something in src/. 
That could be main.rs or lib.rs, which are defaults, or could be something else but then then
that needs to be specified in Cargo.toml. 
This package has a src/lib.rs file with a small utility function used in several examples.

It is unlikely that you would ever want to call functions in this package from another package,
so I do not expect to ever set it up as a crate for importing.


##  Notes on the Examples

This examples are derived after working through many other examples, in particular the exaples
in [stm32f1xx-hal.](https://github.com/stm32-rs/stm32f1xx-hal)

There is more detail about these examples in comments in the source files, 
but here is a brief summary

| xxx                     | notes |   Description CHECK THESE AGAIN                                |
| ----------------------- |:-----:|:-------------------------------------------------------------- |
| blink                   |   1   | Blinks off-board LEDs                                          |
| serial_loopback_char    |       | Single character loopback + semihost output                    |
| serial_fmt              |       | Formatted string write to console on usart1                             |
| serial_dma_tx           |       | String writes to console interface                             |
| serial_pass_thru_string |       | Read 15 chars input from console, output to semihost, repeat   |
| serial_loopback_string  |       | String serial interface loopback  + semihost output            |
| echo_by_char            |   2   | Echo back console input, char by char,  + semihost output      |
| serial_gps_rw           |   3   | Read by str from GPS with echo to console + semihost output    |
| serial_gps_rw_by_char   |       | Read by char from GPS with echo to console + semihost output   |
| serial_cross            |       | Str write from one usart and read on another + semihost output |


0.  Using the git versions of HALs (in Nov 2019 much is changing and release in crates.io is old). 
1.  Blink_test does not blink in gdb steps, use continue.
2.  With local echo on in console the characters are doubled, <cr> adds a single <lf>.
     Without local echo there is no <lf>. There is trouble if you type too fast.
4.  Ublox GPS by default uses 9600bps, odd Parity, 1 stop bit (minicom 8-N1). 
      This can be checked by directly connecting a computer through usb-ttl dongle to the GPS, 
      completely eliminating the development board. 
      (If the dongle power is used. 5v if preferred on mine.)

This is the status of examples as of November 2019:

|  xxx                    |  blue | pill  |    
|                         | build |  run  | 
| ----------------------- |:-----:|:-----:|
| blink                   | yes   | works | 
| serial_loopback_char    | yes   | works | 
| serial_fmt              | yes   | works | 
| serial_dma_tx           | no    |       |
| serial_pass_thru_string | yes   | works | 
| serial_loopback_string  | yes   |       |
| echo_by_char            | yes   | works | 
| serial_gps_rw           | yes   |       |
| serial_gps_rw_by_char   | yes   |       |
| serial_cross            | yes   |       |           


To build the examples use
```rust
cargo build  --target $TARGET  --features $MCU --example xxx
```
where `xxx` is one of the examples from the table above, and `TARGET` and `MCU` are environment
variables for your processor. Boards indicated above use one of 
```
  export MCU=stm32f103 TARGET=thumbv7m-none-eabi     #  bluepill Cortex-M3
  export MCU=stm32f303 TARGET=thumbv7em-none-eabihf  # STM32F303 Cortex-M3
  export MCU=stm32f411 TARGET=thumbv7em-none-eabihf  # nucleo-64
  export MCU=stm32l151 TARGET=thumbv7m-none-eabi     # heltec-lora-node151 Cortex-M3
```

Running the examples will require three shell windows on your desktop. 
One to run cargo and compile the examples and run gdb to load and debug them.
Another to run openocd to interface through the STlink to the development board.
And the third to run a console connected to a usb-ttl dongle for IO in some of the examples.
(I use minicom for this last, but there are many other possibilities.) 

To run the examples first connect the development board to the desktop.
Then in a separate windows do
```
minicom -D /dev/ttyUSBxx -b9600
```
where `xx` is replaced by the number of the USB port (see more notes below),
9600 is the bit rate in the code but can be change,
and
```
openocd -f interface/$INTERFACE.cfg -f target/$PROC.cfg 
```
and in the other window do
```
cargo  run --target $TARGET --features $MCU --example xxx
```

## Hardware Notes

I you have not yet bought a development board and are just looking to start then consider 
a 'blue pill' with a cheap STlink dongle. It is not only the cheapest by far (I think I got
5 for $10 with a dongle) but it also seems to be the best supported by HAL at the moment (Nov 2019). 
No doubt the support is because all the developers have a few lying around.
You could well want a more expensive development board when you get further along.

With the 'blue pill' beware that it
should have 1.5K pull up resistor on D+ (R10 on board). Ones I purchased do,
but some versions are shipped with 10K or 4.7K. Some PCs will tolerate this.
Also, the 3.3v regulator is very small (300mA) and feeds the input voltage through when it fails! Don't exceed 100mA.

FILL IN LAYOUT

## Misc Notes on STlink and OpenOCD

The openocd  command above uses `INTERFACE` and `PROC` environment variables that indicate the
STlink version and the development board MCU family respectively. 
(The PROC will be similar to the MCU setting, unfortunately they are not exactly the same.)
A typical specification for for bluepill development board and cheapo STlink dongle would be

```
  export INTERFACE=stlink-v2   PROC=stm32f1x 
```
Many development boads have an STlink built onto the board, in which case you need to determine
the version, and that is not always clear. My Discovery kit STM32F303 says STlink V2-B but that 
seems to mean v2-1. One symptom of an incorrect setting is that the openocd command start up 
stalls at
```
  in procedure 'ocd_bouncer'
```
The openocd command should alway get to something like
```
...
Info : stm32f1x.cpu: hardware has 6 breakpoints, 4 watchpoints
```

Some of the other causes for the `in procedure 'ocd_bouncer'` can be that the board is not 
properly powered up, or has some other boot loader pre-burned into it.

By removing 2 connectors on the 'ST-LINK' header it is possible to use the built in STlink on 
some development boards to program another board.
For example,  I can use the STlink on my Discovery STM32L100 to connect to a blue pill. 
One reason to do this is that  SWD header on the Discovery has the SWO pin, which can be
connected to PB3 pin on the blue pill to use itm. (Caveate, I have not got itm to work yet.)
Another reason is that some boards have STlink v2-1. Cheapo dongles typically have STlink v2
and only SWD header pins SWCLK, GND, SWDIO, and power are supported.

A cheapo dongle can provide power to a bluepill using the 3.3v on the dongle connection to 
3.3v on the blue pill SWD header. Beware that using STlink on another development board the power
through this SWD header connection is a voltage sensor and is not sufficient to power the blue pill.
Either power the blue pill with its own supply (eg. battery) or with separate 3.3v and gnd 
lines from the development board (typically pins 1 and 2) and do not exceed about 100mw for 
the blue pill and other things attached.

Here are settings I have used
```
  export  PROC=stm32f1x  # bluepill
  export  PROC=stm32l1x  # discovery-stm32l100
```
and
```
  export INTERFACE=stlink-v2   #  WaveGat dongle
  export INTERFACE=stlink-v2   #  STlink on Discovery STM32L100
  export INTERFACE=stlink-v2-1 #  STlink on Discovery STM32F303
```
The complete list of possible  openocd cfg file options are in
`/usr/share/openocd/scripts/interface/`, `/usr/share/openocd/scripts/target`
and `/usr/share/openocd/scripts/board`

## Misc Install Notes

Above assumes a development environment that has cargo, rust with cross compiler, rustup, 
gdb with remote processor support (gdb-multiarch ,  openocd, etc. 
These are described in detail in other places see, for example
- https://rust-embedded.github.io/book/intro/tooling.html
- https://docs.rust-embedded.org/book/intro/install/linux.html
- https://github.com/rust-lang/rustup.rs  


It is recommended to use the rust/cargo install directly from https://rust-lang.org rather than 
using apt-get, since apt-get requires root, does not have rustup, and is not the most recent.
I did something like
```
 sudo apt install gdb-multiarch  qemu-system-arm    # QEMU is emulator
 sudo apt install openocd       # for on chip debuging with ST-Link

 curl https://sh.rustup.rs -sSf | sh   # installs rustc, cargo, and rustup in  ~/.cargo/bin
 cargo install cargo-binutils          #updates in ~/.cargo/bin
 cargo install itm                     #updates in ~/.cargo/bin
 [rustup component add llvm-tools-preview # not sure ]

 rustc  --version
 rustup --version 
 rustup update

 rustup target list    # To see a list of available targets
 rustup show           # Show toolchain info. (Directories can have override of default.)
 rustup target add thumbv6m-none-eabi    # Cortex-M0, M0+, and M1 (ARMv6-M architecture)
 rustup target add thumbv7m-none-eabi    # Cortex-M3 (ARMv7-M architecture)
 rustup target add thumbv7em-none-eabi   # Cortex-M4 and M7 without hardware floating point
 rustup target add thumbv7em-none-eabihf # Cortex-M4F and M7F with hardware floating point
```

To use cargo run to also build and start gdb, in .cargo/config uncomment
```
  runner = "gdb-multiarch -q -x openocd.gdb"
```

##  Links

TO BE ORGANIZED SOMETIME
- The Rust Programming Language  https://docs.rust-lang.org/book/title-page
- The Embedded Rust Book   https://docs.rust-embedded.org/book/intro/index.html
- Survey of rust embedded documentation https://docs.rust-embedded.org/

- [HALs on Github](https://github.com/stm32-rs) and on [Travis CI.](https://travis-ci.org/stm32-rs)
- The CI for several rust embedded projects is [here.](https://travis-ci.org/rust-embedded)

- rust-embedded quickstart example  https://rust-embedded.github.io/book/start/qemu.html

- https://www.rust-lang.org/learn/get-started
- https://rust-embedded.github.io/book/
- https://crates.io/
- https://doc.rust-lang.org/cargo/index.html
- https://doc.rust-lang.org/cargo/guide/
- https://doc.rust-lang.org/book/ch11-00-testing.html
- https://github.com/japaric/rust-cross
- http://blog.japaric.io/brave-new-io/


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
