## Broodmother

**Crackhead engineering, Vol. 1: Clump of wires.**\
WIP!

By [`@exoudueux`](https://github.com/exoudueux) and [`@mxxntype`](https://github.com/mxxntype)

## The board's schematic

```text
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

## Developer setup

```nushell
# Prepare a Rust toolchain.
rustup toolchain install nightly
rustup override set nightly

# Compile a binary for the board.
nix develop -c just build

# Flash the binary to the board.
nix develop -c just flash
```
