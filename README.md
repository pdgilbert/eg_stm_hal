
# Status of Examples

##  Links
- [code repository](https://github.com/pdgilbert/eg_stm_hal) 
- [main documentation](https://github.com/pdgilbert/eg_stm_hal#examples-using-embedded-rust)
- [Travis CI](https://travis-ci.org/pdgilbert/eg_stm_hal)
- [Table of Core Examples Status](#table-of-core-examples-status)
- [Table of Additional Examples Status](#table-of-additional-examples-status)


Following is the status of examples. 
The examples are run with `stm32f1xx_hal` on a `bluepill`,
`stm32f3xx_hal` on a `Discovery kit STM32F303`, `stm32l1xx_hal` on a `STM32L100C Discovery` 
and  a `heltec-lora-node151`, `stm32f4xx_hal` on a `Nucleo-64 STM32F411`, a `blackpill` with MCU `stm32f401`, 
and a `blackpill` with MCU `stm32f411`.
In the table cells: 
green check marks and red X marks indicate that the CI of the example builds or fails.
`runs` is an indication that a manual test on actual hardware has been done and it works correctly or as noted, and 
`no` means the manual test fails badly as noted. The CI testing is automatic and corresponds to the most recent
example code, and using recent git versions of crates. Note however, if the build fails before checking
examples, the old status marks will be left in place and not indicated the proper status. This can happen,
for example, because a repository has disappeared. To check for that it is necessary to consult the
[Travis CI](https://travis-ci.org/pdgilbert/eg_stm_hal).
The manual tests are not automatic, and less current.
When I remember to record it, 
clicking on `runs` will go to the code repository history for the commit when a manual test was done.
In addition to the example code at the time, the `Cargo.lock` file in the board directories gives an
indication of the versions of dependencies.


##  Table of Core Examples Status

These examples use only the indicated hal and the main embedded crates.
The names in the board column are also the names of the build directories. 
Those names link to a file recording the commit that was used for the last CI example build tests.

{% assign b01 = "[bluepill](examplesStatus/bluepill/COMMIT)" %}
{% assign b02 = "[discovery-stm32f303](examplesStatus/discovery-stm32f303/COMMIT)" %}
{% assign b03 = "[nucleo-64](examplesStatus/nucleo-64/COMMIT)" %}
{% assign b04 = "[blackpill-stm32f411](examplesStatus/blackpill-stm32f411/COMMIT)" %}
{% assign b05 = "[blackpill-stm32f401](examplesStatus/blackpill-stm32f401/COMMIT)" %}
{% assign b06 = "[none-stm32f722](examplesStatus/none-stm32f722/COMMIT)" %}
{% assign b07 = "[none-stm32h742](examplesStatus/none-stm32h742/COMMIT)" %}
{% assign b08 = "[none-stm32l0x2](examplesStatus/none-stm32l0x2/COMMIT)" %}
{% assign b09 = "[discovery-stm32l100](examplesStatus/discovery-stm32l100/COMMIT)" %}
{% assign b10 = "[heltec-lora-node151](examplesStatus/heltec-lora-node151/COMMIT)" %}
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
{% assign db10 = "examplesStatus/heltec-lora-node151/" %}
{% assign db11 = "examplesStatus/none-stm32l4x1/" %}

{% capture p %}width="20" alt="?" {% endcapture %}

<!--   commit is given by   git rev-parse HEAD   -->
{% capture runA %}<a href="https://github.com/pdgilbert/eg_stm_hal/tree/9031bbbcbb00bc7e4c1108b9dcfa5a529455e82e" title="link to commit">runs</a>{% endcapture %}
{% capture runB %}<a href="https://github.com/pdgilbert/eg_stm_hal/tree/5d68a2ade698493226fc1d1f7c00f3fb2abd2ae0" title="link to commit">runs</a>{% endcapture %}
{% capture runC %}<a href="https://github.com/pdgilbert/eg_stm_hal/tree/84601e2517dd30f163d627983013004870e41b52" title="link to commit">runs</a>{% endcapture %}

{% capture blink          %}<a href="https://github.com/pdgilbert/eg_stm_hal/blob/master/examples/blink.rs"          title="link to example source code">blink</a>                   {% endcapture %}
{% capture blink3         %}<a href="https://github.com/pdgilbert/eg_stm_hal/blob/master/examples/blink3.rs"         title="link to example source code">blink3</a>                  {% endcapture %}
{% capture echo_by_char   %}<a href="https://github.com/pdgilbert/eg_stm_hal/blob/master/examples/echo_by_char.rs"   title="link to example source code">echo_by_char</a>            {% endcapture %}
{% capture echo_string    %}<a href="https://github.com/pdgilbert/eg_stm_hal/blob/master/examples/echo_string.rs"    title="link to example source code">echo_string</a>             {% endcapture %}
{% capture serial_char    %}<a href="https://github.com/pdgilbert/eg_stm_hal/blob/master/examples/serial_char.rs"    title="link to example source code">serial_char</a>             {% endcapture %}
{% capture serial_string  %}<a href="https://github.com/pdgilbert/eg_stm_hal/blob/master/examples/serial_string.rs"  title="link to example source code">serial_string</a>           {% endcapture %}
{% capture gps_rw_by_char %}<a href="https://github.com/pdgilbert/eg_stm_hal/blob/master/examples/gps_rw_by_char.rs" title="link to example source code">gps_rw_by_char</a>          {% endcapture %}
{% capture gps_rw         %}<a href="https://github.com/pdgilbert/eg_stm_hal/blob/master/examples/gps_rw.rs"         title="link to example source code">gps_rw</a>                  {% endcapture %}
{% capture temperature    %}<a href="https://github.com/pdgilbert/eg_stm_hal/blob/master/examples/temperature.rs"    title="link to example source code">temperature</a>             {% endcapture %}

{% capture dht            %}<a href="https://github.com/pdgilbert/eg_stm_hal/blob/master/examples/dht.rs"            title="link to example source code">dht</a>                     {% endcapture %}
{% capture dht11          %}<a href="https://github.com/pdgilbert/eg_stm_hal/blob/master/examples/dht11.rs"          title="link to example source code">dht11</a>                   {% endcapture %}
{% capture text_i2c       %}<a href="https://github.com/pdgilbert/eg_stm_hal/blob/master/examples/text_i2c.rs"       title="link to example source code">text_i2c</a>                {% endcapture %}
{% capture oled_gps       %}<a href="https://github.com/pdgilbert/eg_stm_hal/blob/master/examples/oled_gps.rs"       title="link to example source code">oled_gps</a>                {% endcapture %}
{% capture lora_send      %}<a href="https://github.com/pdgilbert/eg_stm_hal/blob/master/examples/lora_send.rs"      title="link to example source code">lora_send</a>               {% endcapture %}
{% capture lora_receive   %}<a href="https://github.com/pdgilbert/eg_stm_hal/blob/master/examples/lora_receive.rs"   title="link to example source code">lora_receive</a>            {% endcapture %}
{% capture lora_gps       %}<a href="https://github.com/pdgilbert/eg_stm_hal/blob/master/examples/lora_gps.rs"       title="link to example source code">lora_gps</a>                {% endcapture %}


|    hal    |  board  |                {{blink}}                      |                {{blink3}}                      |                   {{echo_by_char}}                     |                 {{echo_string}}                       |                {{serial_char}}                         |                {{serial_string}}                    |                {{gps_rw_by_char}}                       |                {{gps_rw}}                       |                  {{temperature}}                    |
|:---------:|:-------:|:-----:|:------:|:------:|:------:|:----------:|:-------------:|:--------------:|:------:|:--------------:|
| stm32f1xx | {{b01}} |<img src="{{db01}}blink.png" {{p}} /> {{runA}} |<img src="{{db01}}blink3.png" {{p}} /> {{runA}} |<img src="{{db01}}echo_by_char.png" {{p}} /> {{runA}}-5 |<img src="{{db01}}echo_string.png" {{p}} /> {{runA}}-5 |<img src="{{db01}}serial_char.png" {{p}} />  {{runA}}-1 |<img src="{{db01}}serial_string.png" {{p}} />   no-2 |<img src="{{db01}}gps_rw_by_char.png" {{p}} /> {{runA}}    |<img src="{{db01}}gps_rw.png" {{p}} /> {{runA}}    |<img src="{{db01}}temperature.png" {{p}} /> {{runA}} |      
| stm32f3xx | {{b02}} |<img src="{{db02}}blink.png" {{p}} /> {{runB}} |<img src="{{db02}}blink3.png" {{p}} /> {{runB}} |<img src="{{db02}}echo_by_char.png" {{p}} /> {{runB}}-5 |<img src="{{db02}}echo_string.png" {{p}} />   no-8,9   |<img src="{{db02}}serial_char.png" {{p}} />  {{runB}}-1 |<img src="{{db02}}serial_string.png" {{p}} />   no-9 |<img src="{{db02}}gps_rw_by_char.png" {{p}} /> {{runB}}-10 |<img src="{{db02}}gps_rw.png" {{p}} /> {{runB}}-10 |<img src="{{db02}}temperature.png" {{p}} />        |
| stm32f4xx | {{b03}} |<img src="{{db03}}blink.png" {{p}} />   runs   |<img src="{{db03}}blink3.png" {{p}} /> {{runB}} |<img src="{{db03}}echo_by_char.png" {{p}} /> {{runB}}-5 |<img src="{{db03}}echo_string.png" {{p}} />   no-9     |<img src="{{db03}}serial_char.png" {{p}} />   no-2      |<img src="{{db03}}serial_string.png" {{p}} />   no-9 |<img src="{{db03}}gps_rw_by_char.png" {{p}} />   no-6      |<img src="{{db03}}gps_rw.png" {{p}} />   no-6      |<img src="{{db03}}temperature.png" {{p}} />        |
| stm32f4xx | {{b04}} |<img src="{{db04}}blink.png" {{p}} /> {{runB}} |<img src="{{db04}}blink3.png" {{p}} /> {{runB}} |<img src="{{db04}}echo_by_char.png" {{p}} />   no-12    |<img src="{{db04}}echo_string.png" {{p}} />   no-9     |<img src="{{db04}}serial_char.png" {{p}} />  {{runB}}   |<img src="{{db04}}serial_string.png" {{p}} />   no-9 |<img src="{{db04}}gps_rw_by_char.png" {{p}} /> {{runB}}-10 |<img src="{{db04}}gps_rw.png" {{p}} /> {{runB}}    |<img src="{{db04}}temperature.png" {{p}} />        |
| stm32f4xx | {{b05}} |<img src="{{db05}}blink.png" {{p}} /> {{runB}} |<img src="{{db05}}blink3.png" {{p}} /> {{runB}} |<img src="{{db05}}echo_by_char.png" {{p}} />   no-12    |<img src="{{db05}}echo_string.png" {{p}} />   no-9     |<img src="{{db05}}serial_char.png" {{p}} />  {{runB}}   |<img src="{{db05}}serial_string.png" {{p}} />   no-9 |<img src="{{db05}}gps_rw_by_char.png" {{p}} /> {{runB}}-10 |<img src="{{db05}}gps_rw.png" {{p}} /> {{runB}}-10 |<img src="{{db05}}temperature.png" {{p}} />        |
| stm32f7xx | {{b06}} |<img src="{{db06}}blink.png" {{p}} />          |<img src="{{db06}}blink3.png" {{p}} />          |<img src="{{db06}}echo_by_char.png" {{p}} />            |<img src="{{db06}}echo_string.png" {{p}} />            |<img src="{{db06}}serial_char.png" {{p}} />             |<img src="{{db06}}serial_string.png" {{p}} />        |<img src="{{db06}}gps_rw_by_char.png" {{p}} />             |<img src="{{db06}}gps_rw.png" {{p}} />             |<img src="{{db06}}temperature.png" {{p}} />        |
| stm32h7xx | {{b07}} |<img src="{{db07}}blink.png" {{p}} />          |<img src="{{db07}}blink3.png" {{p}} />          |<img src="{{db07}}echo_by_char.png" {{p}} />            |<img src="{{db07}}echo_string.png" {{p}} />            |<img src="{{db07}}serial_char.png" {{p}} />             |<img src="{{db07}}serial_string.png" {{p}} />        |<img src="{{db07}}gps_rw_by_char.png" {{p}} />             |<img src="{{db07}}gps_rw.png" {{p}} />             |<img src="{{db07}}temperature.png" {{p}} />        |
| stm32l0xx | {{b08}} |<img src="{{db08}}blink.png" {{p}} />          |<img src="{{db08}}blink3.png" {{p}} />          |<img src="{{db08}}echo_by_char.png" {{p}} />            |<img src="{{db08}}echo_string.png" {{p}} />            |<img src="{{db08}}serial_char.png" {{p}} />             |<img src="{{db08}}serial_string.png" {{p}} />        |<img src="{{db08}}gps_rw_by_char.png" {{p}} />             |<img src="{{db08}}gps_rw.png" {{p}} />             |<img src="{{db08}}temperature.png" {{p}} />        |
| stm32l1xx | {{b09}} |<img src="{{db09}}blink.png" {{p}} />   runs   |<img src="{{db09}}blink3.png" {{p}} /> {{runC}} |<img src="{{db09}}echo_by_char.png" {{p}} /> {{runC}}-5 |<img src="{{db09}}echo_string.png" {{p}} />            |<img src="{{db09}}serial_char.png" {{p}} />  {{runC}}   |<img src="{{db09}}serial_string.png" {{p}} />        |<img src="{{db09}}gps_rw_by_char.png" {{p}} />   no-6      |<img src="{{db09}}gps_rw.png" {{p}} />  no-6       |<img src="{{db09}}temperature.png" {{p}} />        |
| stm32l1xx | {{b10}} |<img src="{{db10}}blink.png" {{p}} /> {{runC}} |<img src="{{db10}}blink3.png" {{p}} /> {{runC}} |<img src="{{db10}}echo_by_char.png" {{p}} /> {{runC}}-5 |<img src="{{db10}}echo_string.png" {{p}} />            |<img src="{{db10}}serial_char.png" {{p}} />    no-4     |<img src="{{db10}}serial_string.png" {{p}} />   no-4 |<img src="{{db10}}gps_rw_by_char.png" {{p}} />   no-4      |<img src="{{db10}}gps_rw.png" {{p}} />  no-6       |<img src="{{db10}}temperature.png" {{p}} />        |
| stm32l4xx | {{b11}} |<img src="{{db11}}blink.png" {{p}} />          |<img src="{{db11}}blink3.png" {{p}} />          |<img src="{{db11}}echo_by_char.png" {{p}} />            |<img src="{{db11}}echo_string.png" {{p}} />            |<img src="{{db11}}serial_char.png" {{p}} />             |<img src="{{db11}}serial_string.png" {{p}} />        |<img src="{{db11}}gps_rw_by_char.png" {{p}} />             |<img src="{{db11}}gps_rw.png" {{p}} />             |<img src="{{db11}}temperature.png" {{p}} />        |


1.  tx2 to rx3 works. tx3 to rx2 works sometimes but sometimes fails unwrapping err value Overrun on receive.
2.  Stalls waiting to receive. Possibly need thread to receive started before send?
3.  Usart2 with Usart3 connection works both ways but jibberish written on console.
4.  The Heltec lora_node 151 uses USART2 and USART3 pins for on board LoRa connections and power detection,
so, only USART1 is available. This means that examples using more than one USART cannot be run. In examples 
`oled_gps` and `lora_gps` the available USART1 is used for the GPS so they might work. 
5.  Works as long as typing is slow.
6.  Fails reading gps (does not return). 
7.  Works once, repeat problems.
8.  Writeln! macro missing from stm32f3xx ?
9.  Uses dma buffering in stm32f1xx. Have not figured out how to do that with other HALs.
10. Some lines miss beginning, truncated, or characters missing.
11. Overrun error.
12. no echo.

## Table of Additional Examples Status

The status of the additional examples is

|    hal    | board   |                {{dht}}                     |                {{dht11}}                    |                 {{text_i2c}}                      |                {{oled_gps}}                      |                {{lora_send}}               |                {{lora_receive}}               |                {{lora_gps}}               |
|:---------:|:--------------------:|:-----:|:-----:|:--------:|:--------:|:---------:|:------------:|:--------:|
| stm32f1xx | {{b01}} |<img src="{{db01}}dht.png" {{p}} />   no-2  |<img src="{{db01}}dht11.png" {{p}} />   no-2  |<img src="{{db01}}text_i2c.png" {{p}} /> {{runA}} |<img src="{{db01}}oled_gps.png" {{p}} />   no-3   |<img src="{{db01}}lora_send.png" {{p}} />   |<img src="{{db01}}lora_receive.png" {{p}} />   |<img src="{{db01}}lora_gps.png" {{p}} />   |
| stm32f3xx | {{b02}} |<img src="{{db02}}dht.png" {{p}} />         |<img src="{{db02}}dht11.png" {{p}} />         |<img src="{{db02}}text_i2c.png" {{p}} /> {{runB}} |<img src="{{db02}}oled_gps.png" {{p}} /> {{runB}}-4 |<img src="{{db02}}lora_send.png" {{p}} />   |<img src="{{db02}}lora_receive.png" {{p}} />   |<img src="{{db02}}lora_gps.png" {{p}} />   |
| stm32f4xx | {{b03}} |<img src="{{db03}}dht.png" {{p}} />         |<img src="{{db03}}dht11.png" {{p}} />         |<img src="{{db03}}text_i2c.png" {{p}} /> {{runB}} |<img src="{{db03}}oled_gps.png" {{p}} />          |<img src="{{db03}}lora_send.png" {{p}} />   |<img src="{{db03}}lora_receive.png" {{p}} />   |<img src="{{db03}}lora_gps.png" {{p}} />   |
| stm32f4xx | {{b04}} |<img src="{{db04}}dht.png" {{p}} />   no-1  |<img src="{{db04}}dht11.png" {{p}} />   no-1  |<img src="{{db04}}text_i2c.png" {{p}} /> {{runB}} |<img src="{{db04}}oled_gps.png" {{p}} /> {{runB}} |<img src="{{db04}}lora_send.png" {{p}} />   |<img src="{{db04}}lora_receive.png" {{p}} />   |<img src="{{db04}}lora_gps.png" {{p}} />   |
| stm32f4xx | {{b05}} |<img src="{{db05}}dht.png" {{p}} />   no-1  |<img src="{{db05}}dht11.png" {{p}} />   no-1  |<img src="{{db05}}text_i2c.png" {{p}} /> {{runB}} |<img src="{{db05}}oled_gps.png" {{p}} /> {{runB}} |<img src="{{db05}}lora_send.png" {{p}} />   |<img src="{{db05}}lora_receive.png" {{p}} />   |<img src="{{db05}}lora_gps.png" {{p}} />   |
| stm32f7xx | {{b06}} |<img src="{{db06}}dht.png" {{p}} />         |<img src="{{db06}}dht11.png" {{p}} />         |<img src="{{db06}}text_i2c.png" {{p}} />          |<img src="{{db06}}oled_gps.png" {{p}} />          |<img src="{{db06}}lora_send.png" {{p}} />   |<img src="{{db06}}lora_receive.png" {{p}} />   |<img src="{{db06}}lora_gps.png" {{p}} />   |
| stm32h7xx | {{b07}} |<img src="{{db07}}dht.png" {{p}} />         |<img src="{{db07}}dht11.png" {{p}} />         |<img src="{{db07}}text_i2c.png" {{p}} />          |<img src="{{db07}}oled_gps.png" {{p}} />          |<img src="{{db07}}lora_send.png" {{p}} />   |<img src="{{db07}}lora_receive.png" {{p}} />   |<img src="{{db07}}lora_gps.png" {{p}} />   |
| stm32l0xx | {{b08}} |<img src="{{db08}}dht.png" {{p}} />         |<img src="{{db08}}dht11.png" {{p}} />         |<img src="{{db08}}text_i2c.png" {{p}} />          |<img src="{{db08}}oled_gps.png" {{p}} />          |<img src="{{db08}}lora_send.png" {{p}} />   |<img src="{{db08}}lora_receive.png" {{p}} />   |<img src="{{db08}}lora_gps.png" {{p}} />   |
| stm32l1xx | {{b09}} |<img src="{{db09}}dht.png" {{p}} />         |<img src="{{db09}}dht11.png" {{p}} />         |<img src="{{db09}}text_i2c.png" {{p}} /> {{runC}} |<img src="{{db09}}oled_gps.png" {{p}} />   no-5   |<img src="{{db09}}lora_send.png" {{p}} />   |<img src="{{db09}}lora_receive.png" {{p}} />   |<img src="{{db09}}lora_gps.png" {{p}} />   |
| stm32l1xx | {{b10}} |<img src="{{db10}}dht.png" {{p}} />         |<img src="{{db10}}dht11.png" {{p}} />         |<img src="{{db10}}text_i2c.png" {{p}} /> {{runC}} |<img src="{{db10}}oled_gps.png" {{p}} />   no-5   |<img src="{{db10}}lora_send.png" {{p}} />   |<img src="{{db10}}lora_receive.png" {{p}} />   |<img src="{{db10}}lora_gps.png" {{p}} />   |
| stm32l4xx | {{b11}} |<img src="{{db11}}dht.png" {{p}} />         |<img src="{{db11}}dht11.png" {{p}} />         |<img src="{{db11}}text_i2c.png" {{p}} />          |<img src="{{db11}}oled_gps.png" {{p}} />          |<img src="{{db11}}lora_send.png" {{p}} />   |<img src="{{db11}}lora_receive.png" {{p}} />   |<img src="{{db11}}lora_gps.png" {{p}} />   |

1. panic. Timer not set right yet.
2. stall/timeout reading sensor.
3. too large for flash.
4. not very reliably. Better on 5v than 3v.
5.  Fails reading gps (does not return). 
