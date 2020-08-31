
##  Contents
- [Status of Examples](#status-of-examples)
- [Additional Examples](#additional-examples)


## Status of Examples

Following is the status of examples. Examples are run with `stm32f1xx_hal` on a `bluepill`,
`stm32f3xx_hal` on a `Discovery kit STM32F303`, `stm32l1xx_hal` on a `STM32L100C Discovery`, 
and `stm32f4xx_hal` on a `Nucleo-64 STM32F411`, a `blackpill` with MCU `stm32f401`, 
and a `blackpill` with MCU `stm32f411`.
In the table cells: 
Green check marks and red X marks indicate that the CI example check builds or fails.
`runs` is an indication that a manual test on actual hardware has been done and it works correctly or as noted, and 
`no` means the manual test fails badly as noted. The CI testing is automatic and corresponds to the most recent
example code, and using recent git versions of packages. The manual tests are not automatic, and less current.

Commit <embed src=examplesStatus/bluepill/COMMIT> on <embed src=examplesStatus/bluepill/DATE.STAMP> 

The link in the board cell goes to a file recording the commit that was used for the example tests.

|    hal    |                        board                                      |                     blink                               |                         blink3                          |                         echo_by_char                            |                  echo_string                                   |                           serial_char                          |                           serial_string                        |                              gps_rw_by_char                        |                       gps_rw                               |                      temperature                           |
|:---------:|:--------------------:|:-----:|:------:|:------------:|:-----------:|:-----------:|:-------------:|:--------------:|:------:|:--------------:|
| stm32f1xx | [bluepill](examplesStatus/bluepill/COMMIT)                        |<img src="examplesStatus/bluepill/blink.png" width="50" /> runs             |![?](examplesStatus/bluepill/blink3.png|width=50px) runs            |![?](examplesStatus/bluepill/echo_by_char.png =10x) runs-5            |![?](examplesStatus/bluepill/echo_string.png =50x) runs-5            |![?](examplesStatus/bluepill/serial_char.png) runs-1            |![?](examplesStatus/bluepill/serial_string.png) no-2            |![?](examplesStatus/bluepill/gps_rw_by_char.png) runs               |![?](examplesStatus/bluepill/gps_rw.png) runs               |![?](examplesStatus/bluepill/temperature.png) runs          |      
| stm32f3xx | [discovery-stm32f303](examplesStatus/discovery-stm32f303/COMMIT)  |![?](examplesStatus/discovery-stm32f303/blink.png) runs  |![?](examplesStatus/discovery-stm32f303/blink3.png) runs |![?](examplesStatus/discovery-stm32f303/echo_by_char.png) runs-5 |![?](examplesStatus/discovery-stm32f303/echo_string.png) no-8,9 |![?](examplesStatus/discovery-stm32f303/serial_char.png) runs-1 |![?](examplesStatus/discovery-stm32f303/serial_string.png) no-9 |![?](examplesStatus/discovery-stm32f303/gps_rw_by_char.png) runs    |![?](examplesStatus/discovery-stm32f303/gps_rw.png) runs-10 | ![?](examplesStatus/discovery-stm32f303/temperature.png)   |
| stm32f4xx | [nucleo-64](examplesStatus/nucleo-64/COMMIT) 	                |![?](examplesStatus/nucleo-64/blink.png) runs            |![?](examplesStatus/nucleo-64/blink3.png) runs           |![?](examplesStatus/nucleo-64/echo_by_char.png) runs-5           |![?](examplesStatus/nucleo-64/echo_string.png) no-9             |![?](examplesStatus/nucleo-64/serial_char.png) no-2             |![?](examplesStatus/nucleo-64/serial_string.png) no-9           |![?](examplesStatus/nucleo-64/gps_rw_by_char.png) no-6              |![?](examplesStatus/nucleo-64/gps_rw.png) no-6              |![?](examplesStatus/nucleo-64/temperature.png)              |
| stm32f4xx | [blackpill-stm32f401](examplesStatus/blackpill-stm32f401/COMMIT)  |![?](examplesStatus/blackpill-stm32f401/blink.png) runs  |![?](examplesStatus/blackpill-stm32f401/blink3.png) runs |![?](examplesStatus/blackpill-stm32f401/echo_by_char.png) runs-5 |![?](examplesStatus/blackpill-stm32f401/echo_string.png) no-9   |![?](examplesStatus/blackpill-stm32f401/serial_char.png) runs   |![?](examplesStatus/blackpill-stm32f401/serial_string.png) no-9 |![?](examplesStatus/blackpill-stm32f401/gps_rw_by_char.png) runs-10 |![?](examplesStatus/blackpill-stm32f401/gps_rw.png) runs-10 |![?](examplesStatus/blackpill-stm32f401/temperature.png)    |
| stm32f4xx | [blackpill-stm32f411](examplesStatus/blackpill-stm32f411/COMMIT)  |![?](examplesStatus/blackpill-stm32f411/blink.png) runs  |![?](examplesStatus/blackpill-stm32f411/blink3.png) runs |![?](examplesStatus/blackpill-stm32f411/echo_by_char.png) no-12  |![?](examplesStatus/blackpill-stm32f411/echo_string.png) no-9   |![?](examplesStatus/blackpill-stm32f411/serial_char.png) runs   |![?](examplesStatus/blackpill-stm32f411/serial_string.png) no-9 |![?](examplesStatus/blackpill-stm32f411/gps_rw_by_char.png) runs    |![?](examplesStatus/blackpill-stm32f411/gps_rw.png) runs    |![?](examplesStatus/blackpill-stm32f411/temperature.png)    |
| stm32l1xx | [discovery-stm32l100](examplesStatus/discovery-stm32l100/COMMIT)  |![?](examplesStatus/discovery-stm32l100/blink.png) runs  |![?](examplesStatus/discovery-stm32l100/blink3.png) runs |![?](examplesStatus/discovery-stm32l100/echo_by_char.png) no     |![?](examplesStatus/discovery-stm32l100/echo_string.png) no     |![?](examplesStatus/discovery-stm32l100/serial_char.png) no     |![?](examplesStatus/discovery-stm32l100/serial_string.png) no   |![?](examplesStatus/discovery-stm32l100/gps_rw_by_char.png) no      |![?](examplesStatus/discovery-stm32l100/gps_rw.png) no      |![?](examplesStatus/discovery-stm32l100/temperature.png)    |


1.  tx2 to rx3 works. tx3 to rx2 works sometimes but sometimes fails unwrapping err value Overrun on receive.
2.  Stalls waiting to receive. Possibly need thread to receive started before send?
3.  Usart2 with Usart3 connection works both ways but jibberish written on console.
4.  Gibberish written on console.
5.  Works as long as typing is slow.
6.  Fails reading gps (does not return). 
7.  Works once, repeat problems.
8.  Writeln! macro missing from stm32f3xx ?
9.  Uses dma buffering in stm32f1xx. Have not figured out how to do that with other HALs.
10. Some lines miss beginning or truncated.
11. Overrun error.
12. no echo.

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

The status of these examples is

|    hal    |         board        |                      dht                              |                    dht11                                |                     text_i2c                               |                        oled_gps                              |                        lora_send                      |                     lora_receive                         |                   lora_gps                           |
|:---------:|:--------------------:|:-----:|:-----:|:--------:|:--------:|:---------:|:------------:|:--------:|
| stm32f1xx | bluepill             |![?](examplesStatus/bluepill/dht.png)            no-1  |![?](examplesStatus/bluepill/dht11.png)            no-1  |![?](examplesStatus/bluepill/text_i2c.png)            runs   |![?](examplesStatus/bluepill/oled_gps.png)            no-2   |![?](examplesStatus/bluepill/lora_send.png)            |![?](examplesStatus/bluepill/lora_receive.png)            |![?](examplesStatus/bluepill/lora_gps.png)            |
| stm32f3xx | discovery-stm32f303  |![?](examplesStatus/discovery-stm32f303/dht.png)       |![?](examplesStatus/discovery-stm32f303/dht11.png)       |![?](examplesStatus/discovery-stm32f303/text_i2c.png) runs   |![?](examplesStatus/discovery-stm32f303/oled_gps.png)        |![?](examplesStatus/discovery-stm32f303/lora_send.png) |![?](examplesStatus/discovery-stm32f303/lora_receive.png) |![?](examplesStatus/discovery-stm32f303/lora_gps.png) |
| stm32f4xx | nucleo-64 	   |![?](examplesStatus/nucleo-64/dht.png)                 |![?](examplesStatus/nucleo-64/dht11.png)                 |![?](examplesStatus/nucleo-64/text_i2c.png)           runs   |![?](examplesStatus/nucleo-64/oled_gps.png)                  |![?](examplesStatus/nucleo-64/lora_send.png)           |![?](examplesStatus/nucleo-64/lora_receive.png)           |![?](examplesStatus/nucleo-64/lora_gps.png)           |
| stm32f4xx | blackpill-stm32f401  |![?](examplesStatus/blackpill-stm32f401/dht.png) no-0  |![?](examplesStatus/blackpill-stm32f401/dht11.png) no-0  |![?](examplesStatus/blackpill-stm32f401/text_i2c.png) runs   |![?](examplesStatus/blackpill-stm32f401/oled_gps.png) runs   |![?](examplesStatus/blackpill-stm32f401/lora_send.png) |![?](examplesStatus/blackpill-stm32f401/lora_receive.png) |![?](examplesStatus/blackpill-stm32f401/lora_gps.png) |
| stm32f4xx | blackpill-stm32f411  |![?](examplesStatus/blackpill-stm32f411/dht.png) no-0  |![?](examplesStatus/blackpill-stm32f411/dht11.png) no-0  |![?](examplesStatus/blackpill-stm32f411/text_i2c.png) runs   |![?](examplesStatus/blackpill-stm32f411/oled_gps.png) runs   |![?](examplesStatus/blackpill-stm32f411/lora_send.png) |![?](examplesStatus/blackpill-stm32f411/lora_receive.png) |![?](examplesStatus/blackpill-stm32f411/lora_gps.png) |
| stm32l1xx | discovery-stm32l100  |![?](examplesStatus/discovery-stm32l100/dht.png)       |![?](examplesStatus/discovery-stm32l100/dht11.png)       |![?](examplesStatus/discovery-stm32l100/text_i2c.png) no     |![?](examplesStatus/discovery-stm32l100/oled_gps.png)        |![?](examplesStatus/discovery-stm32l100/lora_send.png) |![?](examplesStatus/discovery-stm32l100/lora_receive.png) |![?](examplesStatus/discovery-stm32l100/lora_gps.png) |

0. panic. Timer not set right yet.
1. stall/timeout reading sensor.
2. too large for flash.

