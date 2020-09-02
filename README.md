
# Status of Examples

##  Links
- [code repository](https://github.com/pdgilbert/eg_stm_hal) 
- [main documentation](https://github.com/pdgilbert/eg_stm_hal#examples-using-embedded-rust)
- [Travis CI](https://travis-ci.org/pdgilbert/eg_stm_hal)
- [Table of Core Examples Status](#table-of-core-examples-status)
- [Table of Additional Examples Status](#table-of-additional-examples-status)


Following is the status of examples. 
The examples are run with `stm32f1xx_hal` on a `bluepill`,
`stm32f3xx_hal` on a `Discovery kit STM32F303`, `stm32l1xx_hal` on a `STM32L100C Discovery`, 
and `stm32f4xx_hal` on a `Nucleo-64 STM32F411`, a `blackpill` with MCU `stm32f401`, 
and a `blackpill` with MCU `stm32f411`.
In the table cells: 
green check marks and red X marks indicate that the CI of the example builds or fails.
`runs` is an indication that a manual test on actual hardware has been done and it works correctly or as noted, and 
`no` means the manual test fails badly as noted. The CI testing is automatic and corresponds to the most recent
example code, and using recent git versions of crates. The manual tests are not automatic, and less current.
When I remember to record it, 
clicking on `runs` will go to the code repository history for the commit when a manual test was done.
In addition to the example code at the time, the `Cargo.lock` file in the board directories gives an
indication of the versions of dependencies.


##  Table of Core Examples Status

These examples use only the indicated hal and the main embedded crates.
The names in the board column are also the names of the build directories. 
Those names link to a file recording the commit that was used for the example tests.

{% assign b01 = "[bluepill](examplesStatus/bluepill/COMMIT)" %}
{% assign b02 = "[discovery-stm32f303](examplesStatus/discovery-stm32f303/COMMIT)" %}
{% assign b03 = "[nucleo-64](examplesStatus/nucleo-64/COMMIT)" %}
{% assign b04 = "[blackpill-stm32f411](examplesStatus/blackpill-stm32f411/COMMIT)" %}
{% assign b05 = "[blackpill-stm32f401](examplesStatus/blackpill-stm32f401/COMMIT)" %}
{% assign b06 = "[none-stm32f722](examplesStatus/none-stm32f722/COMMIT)" %}
{% assign b07 = "[none-stm32h742](examplesStatus/none-stm32h742/COMMIT)" %}
{% assign b08 = "[none-stm32l0x2](examplesStatus/none-stm32l0x2/COMMIT)" %}
{% assign b09 = "[discovery-stm32l100](examplesStatus/discovery-stm32l100/COMMIT)" %}
{% assign b10 = "[heltec-stm32l151](examplesStatus/heltec-stm32l151/COMMIT)" %}
{% assign b11 = "[none-stm32l4x1](examplesStatus/none-stm32l4x1/COMMIT)" %}

{% assign db01 = "examplesStatus/bluepill/" %}
{% assign db02 = "examplesStatus/discovery-stm32f303/" %}
{% assign db03 = "examplesStatus/nucleo-64/" %}
{% assign db04 = "examplesStatus/blackpill-stm32f411/" %}
{% assign db05 = "examplesStatus/blackpill-stm32f401/" %}
{% assign db06 = "examplesStatus/none-stm32f722/" %}
{% assign db07 = "examplesStatus/none-stm32h742/" %}
{% assign db08 = "examplesStatus/none-stm32l0x2/" %}
{% assign db09 = "examplesStatus/discovery-stm32l100/" %}
{% assign db10 = "examplesStatus/heltec-stm32l151/" %}
{% assign db11 = "examplesStatus/none-stm32l4x1/" %}

{% capture p %}width="20" alt="?" {% endcapture %}

{% capture runA %}<a href="https://github.com/pdgilbert/eg_stm_hal/tree/543a6d12100c2856cbf37d978626cee47c462111" title="link to commit">runs</a>{% endcapture %}



|    hal    |  board  |                 blink                         |                  blink3                 |                     echo_by_char                        |                  echo_string                       |                   serial_char                        |                  serial_string                      |                  gps_rw_by_char                         |                  gps_rw                         |                  temperature                                                        |
|:---------:|:-------:|:-----:|:------:|:------:|:------:|:----------:|:-------------:|:--------------:|:------:|:--------------:|
| stm32f1xx | {{b01}} |<img src="{{db01}}blink.png" {{p}} /> {{runA}} |<img src="{{db01}}blink3.png" {{p}} /> runs |<img src="{{db01}}echo_by_char.png" {{p}} />   runs-5 |<img src="{{db01}}echo_string.png" {{p}} />   runs-5 |<img src="{{db01}}serial_char.png" {{p}} />   runs-1 |<img src="{{db01}}serial_string.png" {{p}} />   no-2 |<img src="{{db01}}gps_rw_by_char.png" {{p}} />   runs    |<img src="{{db01}}gps_rw.png" {{p}} />   runs    |<img src="{{db01}}temperature.png" {{p}} />   runs |      
| stm32f3xx | {{b02}} |<img src="{{db02}}blink.png" {{p}} />   runs   |<img src="{{db02}}blink3.png" {{p}} /> runs |<img src="{{db02}}echo_by_char.png" {{p}} />   runs-5 |<img src="{{db02}}echo_string.png" {{p}} />   no-8,9 |<img src="{{db02}}serial_char.png" {{p}} />   runs-1 |<img src="{{db02}}serial_string.png" {{p}} />   no-9 |<img src="{{db02}}gps_rw_by_char.png" {{p}} />   runs    |<img src="{{db02}}gps_rw.png" {{p}} />   runs-10 |<img src="{{db02}}temperature.png" {{p}} />        |
| stm32f4xx | {{b03}} |<img src="{{db03}}blink.png" {{p}} />   runs   |<img src="{{db03}}blink3.png" {{p}} /> runs |<img src="{{db03}}echo_by_char.png" {{p}} />   runs-5 |<img src="{{db03}}echo_string.png" {{p}} />   no-9   |<img src="{{db03}}serial_char.png" {{p}} />   no-2   |<img src="{{db03}}serial_string.png" {{p}} />   no-9 |<img src="{{db03}}gps_rw_by_char.png" {{p}} />   no-6    |<img src="{{db03}}gps_rw.png" {{p}} />   no-6    |<img src="{{db03}}temperature.png" {{p}} />        |
| stm32f4xx | {{b04}} |<img src="{{db04}}blink.png" {{p}} />   runs   |<img src="{{db04}}blink3.png" {{p}} /> runs |<img src="{{db04}}echo_by_char.png" {{p}} />   runs-5 |<img src="{{db04}}echo_string.png" {{p}} />   no-9   |<img src="{{db04}}serial_char.png" {{p}} />   runs   |<img src="{{db04}}serial_string.png" {{p}} />   no-9 |<img src="{{db04}}gps_rw_by_char.png" {{p}} />   runs-10 |<img src="{{db04}}gps_rw.png" {{p}} />   runs-10 |<img src="{{db04}}temperature.png" {{p}} />        |
| stm32f4xx | {{b05}} |<img src="{{db05}}blink.png" {{p}} />   runs   |<img src="{{db05}}blink3.png" {{p}} /> runs |<img src="{{db05}}echo_by_char.png" {{p}} />   no-12  |<img src="{{db05}}echo_string.png" {{p}} />   no-9   |<img src="{{db05}}serial_char.png" {{p}} />   runs   |<img src="{{db05}}serial_string.png" {{p}} />   no-9 |<img src="{{db05}}gps_rw_by_char.png" {{p}} />   runs    |<img src="{{db05}}gps_rw.png" {{p}} />   runs    |<img src="{{db05}}temperature.png" {{p}} />        |
| stm32f7xx | {{b06}} |<img src="{{db06}}blink.png" {{p}} />          |<img src="{{db06}}blink3.png" {{p}} />      |<img src="{{db06}}echo_by_char.png" {{p}} />          |<img src="{{db06}}echo_string.png" {{p}} />          |<img src="{{db06}}serial_char.png" {{p}} />          |<img src="{{db06}}serial_string.png" {{p}} />        |<img src="{{db06}}gps_rw_by_char.png" {{p}} />           |<img src="{{db06}}gps_rw.png" {{p}} />           |<img src="{{db06}}temperature.png" {{p}} />        |
| stm32h7xx | {{b07}} |<img src="{{db07}}blink.png" {{p}} />          |<img src="{{db07}}blink3.png" {{p}} />      |<img src="{{db07}}echo_by_char.png" {{p}} />          |<img src="{{db07}}echo_string.png" {{p}} />          |<img src="{{db07}}serial_char.png" {{p}} />          |<img src="{{db07}}serial_string.png" {{p}} />        |<img src="{{db07}}gps_rw_by_char.png" {{p}} />           |<img src="{{db07}}gps_rw.png" {{p}} />           |<img src="{{db07}}temperature.png" {{p}} />        |
| stm32l0xx | {{b08}} |<img src="{{db08}}blink.png" {{p}} />          |<img src="{{db08}}blink3.png" {{p}} />      |<img src="{{db08}}echo_by_char.png" {{p}} />          |<img src="{{db08}}echo_string.png" {{p}} />          |<img src="{{db08}}serial_char.png" {{p}} />          |<img src="{{db08}}serial_string.png" {{p}} />        |<img src="{{db08}}gps_rw_by_char.png" {{p}} />           |<img src="{{db08}}gps_rw.png" {{p}} />           |<img src="{{db08}}temperature.png" {{p}} />        |
| stm32l1xx | {{b09}} |<img src="{{db09}}blink.png" {{p}} />   runs   |<img src="{{db09}}blink3.png" {{p}} /> runs |<img src="{{db09}}echo_by_char.png" {{p}} />          |<img src="{{db09}}echo_string.png" {{p}} />          |<img src="{{db09}}serial_char.png" {{p}} />          |<img src="{{db09}}serial_string.png" {{p}} />        |<img src="{{db09}}gps_rw_by_char.png" {{p}} />           |<img src="{{db09}}gps_rw.png" {{p}} />           |<img src="{{db09}}temperature.png" {{p}} />        |
| stm32l1xx | {{b10}} |<img src="{{db09}}blink.png" {{p}} />   runs   |<img src="{{db09}}blink3.png" {{p}} /> runs |<img src="{{db09}}echo_by_char.png" {{p}} />          |<img src="{{db09}}echo_string.png" {{p}} />          |<img src="{{db09}}serial_char.png" {{p}} />          |<img src="{{db09}}serial_string.png" {{p}} />        |<img src="{{db09}}gps_rw_by_char.png" {{p}} />           |<img src="{{db09}}gps_rw.png" {{p}} />           |<img src="{{db09}}temperature.png" {{p}} />        |
| stm32l4xx | {{b11}} |<img src="{{db10}}blink.png" {{p}} />          |<img src="{{db10}}blink3.png" {{p}} />      |<img src="{{db10}}echo_by_char.png" {{p}} />          |<img src="{{db10}}echo_string.png" {{p}} />          |<img src="{{db10}}serial_char.png" {{p}} />          |<img src="{{db10}}serial_string.png" {{p}} />        |<img src="{{db10}}gps_rw_by_char.png" {{p}} />           |<img src="{{db10}}gps_rw.png" {{p}} />           |<img src="{{db10}}temperature.png" {{p}} />        |


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

## Table of Additional Examples Status

The status of the additional examples is

|    hal    | board   |                  dht                       |                  dht11                       |                  text_i2c                        |                 oled_gps                        |                  lora_send                |                   lora_receive                 |                  lora_gps                 |
|:---------:|:--------------------:|:-----:|:-----:|:--------:|:--------:|:---------:|:------------:|:--------:|
| stm32f1xx | {{b01}} |<img src="{{db01}}dht.png" {{p}} />   no-1  |<img src="{{db01}}dht11.png" {{p}} />   no-1  |<img src="{{db01}}text_i2c.png" {{p}} />   runs   |<img src="{{db01}}oled_gps.png" {{p}} />   no-2  |<img src="{{db01}}lora_send.png" {{p}} />   |<img src="{{db01}}lora_receive.png" {{p}} />   |<img src="{{db01}}lora_gps.png" {{p}} />   |
| stm32f3xx | {{b02}} |<img src="{{db02}}dht.png" {{p}} />         |<img src="{{db02}}dht11.png" {{p}} />         |<img src="{{db02}}text_i2c.png" {{p}} />   runs   |<img src="{{db02}}oled_gps.png" {{p}} />         |<img src="{{db02}}lora_send.png" {{p}} />   |<img src="{{db02}}lora_receive.png" {{p}} />   |<img src="{{db02}}lora_gps.png" {{p}} />   |
| stm32f4xx | {{b03}} |<img src="{{db03}}dht.png" {{p}} />         |<img src="{{db03}}dht11.png" {{p}} />         |<img src="{{db03}}text_i2c.png" {{p}} />   runs   |<img src="{{db03}}oled_gps.png" {{p}} />         |<img src="{{db03}}lora_send.png" {{p}} />   |<img src="{{db03}}lora_receive.png" {{p}} />   |<img src="{{db03}}lora_gps.png" {{p}} />   |
| stm32f4xx | {{b04}} |<img src="{{db04}}dht.png" {{p}} />   no-0  |<img src="{{db04}}dht11.png" {{p}} />   no-0  |<img src="{{db04}}text_i2c.png" {{p}} />   runs   |<img src="{{db04}}oled_gps.png" {{p}} />   runs  |<img src="{{db04}}lora_send.png" {{p}} />   |<img src="{{db04}}lora_receive.png" {{p}} />   |<img src="{{db04}}lora_gps.png" {{p}} />   |
| stm32f4xx | {{b05}} |<img src="{{db05}}dht.png" {{p}} />   no-0  |<img src="{{db05}}dht11.png" {{p}} />   no-0  |<img src="{{db05}}text_i2c.png" {{p}} />   runs   |<img src="{{db05}}oled_gps.png" {{p}} />   runs  |<img src="{{db05}}lora_send.png" {{p}} />   |<img src="{{db05}}lora_receive.png" {{p}} />   |<img src="{{db05}}lora_gps.png" {{p}} />   |
| stm32f7xx | {{b06}} |<img src="{{db06}}dht.png" {{p}} />         |<img src="{{db06}}dht11.png" {{p}} />         |<img src="{{db06}}text_i2c.png" {{p}} />          |<img src="{{db06}}oled_gps.png" {{p}} />         |<img src="{{db06}}lora_send.png" {{p}} />   |<img src="{{db06}}lora_receive.png" {{p}} />   |<img src="{{db06}}lora_gps.png" {{p}} />   |
| stm32h7xx | {{b07}} |<img src="{{db06}}dht.png" {{p}} />         |<img src="{{db06}}dht11.png" {{p}} />         |<img src="{{db06}}text_i2c.png" {{p}} />          |<img src="{{db06}}oled_gps.png" {{p}} />         |<img src="{{db06}}lora_send.png" {{p}} />   |<img src="{{db06}}lora_receive.png" {{p}} />   |<img src="{{db06}}lora_gps.png" {{p}} />   |
| stm32l0xx | {{b08}} |<img src="{{db06}}dht.png" {{p}} />         |<img src="{{db06}}dht11.png" {{p}} />         |<img src="{{db06}}text_i2c.png" {{p}} />          |<img src="{{db06}}oled_gps.png" {{p}} />         |<img src="{{db06}}lora_send.png" {{p}} />   |<img src="{{db06}}lora_receive.png" {{p}} />   |<img src="{{db06}}lora_gps.png" {{p}} />   |
| stm32l1xx | {{b09}} |<img src="{{db06}}dht.png" {{p}} />         |<img src="{{db06}}dht11.png" {{p}} />         |<img src="{{db06}}text_i2c.png" {{p}} />          |<img src="{{db06}}oled_gps.png" {{p}} />         |<img src="{{db06}}lora_send.png" {{p}} />   |<img src="{{db06}}lora_receive.png" {{p}} />   |<img src="{{db06}}lora_gps.png" {{p}} />   |
| stm32l1xx | {{b10}} |<img src="{{db06}}dht.png" {{p}} />         |<img src="{{db06}}dht11.png" {{p}} />         |<img src="{{db06}}text_i2c.png" {{p}} />          |<img src="{{db06}}oled_gps.png" {{p}} />         |<img src="{{db06}}lora_send.png" {{p}} />   |<img src="{{db06}}lora_receive.png" {{p}} />   |<img src="{{db06}}lora_gps.png" {{p}} />   |
| stm32l4xx | {{b11}} |<img src="{{db06}}dht.png" {{p}} />         |<img src="{{db06}}dht11.png" {{p}} />         |<img src="{{db06}}text_i2c.png" {{p}} />          |<img src="{{db06}}oled_gps.png" {{p}} />         |<img src="{{db06}}lora_send.png" {{p}} />   |<img src="{{db06}}lora_receive.png" {{p}} />   |<img src="{{db06}}lora_gps.png" {{p}} />   |

0. panic. Timer not set right yet.
1. stall/timeout reading sensor.
2. too large for flash.

