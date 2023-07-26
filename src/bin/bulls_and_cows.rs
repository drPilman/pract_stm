#![no_main]

use cortex_m::register::control::Control;
use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::interrupt::InterruptExt;
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};
use smt32_project::display::*;
use smt32_project::display::{Char, FullChar};
use smt32_project::keypad::{KeyPad, Button};

use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;
use rand::RngCore;

trait Bounded {
    fn inc(&mut self, right: u8);
    fn dec(&mut self, right: u8);
}

impl Bounded for u8 {
    fn inc(&mut self, right: u8) {
        *self += 1;
        if *self == right {
            *self = 0;
        }
    }
    fn dec(&mut self, right: u8) {
        if *self == 0 {
            *self = right - 1;
        } else {
            *self -= 1;
        }
    }
}

fn check_is_uniq(arr: &[u8; 4]) -> bool {
    for i in 0..3 {
        for j in i + 1..4 {
            if arr[i] == arr[j] {
                return false;
            }
        }
    }
    true
}

const ERR_: [FullChar; 4] = [
    FullChar::Custom(0b0111_1001),
    FullChar::Custom(0b0011_0001),
    FullChar::Custom(0b0011_0001),
    FullChar::No(Char::None)
];

const CHAR_COW: FullChar = FullChar::Custom(0b0011_1001);
const CHAR_BULL: FullChar = FullChar::Custom(0b0111_1111);

fn gen(array: &mut [u8;4], small_rng: &mut SmallRng){
    for i in 0..4{

        let d ='outer : loop{
            let rand_num = (small_rng.next_u32()%10) as u8;
            for j in 0..i{
                if array[j]==rand_num{
                    continue 'outer ;
                }
            }
            break rand_num;
        };
        array[i] = d;
    }
}

#[embassy_executor::task]
pub async fn game(mut display: TMI1638<'static>, mut keypad: KeyPad<'static, 5,4>) {

    let mut small_rng = SmallRng::seed_from_u64(1232145778);
    //
    // let mut seed = [0; 8];
    // rng.fill_bytes(&mut seed);
    // let seed = u64::from_le_bytes(seed);

    let mut secret: [u8; 4] = [0, 1, 2, 3];



    let mut user = [0u8; 4];
    display.println(&user[..], 4);
    // for i in 0..4 {
    //     display.print_at(secret[i], 4 + i);
    // }

    let mut pos: u8 = 0;
    display.print_at(FullChar::Dot(Char::from(user[pos as usize])), (pos as usize) + 4);
    'game: loop {
        gen(&mut secret, &mut small_rng);
        println!("{:?}", secret);

        let win = 'read: loop {
            if let Some(b) = keypad.wait_one().await {
                let last_pos = pos;
                if let Some(digit) = b.digit {
                    user[pos as usize] = digit;
                    display.print_at(FullChar::Dot(Char::from(user[pos as usize])), (pos as usize) + 4);
                    pos.inc(4)
                } else {
                    match b.string {
                        "UP" => { user[pos as usize].inc(10); }
                        "DOWN" => { user[pos as usize].dec(10); }
                        "LEFT" => { pos.dec(4); }
                        "RIGHT" => { pos.inc(4); }
                        "ESC" => { break false; }
                        "ENTER" => {
                            if !check_is_uniq(&user) {
                                display.println(&ERR_[..], 0);
                                continue;
                            }
                            let bulls: u8 = {
                                let mut c = 0;
                                for i in 0..4 {
                                    if user[i] == secret[i] { c += 1 }
                                }
                                c
                            };

                            let cows: u8 = {
                                let mut c = 0;
                                for i in 0..4 {
                                    for j in 0..4 {
                                        if i != j && user[i] == secret[j] {
                                            c += 1;
                                        }
                                    }
                                }
                                c
                            };

                            display.print_at(CHAR_BULL, 0);
                            display.print_at(CHAR_COW, 2);
                            display.print_at(cows, 3);
                            display.print_at(bulls, 1);
                            if bulls == 4 {
                                break 'read true;
                            }
                        }
                        _ => { continue; }
                    }

                }

                display.print_at(FullChar::No(Char::from(user[last_pos as usize])), (last_pos as usize) + 4);
                display.print_at(FullChar::Dot(Char::from(user[pos as usize])), (pos as usize) + 4);
                keypad.wait_none().await;
            }
        };
        if win {
            let mut c: u8 = 0;
            let mut one_time_low = false;
            loop {
                display.set_led(false, c as usize);
                c.inc(8);
                display.set_led(true, c as usize);
                keypad.update().await;
                if keypad.buffer.count_ones()==0 {one_time_low=true}
                if one_time_low && keypad.buffer.count_ones()!=0{break}
                Timer::after(Duration::from_millis(50)).await;
            }
            display.set_led(false, c as usize);
        }


        display.println("____", 0);
    }
    // display.print_at('0', 0);
    // display.print_at(1, 1);
    // display.set_led(false, 3);
    // display.println(&[0u8,1,2,3,4,5,6,7][..]);
    // display.println("____0119");
    // display.println(&[0,1,2,3,4,5,6,7][..]);

    // let mut led = Output::new(p.PC13, Level::Low, Speed::Low);
    // loop {
    //     display.update_buttons();
    //     Timer::after(Duration::from_millis(10)).await;
    //     if display.is_any_button_pressed() {
    //         println!("high {}", display.is_button_pressed(5));
    //     } else {
    //         Timer::after(Duration::from_micros(1)).await;
    //     }
    //
    //     // println!("high");
    //     // led.set_high();
    //     // Timer::after(Duration::from_millis(1000)).await;
    //     // display.println(&[Char::D6][..]);
    //     // // display_a.set_led(false, 3);
    //     //
    //     // println!("low");
    //     // led.set_low();
    //     // Timer::after(Duration::from_millis(1000)).await;
    //     // display.println("____0119");
    //     // // display_a.set_led(true, 3);
    // }
}