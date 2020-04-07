# Examples Using embedded Rust

The [status for individual boards described below can be 
seen at Travis CI.](https://travis-ci.org/pdgilbert/eg_stm_hal)

This project's overall Travis CI build status is 
[![Build Status](https://travis-ci.org/pdgilbert/eg_stm_hal.svg?branch=master)](https://travis-ci.org/pdgilbert/eg_stm_hal).
This will indicate an error if any examples fail on any boards, 
so often will indicate `error` even when most examples work on most boards.

## Preamble
These are newbie notes. I really am new to embedded programming and to Rust. 
However, I do have experience identifying bugs in other languages, and setting
up examples and tests to help eliminate them.
This is my attempt to organize notes made while trying to figure out Rust/embedded,
and to use Travis CI to monitor what is working or not.
I have put the examples and notes here so they  can be useful to others. 
There is a lot of confusing out-of-date information
on the web, so my hope is that the CI links here will warn readers when this project 
becomes old and broken.

Rust seems to have many attractive features compared to more mature languages.
It has a modern packaging system which encourages documentation and testing.
The language elements are designed so the compiler can catch many errors.
This is frustrating while learning the language but will be a substantial time saving
compared to run time debugging. The downside is that Rust is newer than many alternatives,
embedded Rust is even newer, and the hardware abstraction library (HAL) project is in 
active development.

So, this is not yet easy territory for faint of heart newbies, or anyone on a strict timeline.
For me, having spent many years dealing with cross platform desktop problems, 
HAL just makes a lot of sense.

After awhile things that were a time consuming stumbling block become so obvious that
it no longer seems necessary to mention them. That is why it is so hard to write beginner
documention, not to mention programmers' general reluctance to document anything.
As a newbie I am probably more sensitive to what is missing or un-said, so please be patient
if you think I am just stating the obvious sometimes.
Also please enter an issue if you think there is something that really needs to be clarified or added.

(My development environment is Linux, so these notes are specific to that 
in many places. However, the examples should work in other development 
environments. Just the setup may change.)

##  Contents
- [Status Summary](#status-summary)
- [This Package Setup](#this-package-setup)
- [Summary of Examples](#summary-of-examples)
- [Building Examples](#building-examples)
- [Running Examples](#running-examples)
- [Hardware Notes](#hardware-notes)
- [Misc Notes on ST-Link and OpenOCD](#misc-notes-on-st-link-and-openocd)
- [Misc Install Notes](#misc-install-notes)
- [Links](#Links)
- [License](#License)
- [Contribution](#Contribution)


##  Status Summary
 (March 2020) work in progress ...

The overall Travis CI build status and the link for individual boards is given above.
Testing if the code runs and does something resembling what it is supposed to do 
requires hardware and is not as automatic as CI. 
This is my summary as of March 2020. If you check the examples using one of these MCUs 
then please provide details 
using [issues](https://github.com/pdgilbert/eg_stm_hal/issues) on this git project page.

|      HAL       | eg MCU    |   eg Board          |   Builds   |  Runs  |          Notes                             |
| -------------- |:---------:|:-------------------:|:----------:|:------:| :----------------------------------------- |
| stm32f1xx-hal  | stm32f103 |      bluepill       |    yes     |  many  | Problems using serial in some examples     |
| stm32f3xx-hal  | stm32f303 | discovery-stm32f303 |   some     |  some  | Hal differences.  Code adjustments needed  |
| stm32f4xx-hal  | stm32f411 |      nucleo-64      |   some     |  some  | Hal differences.  Code adjustments needed  |
| stm32l1xx-hal  | stm32l100 | discovery-stm32l100 |    no      |   no   | Hal does not build.                        |
| stm32l1xx-hal  | stm32l151 | heltec-lora-node151 |    no      |   no   | Hal does not build.                        |

This project's examples depend on HALs. 
See [HALs on Github](https://github.com/stm32-rs) and on [Travis CI.](https://travis-ci.com/stm32-rs)

|   HAL git                         |       HAL Travis CI  Status           | 
|:---------------------------------:|:-------------------------------------:|
| [stm32f0xx-hal](https://github.com/stm32-rs/stm32f0xx-hal) | [![Build Status](https://travis-ci.com/stm32-rs/stm32f0xx-hal.svg?branch=master)](https://travis-ci.com/stm32-rs/stm32f0xx-hal) |
| [stm32f1xx-hal](https://github.com/stm32-rs/stm32f1xx-hal) | [![Build Status](https://travis-ci.com/stm32-rs/stm32f1xx-hal.svg?branch=master)](https://travis-ci.com/stm32-rs/stm32f1xx-hal) |
| [stm32f3xx-hal](https://github.com/stm32-rs/stm32f3xx-hal) | [![Build Status](https://travis-ci.com/stm32-rs/stm32f3xx-hal.svg?branch=master)](https://travis-ci.com/stm32-rs/stm32f3xx-hal) |
| [stm32f4xx-hal](https://github.com/stm32-rs/stm32f4xx-hal) | [![Build Status](https://travis-ci.com/stm32-rs/stm32f4xx-hal.svg?branch=master)](https://travis-ci.com/stm32-rs/stm32f4xx-hal) |
| [stm32f7xx-hal](https://github.com/stm32-rs/stm32f7xx-hal) | [![Build Status](https://travis-ci.com/stm32-rs/stm32f7xx-hal.svg?branch=master)](https://travis-ci.com/stm32-rs/stm32f7xx-hal) |
| [stm32g0xx-hal](https://github.com/stm32-rs/stm32g0xx-hal) | [![Build Status](https://travis-ci.com/stm32-rs/stm32g0xx-hal.svg?branch=master)](https://travis-ci.com/stm32-rs/stm32g0xx-hal) |
| [stm32g4xx-hal](https://github.com/stm32-rs/stm32g4xx-hal) | [![Build Status](https://travis-ci.com/stm32-rs/stm32g4xx-hal.svg?branch=master)](https://travis-ci.com/stm32-rs/stm32g4xx-hal) |
| [stm32h7xx-hal](https://github.com/stm32-rs/stm32h7xx-hal) | [![Build Status](https://travis-ci.com/stm32-rs/stm32h7xx-hal.svg?branch=master)](https://travis-ci.com/stm32-rs/stm32h7xx-hal) |
| [stm32l0xx-hal](https://github.com/stm32-rs/stm32l0xx-hal) | [![Build Status](https://travis-ci.com/stm32-rs/stm32l0xx-hal.svg?branch=master)](https://travis-ci.com/stm32-rs/stm32l0xx-hal) |
| [stm32l1xx-hal](https://github.com/stm32-rs/stm32l1xx-hal) | [![Build Status](https://travis-ci.com/stm32-rs/stm32l1xx-hal.svg?branch=master)](https://travis-ci.com/stm32-rs/stm32l1xx-hal) |
| [stm32l4xx-hal](https://github.com/stm32-rs/stm32l4xx-hal) | [![Build Status](https://travis-ci.com/stm32-rs/stm32l4xx-hal.svg?branch=master)](https://travis-ci.com/stm32-rs/stm32l4xx-hal) |


##  This Package Setup

I am trying to have a common code base of examples that run on different boards.
(This may be wishful thinking.) I have still not decided the best way to 
organize this for Cargo. Workspaces do not seem to be intended for this.
My current setup is to have common files `src/`, `examples/`, ..., at the top level.
Then in the `boards/` directories use soft links to the common files. 
That leaves only `memory.x` and and build files `target/` and `Cargo.lock` in
the `boards/` directories.

You can get this package from Github with 
```
git clone https://github.com/pdgilbert/eg_stm_hal.git
```
The package is mostly examples in directory `examples/`, but the
build fails unless there are targets so there needs to be something in `src/`. 
That could be `main.rs` or `lib.rs`, which are defaults, or could be something else but then
that needs to be specified in Cargo.toml. 
This package has a `src/lib.rs` file with a small utility function used in several examples.

It is unlikely that you would ever want to call functions in this package from another package,
so I do not expect to ever set it up as a crate for importing.


## Summary of Examples

These examples are derived after working through many other examples, in particular the examples
in [stm32f1xx-hal.](https://github.com/stm32-rs/stm32f1xx-hal)

There is more detail about examples in comments in the source files, see the 
[example directory](https://github.com/pdgilbert/eg_stm_hal/tree/master/examples).
Here is a brief summary. 'Console' means a terminal session 
(eg. minicom) on a computer connected via usb-to-ttl to the MPU USART1 (pins pa9, pa10 on bluepill).
See [Running Examples](#running-examples) for more details.


| xxx                  | notes |   Description                                                  |
| -------------------- |:-----:|:-------------------------------------------------------------- |
| blink                |   1   | Blinks off-board LEDs attached to  pb 13,14,15                 |
| serial_char          |       | Single char between usarts 2 and 3, console and semihost output|
| serial_string        |       | String writes between usarts 2 and 3, console and semihost output|
| echo_console_by_char |   2   | Echo console input, char by char,  + semihost output           |
| echo_console_string  |       | Read 15 chars input from console, output to semihost, repeat   |
| gps_rw_by_char       |       | Read by char from GPS with echo to console + semihost output   |
| gps_rw               |   3   | Read by str  from GPS with echo to console + semihost output   |


0.  Using the git versions of HALs (in Nov 2019 much is changing and release in crates.io is old). 
1.  Blink does not blink when stepping in gdb, use continue.
2.  With local echo on in console the characters are doubled, `<cr>` adds a single `<lf>`.
     Without local echo there is no `<lf>`. There is trouble if you type too fast.
4.  Ublox GPS by default uses 9600bps, odd Parity, 1 stop bit (minicom 8-N-1). 
      This can be checked by directly connecting a computer through usb-ttl dongle to the GPS, 
      completely eliminating the development board. 
      (If the dongle power is used. 5v if preferred on mine.)

This is the status of examples as of April 2020:

| ---------------------------- | -- bluepill --- | -- disc. f303 --- | -- nucleo-64 --- |
|:--------------------:|:----------:|:----------:|:----------:|

|  xxx                 | build |  run  | build |  run  | build |  run  | 
|:--------------------:|:-----:|:-----:|:-----:|:-----:|:-----:|:-----:|
| blink                |  yes  | works |  yes  | works |  yes  | works | 
| serial_char          |  yes  |   1   |  yes  |  1c   |  yes  |   1b  |
| serial_string        |  no   |       |       |       |       |       |
| echo_console_by_char |  yes  | works |       |       |  yes  | works | 
| echo_console_string  |  no   |       |       |       |       |       | 
| gps_rw_by_char       |  yes  |   3   |  yes  |       |  yes  |   3   |
| gps_rw               |       |       |       |       |       |       |

1.   tx2 to rx3 works. tx3 to rx2 fails unwrapping err value on receive.
1b.  Stalls waiting to receive.
1c.  Usart2 with Usart3 connection works both ways but jibberish written on console.
2.   Jibberish written on console.
3.   Fails reading gps. 
4.   Works once, repeat problems.

## Building Examples

To build the examples cd into one of the board directories, eg `cd boards/bluepill` 
and use
```rust
cargo build  --target $TARGET  --features $HAL,$MCU --example xxx
```
where `xxx` is one of the examples from the table above, and `TARGET`, `HAL`  and `MCU` are
environment variables for your processor (and corresponding to the board directory). 
Variables `HAL`  and `MCU` overlap, it should be possible to determine  `HAL`  based on `MCU`.
The variable `HAL` is used in the example code whereas some of the underlying HAL packages
actually need the specific `MCU`.
Boards indicated above use one of 
```
  export HAL=stm32f1xx MCU=stm32f103 TARGET=thumbv7m-none-eabi     # bluepill            Cortex-M3
  export HAL=stm32f1xx MCU=stm32f100 TARGET=thumbv7m-none-eabi     # none-stm32f100      Cortex-M3
  export HAL=stm32f1xx MCU=stm32f101 TARGET=thumbv7m-none-eabi     # none-stm32f101      Cortex-M3
  export HAL=stm32f3xx MCU=stm32f303 TARGET=thumbv7em-none-eabihf  # discovery-stm32f303 Cortex-M3
  export HAL=stm32f4xx MCU=stm32f411 TARGET=thumbv7em-none-eabihf  # nucleo-64           Cortex-M4
  export HAL=stm32l1xx MCU=stm32l100 TARGET=thumbv7m-none-eabi     # discovery-stm32l100 Cortex-M3
  export HAL=stm32l1xx MCU=stm32l151 TARGET=thumbv7m-none-eabi     # heltec-lora-node151 Cortex-M3
```

## Running Examples

Running the examples will require three shell windows on your desktop. 
One to run cargo and compile the examples and run gdb to load and debug them.
Another to run openocd to interface through the STlink to the development board.
And the third to run a console connected to a usb-ttl dongle for IO in some of the examples.
(I use minicom for this last, but there are many other possibilities.) 

(If the next connections to USB fail with `Permission denied` then the simplest 
fix is to change the permissions on /dev/ttyUSBx so it is world rw.
This has security implications so you might consider something safer.
For the console you can add  your user name to the dialout group
with `sudo adduser username dialout`. 
Beware group changes may not take effect until you re-login. 
For the STlink the udev rules can be set, as described in
[Rust-embedded book](https://rust-embedded.github.io/book/intro/install/linux.html).
I have had to do this for one computer and not for another.
I don't understand it well enough to explain.)

To run the examples first connect the development board (STlink and console 
USB-TTL) to two usb ports on the computer and determine 
the USB device number for the console by
```
dmesg | grep -i tty  
```
Then in a separate windows do
```
minicom -D /dev/ttyUSBx -b9600
```
where `x` is replaced by the number of the USB console device.
9600 is the bit rate in the code but can be change.
Next determine the settings for `INTERFACE` and `PROC` as described below in
[Misc Notes on ST-Link and OpenOCD](#misc-notes-on-st-link-and-openocd)
and then
```
openocd -f interface/$INTERFACE.cfg -f target/$PROC.cfg 
```
`openocd` seems to figure out the USB device to use. In the other window do
```
cargo  run --target $TARGET --features $HAL,$MCU --example xxx
```
This assumes you have set up a runner in `.cargo/config` as mentioned below in
[Misc Install Notes](#misc-install-notes). If all works then gdb will load the example and
stop at the first breakpoint. Use
```
   (gdb) continue 
```
to start running the example code.

## Hardware Notes

If you have not yet bought a development board and are just looking to start then consider 
a 'blue pill' with a cheap ST-Link dongle. It is not only the cheapest by far (I think I got
5 for $10 with a dongle) but it also seems to be the best supported by HAL at the moment (Nov 2019). 
No doubt the support is because all the developers have a few lying around.
You could well want a more expensive development board when you get further along.

With the 'blue pill' beware that it
should have 1.5K pull up resistor on D+ (R10 on board). Ones I purchased do,
but some versions are shipped with 10K or 4.7K. Googling suggests some PCs will tolerate this.
Also, the 3.3v regulator is small (300mA) and apparently feeds the input voltage through 
when it fails! Don't exceed 100mA.

FILL IN LAYOUT

## Misc Notes on ST-Link and OpenOCD

The openocd  command above uses `INTERFACE` and `PROC` environment variables that indicate the
ST-Link version and the development board MCU family respectively. 
(The PROC will be similar to the HAL and MCU setting, unfortunately they are not exactly the same.)
Typical specification for a bluepill development board and ST-Link dongle would be

```
  export INTERFACE=stlink-v2    PROC=stm32f1x  #cheap  dongle and blue pill
  export INTERFACE=stlink-v2-1  PROC=stm32f1x  #better dongle and blue pill
```
Many development boads have an ST-Link built onto the board, in which case you need to determine
the version, and that is not always clear. My discovery-stm32f303 (Discovery kit STM32F303)
says STlink V2-B but that 
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

Development boards I have tried:
```
  export INTERFACE=stlink-v2-1  PROC=stm32f3x  #discovery-stm32f303
  export INTERFACE=stlink-v2-1  PROC=stm32f4x  #nucleo-64
```
The discovery-stm32f303 and nucleo-64 pop up a a window with Mbed.htm which I dismiss and then run `openocd`.

It is possible to use the built in STlink on some development boards to program another board. 
To do this it is necessary to  removing 2 connectors on the 'ST-LINK' header and connect the 
SWD header to the other board.
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
  export  PROC=stm32l1x  # MCU on Discovery STM32L100
  export  PROC=stm32f3x  # MCU on Discovery STM32F303
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
 sudo apt install gcc-multilib  # for 32 bit support I think
 sudo apt install minicom

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

To use cargo to build and also start gdb and run the compiled code, in `.cargo/config` uncomment
```
  runner = "gdb-multiarch -q -x openocd.gdb"
```
This is already done in the `.cargo/config` in this package.

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
