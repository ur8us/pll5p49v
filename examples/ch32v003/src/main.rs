#![no_std]
#![no_main]

use hal::delay::Delay;
use hal::gpio::{Level, Output};
use {ch32_hal as hal, panic_halt as _};

use pll5p49v::{calibrate_vco_blocking, write_config_blocking};

use hal::i2c::I2c;
use hal::time::Hertz;

#[qingke_rt::entry]
fn main() -> ! {
    hal::debug::SDIPrint::enable();
    let mut config = hal::Config::default();
    config.rcc = hal::rcc::Config::SYSCLK_FREQ_48MHZ_HSE;
    let p = hal::init(config);

    let mut delay = Delay;

    let scl = p.PC2;
    let sda = p.PC1;
    let i2c_config = hal::i2c::Config::default();
    let mut i2c = I2c::new_blocking(p.I2C1, scl, sda, Hertz::khz(100), i2c_config);

    let clock_fq_hz = 10_000_000u32;
    let vco_fq_hz = 2_700_000_000u32;
    let (out1, out2, out3, out4) = (40_000_000, 25_000_000, 24_000_000, 28_800_000);

    if let Err(e) = write_config_blocking(&mut i2c, clock_fq_hz, vco_fq_hz, out1, out2, out3, out4)
    {
        hal::println!("Error programming PLL: {:?}", e);
    }

    if let Err(e) = calibrate_vco_blocking(&mut i2c) {
        hal::println!("Error calibrating VCO: {:?}", e);
    }

    let mut led = Output::new(p.PD6, Level::Low, Default::default());
    loop {
        led.toggle();
        delay.delay_ms(100);
    }
}
