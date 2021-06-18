
# Status of Examples

##  Links
- [code repository](https://github.com/pdgilbert/eg_stm_hal) 
- [main documentation](https://github.com/pdgilbert/eg_stm_hal#examples-using-embedded-rust)
- [Github workflow CI](https://github.com/pdgilbert/eg_stm_hal/actions)
- [Table of Core Examples Status](#table-of-core-examples-status)
- [Additional Examples](#additional-examples)
- [Additional Examples CI](https://github.com/pdgilbert/rust-integration-testing/actions)


Following is the status of examples. 
The examples are run with `stm32f1xx_hal` on a `bluepill`,
`stm32f3xx_hal` on a `Discovery kit STM32F303`, `stm32l1xx_hal` on a `STM32L100C Discovery` 
and  a `heltec-lora-node151`, `stm32f4xx_hal` on a `Nucleo-64 STM32F411`, a `blackpill` with MCU `stm32f401`, 
and a `blackpill` with MCU `stm32f411`.
A board name starting with `none-` is an indication that I do not have hardware for an MCU that uses the
hal, so cannot do the manual testing myself.
(If you do run the examples on MCUs using these HALs, please report via
[the repository issues](https://github.com/pdgilbert/eg_stm_hal/issues) and I will add notes.)
In the table cells: 
green check marks and red X marks indicate that the CI of the example builds or fails.
`runs` is an indication that a manual test on actual hardware has been done and it works correctly or as noted, and 
`no` means the manual test fails badly as noted. 

As of June 2021 the Travis.org CI has stopped. The Github Workflow testing is working but the generation 
of the table below has not been migrated from Travis. It is relatively accurate (as of June 2021) but for
recent results see the [Github workflow CI.](https://github.com/pdgilbert/eg_stm_hal/actions)

The CI testing is automatic and corresponds to the most recent example code, and using recent git versions of crates. 
The manual tests are not automatic, and less current. When I remember to record it, 
clicking on `runs` will go to the code repository history for the commit when a manual test was done.
In addition to the example code at the time, the `Cargo.lock` file in the board directories gives an
indication of the versions of dependencies.


##  Table of Core Examples Status

These examples use only the indicated hal and the main embedded crates.
The names in the board column are also the names of the build directories. 
Those names link to a file recording the commit that was used for the last CI example build tests.

{% capture stm32f0 %}<a href="https://github.com/stm32-rs/stm32f0xx-hal" title="link to HAL git repository">stm32f0xx</a>{% endcapture %}
{% capture stm32f1 %}<a href="https://github.com/stm32-rs/stm32f1xx-hal" title="link to HAL git repository">stm32f1xx</a>{% endcapture %}
{% capture stm32f3 %}<a href="https://github.com/stm32-rs/stm32f3xx-hal" title="link to HAL git repository">stm32f3xx</a>{% endcapture %}
{% capture stm32f4 %}<a href="https://github.com/stm32-rs/stm32f4xx-hal" title="link to HAL git repository">stm32f4xx</a>{% endcapture %}
{% capture stm32f7 %}<a href="https://github.com/stm32-rs/stm32f7xx-hal" title="link to HAL git repository">stm32f7xx</a>{% endcapture %}
{% capture stm32h7 %}<a href="https://github.com/stm32-rs/stm32h7xx-hal" title="link to HAL git repository">stm32h7xx</a>{% endcapture %}
{% capture stm32l0 %}<a href="https://github.com/stm32-rs/stm32l0xx-hal" title="link to HAL git repository">stm32l0xx</a>{% endcapture %}
{% capture stm32l1 %}<a href="https://github.com/stm32-rs/stm32l1xx-hal" title="link to HAL git repository">stm32l1xx</a>{% endcapture %}
{% capture stm32l4 %}<a href="https://github.com/stm32-rs/stm32l4xx-hal" title="link to HAL git repository">stm32l4xx</a>{% endcapture %}

{% assign b00 = "[none-stm32f030](examplesStatus/none-stm32f030/COMMIT)" %}
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
{% assign b11 = "[none-stm32l4x2](examplesStatus/none-stm32l4x2/COMMIT)" %}

{% assign db00 = "examplesStatus/none-stm32f030/" %}
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
{% assign db11 = "examplesStatus/none-stm32l4x2/" %}

{% capture p %}width="20" alt="?" {% endcapture %}

<!--   commit is given by   git rev-parse HEAD   -->
{% capture runA %}<a href="https://github.com/pdgilbert/eg_stm_hal/tree/9031bbbcbb00bc7e4c1108b9dcfa5a529455e82e" title="link to commit">runs</a>{% endcapture %}
{% capture runB %}<a href="https://github.com/pdgilbert/eg_stm_hal/tree/5d68a2ade698493226fc1d1f7c00f3fb2abd2ae0" title="link to commit">runs</a>{% endcapture %}
{% capture runC %}<a href="https://github.com/pdgilbert/eg_stm_hal/tree/84601e2517dd30f163d627983013004870e41b52" title="link to commit">runs</a>{% endcapture %}
{% capture runD %}<a href="https://github.com/pdgilbert/eg_stm_hal/commit/6a94c3b2848c05f08e916b4a82651ad4600f56de" title="link to commit">runs</a>{% endcapture %}
{% capture runE %}<a href="https://github.com/pdgilbert/eg_stm_hal/commit/090e95515a65be88cc1e87aca8f99d382904c4a6" title="link to commit">runs</a>{% endcapture %}
{% capture runF %}<a href="https://github.com/pdgilbert/eg_stm_hal/commit/4d518d37bbb188e7eb7d776652893c7447135a8d" title="link to commit">runs</a>{% endcapture %}
{% capture runG %}<a href="https://github.com/pdgilbert/eg_stm_hal/commit/b9122fdb73cf69143f746ab5f1cd41743697f31f" title="link to commit">runs</a>{% endcapture %}
{% capture runH %}<a href="https://github.com/pdgilbert/eg_stm_hal/commit/ef3d172ee03c899de34e8491a5a4bda8f697882b" title="link to commit">runs</a>{% endcapture %}

{% capture blink          %}<a href="https://github.com/pdgilbert/eg_stm_hal/blob/master/examples/blink.rs"          title="link to example source code">blink</a>                   {% endcapture %}
{% capture blink3         %}<a href="https://github.com/pdgilbert/eg_stm_hal/blob/master/examples/blink3.rs"         title="link to example source code">blink3</a>                  {% endcapture %}
{% capture echo_by_char   %}<a href="https://github.com/pdgilbert/eg_stm_hal/blob/master/examples/echo_by_char.rs"   title="link to example source code">echo_by_char</a>            {% endcapture %}
{% capture echo_string    %}<a href="https://github.com/pdgilbert/eg_stm_hal/blob/master/examples/echo_string.rs"    title="link to example source code">echo_string</a>             {% endcapture %}
{% capture serial_char    %}<a href="https://github.com/pdgilbert/eg_stm_hal/blob/master/examples/serial_char.rs"    title="link to example source code">serial_char</a>             {% endcapture %}
{% capture serial_string  %}<a href="https://github.com/pdgilbert/eg_stm_hal/blob/master/examples/serial_string.rs"  title="link to example source code">serial_string</a>           {% endcapture %}
{% capture gps_rw         %}<a href="https://github.com/pdgilbert/eg_stm_hal/blob/master/examples/gps_rw.rs"         title="link to example source code">gps_rw</a>                  {% endcapture %}
{% capture temperature    %}<a href="https://github.com/pdgilbert/eg_stm_hal/blob/master/examples/temperature.rs"    title="link to example source code">temperature</a>             {% endcapture %}

{% capture dht            %}<a href="https://github.com/pdgilbert/eg_stm_hal/blob/master/examples/dht.rs"            title="link to example source code">dht</a>                     {% endcapture %}
{% capture dht11          %}<a href="https://github.com/pdgilbert/eg_stm_hal/blob/master/examples/dht11.rs"          title="link to example source code">dht11</a>                   {% endcapture %}
{% capture text_i2c       %}<a href="https://github.com/pdgilbert/eg_stm_hal/blob/master/examples/text_i2c.rs"       title="link to example source code">text_i2c</a>                {% endcapture %}
{% capture oled_gps       %}<a href="https://github.com/pdgilbert/eg_stm_hal/blob/master/examples/oled_gps.rs"       title="link to example source code">oled_gps</a>                {% endcapture %}
{% capture lora_spi_send    %}<a href="https://github.com/pdgilbert/eg_stm_hal/blob/master/examples/lora_spi_send.rs"         title="link to example source code">lora_spi_send</a>       {% endcapture %}
{% capture lora_spi_receive %}<a href="https://github.com/pdgilbert/eg_stm_hal/blob/master/examples/lora_spi_receive.rs" title="link to example source code">lora_spi_receive</a>    {% endcapture %}
{% capture lora_spi_gps     %}<a href="https://github.com/pdgilbert/eg_stm_hal/blob/master/examples/lora_spi_gps.rs"         title="link to example source code">lora_spi_gps</a>             {% endcapture %}
{% capture lora_send      %}<a href="https://github.com/pdgilbert/eg_stm_hal/blob/master/examples/lora_send.rs"      title="link to example source code">lora_send</a>               {% endcapture %}
{% capture lora_receive   %}<a href="https://github.com/pdgilbert/eg_stm_hal/blob/master/examples/lora_receive.rs"   title="link to example source code">lora_receive</a>            {% endcapture %}
{% capture lora_gps       %}<a href="https://github.com/pdgilbert/eg_stm_hal/blob/master/examples/lora_gps.rs"       title="link to example source code">lora_gps</a>                {% endcapture %}

|    hal      |  board  |                {{blink}}                      |                {{blink3}}                      |                   {{echo_by_char}}                     |   {{echo_string}}   |                {{serial_char}}                         |                {{serial_string}}                    |                  {{gps_rw}}                           |                      {{temperature}}                         |
|:-----------:|:-------:|:-----:|:------:|:------:|:------:|:----------:|:-------------:|:--------------:|:------:|:--------------:|
| {{stm32f0}} | {{b00}} |<img src="{{db00}}blink.png" {{p}} />          |<img src="{{db00}}blink3.png" {{p}} />          |<img src="{{db00}}echo_by_char.png" {{p}} />            |<img src="{{db00}}echo_string.png" {{p}} />            |<img src="{{db00}}serial_char.png" {{p}} />             |<img src="{{db00}}serial_string.png" {{p}} />        |<img src="{{db00}}gps_rw.png" {{p}} />             |<img src="{{db00}}temperature.png" {{p}} />          |        
| {{stm32f1}} | {{b01}} |<img src="{{db01}}blink.png" {{p}} /> {{runA}} |<img src="{{db01}}blink3.png" {{p}} /> {{runA}} |<img src="{{db01}}echo_by_char.png" {{p}} /> {{runA}}-5 |<img src="{{db01}}echo_string.png" {{p}} />{{runA}}-8,9|<img src="{{db01}}serial_char.png" {{p}} />  {{runA}}-1 |<img src="{{db01}}serial_string.png" {{p}} /> no-8,9 |<img src="{{db01}}gps_rw.png" {{p}} /> {{runA}}    |<img src="{{db01}}temperature.png" {{p}} /> {{runF}} |          
| {{stm32f3}} | {{b02}} |<img src="{{db02}}blink.png" {{p}} /> {{runB}} |<img src="{{db02}}blink3.png" {{p}} /> {{runB}} |<img src="{{db02}}echo_by_char.png" {{p}} /> {{runB}}-5 |<img src="{{db02}}echo_string.png" {{p}} />{{runD}}-5,9|<img src="{{db02}}serial_char.png" {{p}} />  {{runB}}-1 |<img src="{{db02}}serial_string.png" {{p}} />{{runE}}|<img src="{{db02}}gps_rw.png" {{p}} /> {{runB}}-10 |<img src="{{db02}}temperature.png" {{p}} />          |
| {{stm32f4}} | {{b03}} |<img src="{{db03}}blink.png" {{p}} />   runs   |<img src="{{db03}}blink3.png" {{p}} /> {{runB}} |<img src="{{db03}}echo_by_char.png" {{p}} /> {{runB}}-5 |<img src="{{db03}}echo_string.png" {{p}} />   no-9     |<img src="{{db03}}serial_char.png" {{p}} />   no-2      |<img src="{{db03}}serial_string.png" {{p}} /> no-8,9 |<img src="{{db03}}gps_rw.png" {{p}} />   no-6      |<img src="{{db03}}temperature.png" {{p}} />          |
| {{stm32f4}} | {{b04}} |<img src="{{db04}}blink.png" {{p}} /> {{runB}} |<img src="{{db04}}blink3.png" {{p}} /> {{runB}} |<img src="{{db04}}echo_by_char.png" {{p}} />   no-12    |<img src="{{db04}}echo_string.png" {{p}} />   no-9     |<img src="{{db04}}serial_char.png" {{p}} />  {{runB}}   |<img src="{{db04}}serial_string.png" {{p}} /> no-8,9 |<img src="{{db04}}gps_rw.png" {{p}} /> {{runB}}    |<img src="{{db04}}temperature.png" {{p}} /> {{runF}} |
| {{stm32f4}} | {{b05}} |<img src="{{db05}}blink.png" {{p}} /> {{runB}} |<img src="{{db05}}blink3.png" {{p}} /> {{runB}} |<img src="{{db05}}echo_by_char.png" {{p}} />   no-12    |<img src="{{db05}}echo_string.png" {{p}} />   no-9     |<img src="{{db05}}serial_char.png" {{p}} />  {{runB}}   |<img src="{{db05}}serial_string.png" {{p}} /> no-8,9 |<img src="{{db05}}gps_rw.png" {{p}} /> {{runB}}-10 |<img src="{{db05}}temperature.png" {{p}} /> {{runF}} |
| {{stm32f7}} | {{b06}} |<img src="{{db06}}blink.png" {{p}} />          |<img src="{{db06}}blink3.png" {{p}} />          |<img src="{{db06}}echo_by_char.png" {{p}} />            |<img src="{{db06}}echo_string.png" {{p}} />            |<img src="{{db06}}serial_char.png" {{p}} />             |<img src="{{db06}}serial_string.png" {{p}} />        |<img src="{{db06}}gps_rw.png" {{p}} />             |<img src="{{db06}}temperature.png" {{p}} />  no-11   |
| {{stm32h7}} | {{b07}} |<img src="{{db07}}blink.png" {{p}} />          |<img src="{{db07}}blink3.png" {{p}} />          |<img src="{{db07}}echo_by_char.png" {{p}} />            |<img src="{{db07}}echo_string.png" {{p}} />            |<img src="{{db07}}serial_char.png" {{p}} />             |<img src="{{db07}}serial_string.png" {{p}} />        |<img src="{{db07}}gps_rw.png" {{p}} />             |<img src="{{db07}}temperature.png" {{p}} />          |
| {{stm32l0}} | {{b08}} |<img src="{{db08}}blink.png" {{p}} />          |<img src="{{db08}}blink3.png" {{p}} />          |<img src="{{db08}}echo_by_char.png" {{p}} />            |<img src="{{db08}}echo_string.png" {{p}} />            |<img src="{{db08}}serial_char.png" {{p}} />             |<img src="{{db08}}serial_string.png" {{p}} />        |<img src="{{db08}}gps_rw.png" {{p}} />             |<img src="{{db08}}temperature.png" {{p}} />          |
| {{stm32l1}} | {{b09}} |<img src="{{db09}}blink.png" {{p}} />   runs   |<img src="{{db09}}blink3.png" {{p}} /> {{runC}} |<img src="{{db09}}echo_by_char.png" {{p}} /> {{runC}}-5 |<img src="{{db09}}echo_string.png" {{p}} />            |<img src="{{db09}}serial_char.png" {{p}} />  {{runC}}   |<img src="{{db09}}serial_string.png" {{p}} />        |<img src="{{db09}}gps_rw.png" {{p}} />  no-6       |<img src="{{db09}}temperature.png" {{p}} />          |
| {{stm32l1}} | {{b10}} |<img src="{{db10}}blink.png" {{p}} /> {{runC}} |<img src="{{db10}}blink3.png" {{p}} /> {{runC}} |<img src="{{db10}}echo_by_char.png" {{p}} /> {{runC}}-5 |<img src="{{db10}}echo_string.png" {{p}} />            |<img src="{{db10}}serial_char.png" {{p}} />    no-4     |<img src="{{db10}}serial_string.png" {{p}} />   no-4 |<img src="{{db10}}gps_rw.png" {{p}} />  no-6       |<img src="{{db10}}temperature.png" {{p}} />          |
| {{stm32l4}} | {{b11}} |<img src="{{db11}}blink.png" {{p}} />          |<img src="{{db11}}blink3.png" {{p}} />          |<img src="{{db11}}echo_by_char.png" {{p}} />            |<img src="{{db11}}echo_string.png" {{p}} />            |<img src="{{db11}}serial_char.png" {{p}} />             |<img src="{{db11}}serial_string.png" {{p}} />        |<img src="{{db11}}gps_rw.png" {{p}} />             |<img src="{{db11}}temperature.png" {{p}} />  no-11   |


1.  tx2 to rx3 works. tx3 to rx2 works sometimes but sometimes fails unwrapping err value Overrun on receive.
2.  Stalls waiting to receive. Possibly need thread to receive started before send?
3.  Usart2 with Usart3 connection works both ways but jibberish written on console.
4.  The Heltec lora_node 151 uses USART2 and USART3 pins for on board LoRa connections and power detection,
so, only USART1 is available. This means that examples using more than one USART cannot be run. In examples 
`oled_gps` and `lora_gps` the available USART1 is used for the GPS so they might work. 
5.  Works as long as typing is slow.
6.  Fails reading gps (does not return). 
7.  Works once, repeat problems.
8.  Current code works for stm32f3xx_hal
9.  Uses dma buffering. Structures and methods are not yet consistent across hals.
10. Some lines miss beginning, truncated, or characters missing.
11. Hal does not yet support adc.
12. no echo.

## Additional Examples

Additional examples use crates in addition to the HAL and the main embedded crates. 
There are now a fairly large number of these examples.
They are [maintained at ](https://github.com/pdgilbert/rust-integration-testing) and 
the [CI testing is at ](https://github.com/pdgilbert/rust-integration-testing/actions). The testing strategy
is more extensive than above. Examples are tested using both released and git versions of the crates.
A simple table as above is difficult to maintain, but the most recent (non-dependabot) github workflow report at 
the above CI link is usually a good summary.

Code for these are in subdirectories of the repository `examples/` directory.
The main groupings are as follows. 

### examples/misc
- `dht` uses crate [dht-sensor](https://github.com/michaelbeaumont/dht-sensor). Run tests were done with 
both DHT11 and DHT22 sensors. DHT22 is specified by adding feature `dht22`. DHT11 is used if nothing is 
specified. The code must be compiled with `--release`  to run or it is not fast enough to read the 
sensor and an `Error Timeout` occurs. The sensor values are approximately 
correct at room temperature, but the sensors have not been calibrated or tested at other temperatures.

- `text_i2c` uses crate [ssd1306](https://github.com/jamwaffles/ssd1306). Run tests have been succesful.

- `oled_gps` uses crate `embedded-graphics`. Run testing on `discovery-stm32f303` is not very reliably, 
but better on 5v than 3v. Run testing on stm32l1xx fails reading gps, it does not return.  
Run tests on other hardware have been mostly succesful.

- the directory also has the core examples in the table above so they are included in the 
expanded CI testing strategies.


### examples/driver-examples
- many examples on [Diego Barrios Romero repository](https://github.com/eldruin/driver-examples) where there are
also links to blog posts describing the examples. Versions in `examples/driver-examples` have been 
adjusted to run with various HAL crates. 
I do not have hardware to test many of these examples so they have not been run tested.

### examples/radio-sx127x has
- `lora_spi_send`, `lora_spi_receive` and `lora_spi_gps` use 
crate [rust-radio-sx127x](https://github.com/rust-iot/rust-radio-sx127x).
The crate uses `embedded-hal 1.0.0 alpha`. 
It works with hal crates built with older embedded-hal by using a `compat()` shim to satisfy traits.
These examples have been (occassionaly) run tested on `bluepill` and `blackpill stm32f411` hardware.

### examples/rtic
- many examples from [RTIC](https://rtic.rs). Not yet working in CI as they still require some manual editing
to work with different HALs. Some have been run tested on `bluepill` and `blackpill stm32f411` hardware.
