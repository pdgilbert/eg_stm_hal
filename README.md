
##  Contents
- [Status of Examples](#status-of-examples)
- [Additional Examples](#additional-examples)


## Status of Examples

Following is the status of examples as of July 2020. Examples are run with `stm32f1xx_hal` on a `bluepill`,
`stm32f3xx_hal` on a `Discovery kit STM32F303`, `stm32l1xx_hal` on a `STM32L100C Discovery`, 
and `stm32f4xx_hal` on a `Nucleo-64 STM32F411`, a `blackpill` with MCU `stm32f401`, 
and a `blackpill` with MCU `stm32f411`.
In the table cells: 
`runs` means builds and runs correctly, or as noted; `builds` means builds but run not tested; 
`no` means does not build, or builds but fails badly as noted. 

Commit <embed src=examplesStatus/bluepill/COMMIT> on <embed src=examplesStatus/bluepill/DATE.STAMP> 

[Commit gets file ](examplesStatus/bluepill/COMMIT)


![alt text](examplesStatus/bluepill/blink.png)

![alt text](checkMark.png)

![?](examplesStatus/bluepill/blink.png)


|    hal    |         board        | blink | blink3 | echo_by_char | echo_string | serial_char | serial_string | gps_rw_by_char | gps_rw |   temperature  |
|:---------:|:--------------------:|:-----:|:------:|:------------:|:-----------:|:-----------:|:-------------:|:--------------:|:------:|:--------------:|
| stm32f1xx | bluepill             |![?](examplesStatus/bluepill/blink.png) runs | runs   |    runs-5    |   runs-5    |    runs-1   |     no-2      |     runs       |  runs  |     runs       |      
| stm32f3xx | discovery-stm32f303  |![?](checkMark.png) runs  | runs   |    runs-5    |   no-8,9    |    runs-1   |     no-9      |     runs       | runs-10|                |
| stm32f4xx | nucleo-64 	   | runs  | runs   |    runs-5    |    no-9     |     no-2    |     no-9      |     no-6       |  no-6  |                |
| stm32f4xx | blackpill-stm32f401  | runs  | runs   |    runs-5    |    no-9     |     runs    |     no-9      |    runs-10     | runs-10|                |
| stm32f4xx | blackpill-stm32f411  | runs  | runs   |    no-12     |    no-9     |     runs    |     no-9      |     runs       |  runs  |                |
| stm32l1xx | discovery-stm32l100  | runs  | runs   |      no      |     no      |      no     |      no       |      no        |   no   |                |


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

|    hal    |         board        |  dht  | dht11 | text_i2c | oled_gps | lora_send | lora_receive | lora_gps |
|:---------:|:--------------------:|:-----:|:-----:|:--------:|:--------:|:---------:|:------------:|:--------:|
| stm32f1xx | bluepill             | no-1  | no-1  |   runs   |   no-2   |  builds   |   builds     |  builds  |
| stm32f3xx | discovery-stm32f303  | builds| builds|   runs   |          |  builds   |   builds     |  builds  |
| stm32f4xx | nucleo-64 	   | builds| builds|   runs   |          |  builds   |   builds     |  builds  |
| stm32f4xx | blackpill-stm32f401  | no-0  | no-0  |   runs   |   runs   |  builds   |   builds     |  builds  |
| stm32f4xx | blackpill-stm32f411  | no-0  | no-0  |   runs   |   runs   |  builds   |   builds     |  builds  |
| stm32l1xx | discovery-stm32l100  |       |       |   no     |          |           |              |          |

0. panic. Timer not set right yet.
1. stall/timeout reading sensor.
2. too large for flash.

