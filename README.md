# Examples Using embedded Rust

See the [status of examples](https://pdgilbert.github.io/eg_stm_hal/) for a summary.

## Preamble
These are newbie notes. 
This is my attempt to organize notes made while trying to figure out Rust/embedded.
CI is used to help keep track of what is working or not.
The examples and notes are here in case they may be useful to others. 
There is a lot of confusing out-of-date information
on the web, so my hope is that the CI links will warn readers when this project 
becomes old and broken.

Rust seems to have many attractive features compared to more mature languages.
It has a modern packaging system which encourages documentation and testing.
The language elements are designed so the compiler can catch many errors.
This is frustrating while learning the language but will be a substantial time saving
compared to run time debugging. The downside is that Rust is newer than many alternatives,
embedded Rust is even newer, and the hardware abstraction library (HAL) project is in 
active development.

After awhile things that were a time consuming stumbling block become so obvious that
it no longer seems necessary to mention them. That is why it is so hard to write beginner
documention, not to mention programmers' general reluctance to document anything.
As a newbie I am probably more sensitive to what is missing or un-said, so please be patient
if you think I am just stating the obvious sometimes.
Also please enter an issue if you think there is something that really needs to be clarified or added.

(My development environment is Linux, so these notes are specific to that 
in many places. However, the examples should work in other development 
environments. Just the setup may change.)

The code in the examples is organized so that the setup for different HALs and hardware is in the 
first section and the generic application code follows. The hope is that this will make clear how
to best take advantage of the generic aspect of `embedded-hal`. (The setup in many examples, 
including these, tends to dominate the application code, so the advantage of the HAL is not
always so obvious.) 
These examples are divided into two groups. The first group of core examples uses only the MCU specific HAL crate and
the main crates associated with `embedded-hal`. The second group of additional examples uses additional device driver crates.


##  Contents
- [Overall Status](#overall-status)
- [Summary of Examples](#summary-of-examples)
- [Additional Examples](#additional-examples)
- [This Package Setup](#this-package-setup)
- [Building Examples](#building-examples)
- [Running Examples](#running-examples)
- [Hardware Notes](#hardware-notes)
- [Misc Notes on ST-Link and OpenOCD](#misc-notes-on-st-link-and-openocd)
- [Misc Install Notes](#misc-install-notes)
- [Links](#Links)
- [License](#License)
- [Contribution](#Contribution)


##  Overall Status
 (Sept 2020) work in progress ...

This project's overall Travis CI build status is 
[![Build Status](https://api.travis-ci.org/pdgilbert/eg_stm_hal.svg?branch=master)](https://travis-ci.org/pdgilbert/eg_stm_hal).
This will indicate `failing` if any examples fail on any boards, 
so will usually indicate `failing` even if most examples work on most boards.
The CI is set up to use recent git versions of dependencies, so this project CI failing can be a result of updates
in dependencies. The upside of this is that if the examples are not failing then they are working with
recent versions of dependencies, 
unlike many examples from the web that I have worked through in my own learning process.

For the individual boards described below 
[details of the build and failures](https://travis-ci.org/pdgilbert/eg_stm_hal) can be
seen at Travis CI.

Testing if the code runs and does something resembling what it is supposed to do 
requires hardware and is not as automatic as CI. 
If you check the examples using MCUs other than those listed below then please provide details 
using [issues](https://github.com/pdgilbert/eg_stm_hal/issues) for this git project.

This project's examples depend on [embedded_hal](https://docs.rs/embedded-hal/) and several stm32 HALs. 
See [stm32 HALs on Github](https://github.com/stm32-rs) and on [Travis CI.](https://travis-ci.com/stm32-rs)

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


## Summary of Examples

These examples are derived after working through many other examples, in particular the examples
in [stm32f1xx-hal.](https://github.com/stm32-rs/stm32f1xx-hal)

There is more detail in comments in the example source files, see the 
[example directory](https://github.com/pdgilbert/eg_stm_hal/tree/master/examples).
Here is a brief summary table. 'Console' means a terminal session 
(eg. minicom) on a computer connected via usb-to-ttl to the MPU USART1 (pins pa9, pa10 on bluepill).
See [Running Examples](#running-examples) for more details.


| xxx                  | notes |   Description                                                  |
| -------------------- |:-----:|:-------------------------------------------------------------- |
| blink                |   1   | Blink on-board LED                                             |
| blink3               |   1   | Blink off-board LEDs attached to  pb 13,14,15                  |
| echo_by_char         |   2   | Echo console input, char by char,  + semihost output           |
| echo_string          |       | Read 15 chars input from console, output to semihost, repeat   |
| serial_char          |       | Single char between usarts 2 and 3, console and semihost output|
| serial_string        |       | String writes between usarts 2 and 3, console and semihost output|
| gps_rw_by_char       |       | Read by char from GPS with echo to console + semihost output   |
| gps_rw               |   3   | Read a line  from GPS with echo to console + semihost output   |
| temperature          |       | Read temperature of MCU and external TMP35 * semihost output   |


0.  Using the git versions of HALs (in July 2020 much is changing and release in crates.io is old). 
1.  Does not blink when stepping in gdb, use continue.
2.  With local echo on in console the characters are doubled, `<cr>` adds a single `<lf>`.
     Without local echo there is no `<lf>`. There is trouble if you type too fast.
3.  Ublox GPS by default uses 9600bps, odd Parity, 1 stop bit (minicom 8-N-1). 
      This can be checked by directly connecting a computer through usb-ttl dongle to the GPS, 
      completely eliminating the development board. 
      (If the dongle power is used. 5v if preferred on mine.)

The current status of examples is [summarized automatically by the CI.](https://pdgilbert.github.io/eg_stm_hal/#status-of-examples)

## Additional Examples

These are examples which use an additional device crate.

| xxx          |    crate    | notes |   Description                                              |
| ------------ |:-----------:|:-----:|:---------------------------------------------------------- |
| dht          | dht         |       | read a dht11 sensor and write to semihost                  |
| dht11        | dht11       |       | read a dht11 sensor and write to semihost                  |
| text_i2c     | ssd1306     |       | write 2 text lines on ssd1306 OLED                         |
| oled_gps     | ssd1306     |       | read gps and write coordinates on ssd1306 OLED             |
| lora_send    | sx127x_lora |       | transmit a character string over LoRa,  + semihost output  |
| lora_receive | sx127x_lora |       | receive  a character string over LoRa,  + semihost output  |
| lora_gps     | sx127x_lora |       | read gps and transmit over LoRa,  + semihost output        |

The current status of these examples is [here.](https://pdgilbert.github.io/eg_stm_hal/#additional-examples)


##  This Package Setup

I am trying to have a common code base for examples that run on different boards.
The setup is to have common files `src/`, `examples/`, ..., at the top level.
Then in the `boards/` directories use soft links to the common files. 
That leaves only `memory.x` and build files `target/` and `Cargo.lock` in
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


## Building Examples

To build the examples cd into one of the board directories, eg `cd boards/bluepill` 
and use
```
cargo build  --target $TARGET  --features $HAL,$MCU --example xxx
```
where `xxx` is one of the examples from the table above, and `TARGET`, `HAL`  and `MCU` are
environment variables for your processor (and corresponding to the board directory). 
Variables `HAL`  and `MCU` overlap. It should be possible to determine  `HAL`  based on `MCU`.
The variable `HAL` is used in the example code whereas some of the underlying HAL packages
actually need the specific `MCU`.
Board directories use one of 
```
  export HAL=stm32f0xx MCU=stm32f030   TARGET=thumbv6m-none-eabi     # none-stm32f030      Cortex-M
  export HAL=stm32f1xx MCU=stm32f103   TARGET=thumbv7m-none-eabi     # bluepill            Cortex-M3
  export HAL=stm32f1xx MCU=stm32f100   TARGET=thumbv7m-none-eabi     # none-stm32f100      Cortex-M3
  export HAL=stm32f1xx MCU=stm32f101   TARGET=thumbv7m-none-eabi     # none-stm32f101      Cortex-M3
  export HAL=stm32f3xx MCU=stm32f303xc TARGET=thumbv7em-none-eabihf  # discovery-stm32f303 Cortex-M3
  export HAL=stm32f4xx MCU=stm32f401   TARGET=thumbv7em-none-eabihf  # blackpill-stm32f401 Cortex-M4
  export HAL=stm32f4xx MCU=stm32f411   TARGET=thumbv7em-none-eabihf  # blackpill-stm32f411 Cortex-M4
  export HAL=stm32f4xx MCU=stm32f411   TARGET=thumbv7em-none-eabihf  # nucleo-64           Cortex-M4
  export HAL=stm32f7xx MCU=stm32f722   TARGET=thumbv7em-none-eabihf  # none-stm32f722      Cortex-M7
  export HAL=stm32h7xx MCU=stm32h742   TARGET=thumbv7em-none-eabihf  # none-stm32h742      Cortex-M7
  export HAL=stm32l0xx MCU=stm32l0x2   TARGET=thumbv6m-none-eabi     # none-stm32l0x2      Cortex-M0
  export HAL=stm32l1xx MCU=stm32l100   TARGET=thumbv7m-none-eabi     # discovery-stm32l100 Cortex-M3
  export HAL=stm32l1xx MCU=stm32l151   TARGET=thumbv7m-none-eabi     # heltec-lora-node151 Cortex-M3
  export HAL=stm32l4xx MCU=stm32l4x1   TARGET=thumbv7em-none-eabi    # none-stm32l4x1      Cortex-M4
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
5 for $10 with a dongle) but it also seems to be the best supported by HAL at the moment (July 2020). 
No doubt the support is because all the developers have a few lying around.
You could well want a more expensive development board when you get further along.

With the 'blue pill' beware that it
should have 1.5K pull up resistor on D+ (R10 on board). Ones I purchased do,
but some versions are shipped with 10K or 4.7K. Googling suggests some PCs will tolerate this.
Also, the 3.3v regulator is small (300mA) and apparently feeds the input voltage through 
when it fails! Don't exceed 100mA. For the blue pill you might also consult https://github.com/TeXitoi/blue-pill-quickstart.

FILL IN LAYOUT

## Misc Notes on ST-Link and OpenOCD

The openocd  command above uses `INTERFACE` and `PROC` environment variables that indicate the
ST-Link version and the development board MCU family respectively. 
(The PROC will be similar to the HAL and MCU setting, unfortunately they are not exactly the same.)
Typical specifications for a bluepill development board and ST-Link dongle would be

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
properly powered up, or has some other boot loader pre-burned into it. (In the later case
try booting once in "System Memory" mode, see 
https://www.electronicshub.org/getting-started-with-stm32f103c8t6-blue-pill/.)
The message `Warn : UNEXPECTED idcode: 0x...` seems to require editing the openocd cfg
file which gets installed in various places, possibly 
`/usr/share/openocd/scripts/target/stm32f1x.cfg`. Changing the CPUTAPID or change the
line `swj_newdap ... -expected-id $_CPUTAPID` to `swj_newdap ... -expected-id 0` so that
CPUTAID is ignored.

Some development boards pop up a a window with Mbed.htm which I dismiss and then run `openocd`.

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

Here are settings I used for manual testing as reported at 
[status of examples](https://pdgilbert.github.io/eg_stm_hal/)
```
  export INTERFACE=stlink-v2    PROC=stm32f1x  #cheap dongle and blue pill
  export INTERFACE=stlink-v2-1  PROC=stm32f3x  #discovery-stm32f303
  export INTERFACE=stlink-v2    PROC=stm32f4x  # blackpills  with STM32F401 and STM32F411
  export INTERFACE=stlink-v2-1  PROC=stm32f4x  #nucleo-64
  export INTERFACE=stlink-v2    PROC=stm32l1   #discovery-stm32l100 
  export INTERFACE=stlink-v2    PROC=stm32l1   #heltec-lora-node151 with cheap dongle  
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
This is already done in the `.cargo/config` in this package. The error message when something is wrong 
with `.cargo` may be something not very helpful, like `Syntax error: word unexpected (expecting ")")`.
The board directories on this package have soft links for `.cargo` to a common file in the root directory.

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
