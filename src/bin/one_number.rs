#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use embassy_stm32::spi::{Config, Spi, MODE_0, MODE_1, MODE_2, MODE_3, BitOrder};
use embassy_time::{Duration, Timer};
use embassy_stm32::time::Hertz;
use {defmt_rtt as _, panic_probe as _};



// impl From<Symbol> for u8{
//     fn from(value: Symbol) -> Self {
//         value as u8
//     }
// }



#[embassy_executor::main]
async fn mm(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");
    let mut config = Config::default();
    config.mode = MODE_0;
    config.bit_order = BitOrder::LsbFirst;

    let mut spi = Spi::new_txonly(
        p.SPI1,
        p.PA5, //SPI1_SCK
        p.PA7, // SPI1_MOSI
        p.DMA2_CH2,
        p.DMA2_CH0,
        Hertz(500_000),
        config,
    );



    // These are the pins for the Inventek eS-Wifi SPI Wifi Adapter.
    //
    // let _boot = Output::new(p.PB12, Level::Low, Speed::VeryHigh);
    // let _wake = Output::new(p.PB13, Level::Low, Speed::VeryHigh);
    // let mut reset = Output::new(p.PE8, Level::Low, Speed::VeryHigh);
    let mut cs = Output::new(p.PB10, Level::High, Speed::VeryHigh);
    // let ready = Input::new(p.PE1, Pull::Up);

    cortex_m::asm::delay(100_000);
    // reset.set_high();
    // cortex_m::asm::delay(100_000);

    // while ready.is_low() {
    //     info!("waiting for ready");
    // }

    let write = [0b01000000u8];
    // let mut read = [0; 10];
    cs.set_low();
    spi.write(&write).await.ok();
    // spi.transfer(&mut read, &write).await.ok();
    cs.set_high();

    cortex_m::asm::delay(100);
    let l:u8 = Symbol::None as u8;
    let mut data =  [l; 17];
    data[0] = 0b11000000u8;
    data[1] = Symbol::from(2u8) as u8;
    data[2] = 1;
    data[3] = Symbol::D8 as u8;
    // let mut read = [0; 10];
    cs.set_low();
    spi.write(&data).await.ok();
    // spi.transfer(&mut read, &write).await.ok();
    cs.set_high();

    // cortex_m::asm::delay(100);
    // let mut data =  [0u16; 8];
    // let s0 = Symbol::from(2u8) as u8;
    // let s1 = 0b1000_0000u8;
    // data[0] = ((s0 as u16) << 8) | s1 as u16;
    // // let mut read = [0; 10];
    // cs.set_low();
    // spi.write(&[0b11000000u8, s0, s1]).await.ok();
    // // spi.write(&data).await.ok();
    // // spi.transfer(&mut read, &write).await.ok();
    // cs.set_high();

    cortex_m::asm::delay(100);

    let write = [0b10_00_1_100u8];
    // let mut read = [0; 10];
    cs.set_low();
    spi.write(&write).await.ok();
    // spi.transfer(&mut read, &write).await.ok();
    cs.set_high();

    // info!("xfer {=[u8]:x}", read);
    Timer::after(Duration::from_millis(1000)).await;
    let mut led = Output::new(p.PC13, Level::Low, Speed::Low);

    loop {
        led.set_high();
        Timer::after(Duration::from_millis(1000)).await;
        led.set_low();
        Timer::after(Duration::from_millis(100)).await;
    }
}