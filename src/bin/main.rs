#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::Pin;
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
async fn multi<'d, STB: Pin, CLK: Pin, DIO: Pin>(display: &mut TMI1638<'d, STB, CLK, DIO, true>){

    println!("multi");
    let mut digits = [Digit::new(); 8];
    let mut buttons = ButtonsBuffer::new();
    loop {
        display.read(&mut buttons).await;
        if buttons.is_pressed(1){
            return;
        }
        for i in 0..8 {
            if buttons.is_pressed(i) {
                display.print_at(i as u8,digits[i].inc() ).await;
            }
        }
    }
}
async fn one<'d, STB: Pin, CLK: Pin, DIO: Pin>(display: &mut TMI1638<'d, STB, CLK, DIO, false>){
    let mut number = 0u32;
    println!("one");
    let mut t=0u32;
    let mut buttons = ButtonsBuffer::new();
    let mut buffer = DisplayBuffer::new();
    loop {
        display.read(&mut buttons).await;
        if !buttons.is_any_pressed(){
            continue
        }
        if buttons.is_pressed(0) {
            return;
        }

        if buttons.is_pressed(7) {
            number += 1;
            if number == 1_0000_0000 {
                number = 0
            }
        }
        if buttons.is_pressed(6) {
            if number == 0 {
                number = 9999_9999u32
            } else {
                number -= 1;
            }
        }
        t=number;
        for i in 0..8{
            Char::from((t%10u32) as u8).to(7-i, &mut buffer);
            t=t/10;
        }
        display.println(&buffer).await;

    }

}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {

    let p = embassy_stm32::init(Default::default());

    let mut display_a = TMI1638::new(p.PB10, p.PB2, p.PB1).await;// auto


    // // let mut buffer = DisplayBuffer::new();
    // // let zero = Char::D0;
    // // for i in 0..8 {
    // //     zero.to(i, &mut buffer);
    // // }
    // // display.println(&mut buffer).await;
    //
    display_a.exec(command::Control::TurnOn).await;
    //
    //
    // // let mut state = DemoMode::One;
    //
    // // let mut buttons = ButtonsBuffer::new();
    //
    loop {
        one(&mut display_a).await;
        let mut display = display_a.to_fixed().await;
        multi(&mut display).await;
        display_a = display.to_auto().await;
    }




}