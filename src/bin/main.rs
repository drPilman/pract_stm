#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use cortex_m::register::control::Control;
use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::interrupt::InterruptExt;
use embassy_time::{Duration, Timer};
// use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};
use smt32_project::display::*;
use smt32_project::display::Char;
use defmt::*;
use embassy_stm32::exti::Channel;
// use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};
use embassy_stm32::gpio::Pin;
// use smt32_project::display::*;
// use smt32_project::display::Char;
mod bulls_and_cows;
use smt32_project::keypad::KeyPad;
// use embassy_stm32::rng::Rng;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    // let core = RngCore::new();
    let display = TMI1638::new(p.PB10, p.PB2, p.PB1);// auto

    let mut keypad = KeyPad::new(
        [
            p.PB0.degrade(),
            p.PA7.degrade(),
            p.PA6.degrade(),
            p.PA5.degrade(),
            p.PA4.degrade(),
        ],
        [
            (p.PA0.degrade(), p.EXTI3.degrade()),
            (p.PA1.degrade(), p.EXTI2.degrade()),
            (p.PA2.degrade(), p.EXTI1.degrade()),
            (p.PA3.degrade(), p.EXTI0.degrade()),
        ],
        [
            ["F1","F2","#","*"],
            ["1","2","3","UP"],
            ["4","5","6","DOWN"],
            ["7","8","9","ESC"],
            ["LEFT","0","RIGHT","ENTER"]]
    );
    //
    // loop{
    //     Timer::after(Duration::from_millis(100)).await;
    //     keypad.update().await;
    //     if let Some(t) = keypad.get_one_key(){
    //         println!("col{} row{} = {} digit({})!", t.col, t.row, t.string, t.digit);
    //     }
    //
    // }


    // keypad.init();

    // let keypad = KeyPad::new(p.PB0, p.PA7, p.PA6, p.PA5, p.PA4,
    //                              p.PA3, p.PA2, p.PA1, p.PA0,
    //                              p.EXTI0.degrade(), p.EXTI1.degrade(), p.EXTI2.degrade(), p.EXTI3.degrade());

    // let t = Rng::new(p.PB12);
    _spawner.spawn(bulls_and_cows::game(display, keypad)).unwrap();


    let mut led = Output::new(p.PC13, Level::Low, Speed::Low);
    loop {
        led.set_high();
        Timer::after(Duration::from_millis(500)).await;
        led.set_low();
        Timer::after(Duration::from_millis(500)).await;
    }


    // display.print_at('0', 0);
    // display.print_at(1, 1);
    // display.set_led(false, 3);
    // display.println(&[0u8,1,2,3,4,5,6,7][..]);
    // display.println("____0119");
    // display.println(&[0,1,2,3,4,5,6,7][..]);
}