/*!
# The Arduino UNO

```not-rust
                             +-----+
+----[PWR]-------------------| USB |--+
|                            +-----+  |
|         GND/RST2  [ ][ ]            |
|       MOSI2/SCK2  [ ][ ]  A5/SCL[ ] |   C5
|          5V/MISO2 [ ][ ]  A4/SDA[ ] |   C4
|                             AREF[ ] |
|                              GND[ ] |
| [ ]N/C      +---+         SCK/13[ ] |   B5
| [ ]IOREF   -| A |-       MISO/12[ ] |   .
| [ ]RST     -| T |-       MOSI/11[ ]~|   .
| [ ]3V3     -| M |-            10[ ]~|   .
| [ ]5v      -| E |-             9[ ]~|   .
| [ ]GND     -| G |-             8[ ] |   B0
| [ ]GND     -| A |-                  |
| [ ]Vin     -| 3 |-             7[ ] |   D7
|            -| 2 |-             6[ ]~|   .
| [ ]A0      -| 8 |-             5[ ]~|   .
| [ ]A1      -| P |-             4[ ] |   .
| [ ]A2       +---+         INT1/3[ ]~|   .
| [ ]A3                     INT0/2[ ] |   .
| [ ]A4/SDA  RST SCK MISO     TX>1[ ] |   .
| [ ]A5/SCL  [ ] [ ] [ ]      RX<0[ ] |   D0
|            [ ] [ ] [ ]              |
|  UNO_R3    GND MOSI 5V  ____________/
\_______________________/
```

Source: <http://busyducks.com/ascii-art-arduinos>
*/

#![no_std]
#![no_main]

use arduino_hal::{delay_ms, pins, Peripherals};
use panic_halt as _;

/// Blink the builtin LED - the "Hello World" of embedded programming.
#[arduino_hal::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = pins!(dp);

    // `d13` is also connected to an onboard LED marked "L"
    let mut onboard_led = pins.d13.into_output();
    onboard_led.set_high();

    loop {
        onboard_led.set_high();
        delay_ms(500);
        onboard_led.set_low();
        delay_ms(500);
    }
}
