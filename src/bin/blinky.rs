#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, OutputOpenDrain, Output, Speed, Pull};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    println!("Hello World!");

    // let mut led = OutputOpenDrain::new(p.PB10, Level::Low, Speed::Low, Pull::None);
    //

    let mut led = Output::new(p.PC13, Level::Low, Speed::Low);
    loop {
        println!("high");
        led.set_high();
        Timer::after(Duration::from_millis(5000)).await;

        println!("low");
        led.set_low();
        Timer::after(Duration::from_millis(1000)).await;
    }
}