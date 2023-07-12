#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
// use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};
use smt32_project::display::*;

// struct Dog{
//
// }
// enum State{
//     ST1,
//     ST2,
// }
//
// struct Cons<const T:bool>{
//
// }
//
// trait True{}
//
// impl<T> True for Cons<{  }> {
//
// }
#[derive(Copy, Clone)]
struct Digit {
    data: u8,
    symbol: FullCharLED,
}

impl Digit {
    fn inc(&mut self)->&FullCharLED {
        self.data = (self.data + 1) % 10;
        if self.data == 0 {
            self.symbol.full_char.dot ^= true;
            self.symbol.led.is_on = true;
        }
        self.symbol.full_char._char = Char::from(self.data);
        // println!("{:?}", self.symbol.full_char._char.as_byte());
        &self.symbol
    }
    fn new() -> Self {
        Self {
            data: 0,
            symbol:
            FullCharLED {
                full_char: FullChar {
                    _char: Char::D0,
                    dot: false,
                },
                led: LED { is_on: false },
            },
        }
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    // println!("???");

    let mut display = TMI1638::new(p.PB1, p.PB2, p.PB10).await;// auto

    let mut buffer = DisplayBuffer::new();
    let zero = Char::D0;
    for i in 0..8 {
        zero.to(i, &mut buffer);
    }

    display.println(&mut buffer).await;


    display.exec(command::Control::TurnOn).await;

    let mut display = display.to_fixed().await;

    let mut digits = [Digit::new(); 8];



    let mut buttons = ButtonsBuffer::new();
    loop {
        display.read(&mut buttons).await;
        for i in 0..8 {
            if buttons.is_pressed(i) {
                display.print_at(i as u8,digits[i].inc() ).await;
            }
        }
    }
    // c.print_at(0, display); ERORR {auto cant print_at}

    // let mut buffer = DisplayBuffer::new();
    //
    // c.to(0,&mut buffer);
    //
    // Timer::after(Duration::from_millis(1000)).await;
    //
    // // println!("{:?}",buffer.data);
    // display.println(&buffer).await;
    // Timer::after(Duration::from_millis(1000)).await;
    //
    //
    //
    // let f = FullChar{
    //     _char: Char::D0,
    //     dot: true,
    // };
    //
    // f.to(1,&mut buffer);
    //
    // LED{is_on:true}.to(2,&mut buffer);
    // FullCharLED{ full_char: FullChar { _char: Char::D9, dot: false }, led: LED { is_on: true } }.to(3,&mut buffer);
    // println!("{:?}",buffer.data);
    // display.println(&buffer).await;


    // display.send(0b01_00_01_00, &[]).await;
    // Timer::after(Duration::from_millis(100)).await;
    // let t1 = [0b1111_0011u8; 16];
    //
    // let t2 = [0b1111_0000u8; 16];
    // loop{
    //     display.send(0b11_00_00_00, &[0b1111_0011u8]).await;
    //     Timer::after(Duration::from_millis(100)).await;
    //     display.send(0b11_00_00_00, &[0b1100_0011u8]).await;
    //     Timer::after(Duration::from_millis(100)).await;
    //
    // }

    // Timer::after(Duration::from_millis(10000)).await;
    //
    // display.exec(command::Control::TurnOff).await;
    // println!("end");
    // let mut u:[u8;4];
    // loop{
    //     u = display.read().await;
    //     println!("{:?}", u);
    // }

    //
    // let mut led = Flex::new(p.PA0);
    //
    // led.set_as_input_output(Speed::Low, Pull::Up);
    //
    // loop{
    //     led.set_high();
    //     Timer::after(Duration::from_millis(1000)).await;
    //     led.set_low();
    //     Timer::after(Duration::from_millis(500)).await;
    // }
    // let mut display = TMI1638::new(p);


    // let mut led = Flex::new(p.PA0);
    //
    // led.set_as_input_output(Speed::Low, Pull::Down);
    //
    // led.set_low();


    //
    //
    // // let display = TMI1638::new(p.PB1, p.PB2, p.PB10);
    // // display.do123();
    //
    // // These are the pins for the Inventek eS-Wifi SPI Wifi Adapter.
    // //
    // // let _boot = Output::new(p.PB12, Level::Low, Speed::VeryHigh);
    // // let _wake = Output::new(p.PB13, Level::Low, Speed::VeryHigh);
    // // let mut reset = Output::new(p.PE8, Level::Low, Speed::VeryHigh);
    // // let mut cs = Output::new(p.PB10, Level::High, Speed::VeryHigh);
    // // let ready = Input::new(p.PE1, Pull::Up);
    //
    // cortex_m::asm::delay(100_000);
    // // reset.set_high();
    // // cortex_m::asm::delay(100_000);
    //
    // // while ready.is_low() {
    // //     info!("waiting for ready");
    // // }
    //
    // let write = [0b01000000u8];
    // // let mut read = [0; 10];
    // cs.set_low();
    // spi.write(&write).await.ok();
    // // spi.transfer(&mut read, &write).await.ok();
    // cs.set_high();
    //
    // cortex_m::asm::delay(100);
    //
    // let write = [0b11000000u8, 0b11111111, 0b11111100, 0b11111000,0b11001111];
    // // let mut read = [0; 10];
    // cs.set_low();
    // spi.write(&write).await.ok();
    // // spi.transfer(&mut read, &write).await.ok();
    // cs.set_high();
    //
    // cortex_m::asm::delay(100);
    //
    // let write = [0b10_00_1_100u8];
    // // let mut read = [0; 10];
    // cs.set_low();
    // spi.write(&write).await.ok();
    // // spi.transfer(&mut read, &write).await.ok();
    // cs.set_high();
    //
    // // info!("xfer {=[u8]:x}", read);
    // Timer::after(Duration::from_millis(1000)).await;
    // let mut led = Output::new(p.PC13, Level::Low, Speed::Low);
    //
    // loop {
    //     led.set_high();
    //     Timer::after(Duration::from_millis(1000)).await;
    //     led.set_low();
    //     Timer::after(Duration::from_millis(100)).await;
    // }
}