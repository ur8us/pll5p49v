#![no_std]
#![no_main]

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::i2c::{Config, I2c, InterruptHandler};
use panic_probe as _;

use pll5p49v::{calibrate_vco, write_config};

// Bind IRQs
embassy_rp::bind_interrupts!(struct Irqs {
    I2C1_IRQ => InterruptHandler<embassy_rp::peripherals::I2C1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut led = Output::new(p.PIN_25, Level::Low);

    let sda = p.PIN_14;
    let scl = p.PIN_15;
    let mut i2c = I2c::new_async(p.I2C1, scl, sda, Irqs, Config::default());

    let clock_fq_hz = 10_000_000u32;
    let vco_fq_hz = 2_700_000_000u32;
    let (out1, out2, out3, out4) = (40_000_000, 25_000_000, 24_000_000, 28_800_000);

    if let Err(e) = write_config(&mut i2c, clock_fq_hz, vco_fq_hz, out1, out2, out3, out4).await {
        defmt::error!("Error programming PLL: {:?}", e);
    }

    if let Err(e) = calibrate_vco(&mut i2c).await {
        defmt::error!("Error calibrating VCO: {:?}", e);
    }

    led.set_high();
    defmt::info!("Finished!");
}
