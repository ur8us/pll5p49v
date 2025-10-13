use std::error::Error;

use pll5p49v::{calibrate_vco_blocking, write_config_blocking};
use rpi_pal::i2c::I2c;

fn main() -> Result<(), Box<dyn Error>> {
    // println!("Hello, world!");

    let mut i2c = I2c::new()?;

    let clock_fq_hz = 10_000_000u32;
    let vco_fq_hz = 2_700_000_000u32;
    let (out1, out2, out3, out4) = (40_000_000, 25_000_000, 24_000_000, 28_800_000);

    if let Err(e) = write_config_blocking(&mut i2c, clock_fq_hz, vco_fq_hz, out1, out2, out3, out4)
    {
        println!("Error programming PLL: {:?}", e);
    }

    if let Err(e) = calibrate_vco_blocking(&mut i2c) {
        println!("Error calibrating VCO: {:?}", e);
    }

    println!("Bone!");
    Ok(())
}
