#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_time::Timer;

use py32_hal::gpio::{Level, Output, Speed};
use py32_hal::i2c::I2c;
use py32_hal::time::Hertz;
use py32_hal::{bind_interrupts, i2c, peripherals};
use {defmt_rtt as _, panic_probe as _};

use pll5p49v::{calibrate_vco_async, write_config_async};

bind_interrupts!(struct Irqs {
    I2C1 => i2c::GlobalInterruptHandler<peripherals::I2C1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Hello world!");
    let p = py32_hal::init(Default::default());

    let scl = p.PA3;
    let sda = p.PA2;

    let mut i2c = I2c::new(
        p.I2C1,
        scl,
        sda,
        Irqs,
        p.DMA1_CH2,
        p.DMA1_CH1,
        Hertz(100_000),
        Default::default(),
    );

    let clock_fq_hz = 10_000_000u32;
    let vco_fq_hz = 2_700_000_000u32;
    let (out1, out2, out3, out4) = (40_000_000, 25_000_000, 24_000_000, 28_800_000);

    if let Err(e) =
        write_config_async(&mut i2c, clock_fq_hz, vco_fq_hz, out1, out2, out3, out4).await
    {
        error!("Error programming PLL: {:?}", e);
    }

    if let Err(e) = calibrate_vco_async(&mut i2c).await {
        error!("Error calibrating VCO: {:?}", e);
    }

    let mut led = Output::new(p.PB5, Level::High, Speed::Low);
    loop {
        led.toggle();
        Timer::after_millis(1000).await;
    }
}
