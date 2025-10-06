#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_mspm0::bind_interrupts;
use embassy_mspm0::gpio::{Level, Output};
use embassy_mspm0::i2c::{Config, I2c, InterruptHandler};
use embassy_mspm0::peripherals::I2C1;

// use embassy_time::Timer;
use {defmt_rtt as _, panic_halt as _};

use pll5p49v::{calibrate_vco_async, write_config_async};

// const ADDRESS: u8 = 0x6a;

bind_interrupts!(struct Irqs {
    I2C1 => InterruptHandler<I2C1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_mspm0::init(Default::default());

    let mut led1 = Output::new(p.PA0, Level::Low);
    led1.set_inversion(true);

    let instance = p.I2C1;
    let scl = p.PA4;
    let sda = p.PA3;

    let mut i2c = unwrap!(I2c::new_async(instance, scl, sda, Irqs, Config::default()));

    // Timer::after_micros(10).await;

    let clock_fq_hz = 10_000_000u32;
    let vco_fq_hz = 2_700_000_000u32;
    let (out1, out2, out3, out4) = (40_000_000, 25_000_000, 24_000_000, 28_800_000);

    if let Err(e) =
        write_config_async(&mut i2c, clock_fq_hz, vco_fq_hz, out1, out2, out3, out4).await
    {
        defmt::error!("Error programming PLL: {:?}", e);
    }

    // Timer::after_millis(1).await;

    if let Err(e) = calibrate_vco_async(&mut i2c).await {
        defmt::error!("Error calibrating VCO: {:?}", e);
    }

    led1.set_high();

    defmt::info!("Finished!");
}
