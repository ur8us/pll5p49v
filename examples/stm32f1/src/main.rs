#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

use embassy_stm32::i2c::I2c;
use embassy_stm32::{bind_interrupts, i2c, peripherals};

use pll5p49v::{calibrate_vco_async, write_config_async};

bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let mut i2c = I2c::new(
        p.I2C1,
        p.PB8,
        p.PB9,
        Irqs,
        p.DMA1_CH6,
        p.DMA1_CH7,
        Default::default(),
    );

    let clock_fq_hz = 10_000_000u32;
    let vco_fq_hz = 2_700_000_000u32;
    let (out1, out2, out3, out4) = (40_000_000, 25_000_000, 24_000_000, 28_800_000);

    if let Err(e) =
        write_config_async(&mut i2c, clock_fq_hz, vco_fq_hz, out1, out2, out3, out4).await
    {
        defmt::error!("Error programming PLL: {:?}", e);
    }

    if let Err(e) = calibrate_vco_async(&mut i2c).await {
        defmt::error!("Error calibrating VCO: {:?}", e);
    }

    let mut led = Output::new(p.PC13, Level::High, Speed::Low);

    loop {
        led.toggle();
        Timer::after_millis(100).await;
    }
}
