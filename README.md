**A simple Renesas 5P49V clock generator driver.**

Tested with 5P49V5925, 5P49V6965.

Schematics: 10 MHz clock is fed into the CLKIN pin (DC coupled, 0.9v level), CLKINB is tied to GND. XIN, XOUT, CLKSEL, SD/OE and OUT0_SEL_I2CB inputs are left floating. 

Embassy, async, tested on RP2350B, MSPM0G3507SRHBR.

Inspired by https://gitlab.com/berkowski/tca9535-rs and https://github.com/daniestevez/ADF4158_SW

The library is using the embedded_hal_async::i2c::I2c trait to pass I2C object to its functions. This makes it compatible to most microcontrollers with Embassy support.

**Usage: add the following dependency to your Cargo.toml:**

[dependencies]
pll5p49v = { version = "0.1.0", git = "https://github.com/ur8us/pll5p49v" }

**Examples:**

cd examples/rp2350
cd examples/mspm0g3507

then

cargo run



 
