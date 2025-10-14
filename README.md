**A simple Renesas 5P49V clock generator driver.**

Tested with 5P49V5925, 5P49V6965 chips.

Schematics: 10 MHz clock is fed into the CLKIN pin (DC coupled, 0.9v level), CLKINB is tied to GND. XIN, XOUT, CLKSEL, SD/OE and OUT0_SEL_I2CB inputs are left floating. 

*Async:* Embassy, tested on RP2350B, MSPM0G3507SRHBR, PY32F003F18P, PY32F030K28U6, STM32F103C8T6.

The library is using the embedded_hal_async::i2c::I2c trait to pass I2C object to its functions. This makes it compatible to most microcontrollers with Embassy support.

*Blocking:* tested on CH32V003 (requires nightly build), Raspberry Pi 4, desktop Linux (using an I2C-Tiny-USB adapter)

Traits used:

embedded_hal::i2c::I2c (v1.x.x)
embedded_hal_async::i2c::I2c (v1.x.x)


Inspided by: https://gitlab.com/berkowski/tca9535-rs , https://github.com/daniestevez/ADF4158_SW 

**Usage: add the following dependency to your Cargo.toml:**

[dependencies]

pll5p49v = { version = "0.1.1", git = "https://github.com/ur8us/pll5p49v" }

**Examples:**

cd examples/rp2350

cd examples/mspm0g3507

cd examples/ch32v003

cd examples/py32f030    *- for both PY32F003 and PY32F030*

cd examples/stm32f1

cd examples/rpi4    *-64-bit OS*

cd examples/linux   *- using a I2C-Tiny-USB adapter*

*then*

cargo run 

*- or -*

cargo run --release
