#![feature(get_many_mut)]

use core::future::Future;
use embassy_executor::Spawner;
use embassy_stm32::{Peripheral, into_ref};
use embassy_stm32::exti::{AnyChannel, ExtiInput};
use embassy_stm32::gpio::{Level, Input, Pull, Speed, OutputOpenDrain, AnyPin};
// use embassy_futures::select;
use embassy_time::{Timer, Duration};

#[derive(Copy, Clone)]
pub struct Button<'a>{
    pub row: u8,
    pub col: u8,
    pub digit: Option<u8>,
    pub string: &'a str,
}
/// 5 rows in OutputOpenDrain without pull.
/// - high=> hi-z
/// - low=> to vss
///
///
/// 4 cols in input with pull up
///
#[non_exhaustive]
pub struct KeyPad<'d, const ROW_N:usize, const COL_N:usize> {
    rows: [OutputOpenDrain<'d, AnyPin>; ROW_N],
    cols: [ExtiInput<'d, AnyPin>; COL_N],
    pub buffer: u32,
    mapping: [[&'d str; COL_N]; ROW_N],
}
fn init_row<'d>(pin: AnyPin) -> OutputOpenDrain<'d, AnyPin> {
    into_ref!(pin);
    OutputOpenDrain::new(pin,
                         Level::High,
                         Speed::Low,
                         Pull::None)
}
fn init_col<'d>(pin_and_channel: (AnyPin, AnyChannel)) -> ExtiInput<'d, AnyPin> {
    let pin = pin_and_channel.0;
    into_ref!(pin);
    ExtiInput::new(Input::new(pin, Pull::Up).degrade(), pin_and_channel.1)
}
// async fn waiting<'d>(pin: &mut ExtiInput<'d, AnyPin>) -> Future<Output=()> {
//     pin.wait_for_falling_edge()
// }

impl<'d, const ROW_N:usize, const COL_N:usize> KeyPad<'d, ROW_N, COL_N> {
    pub fn new(rows: [AnyPin; ROW_N],
               cols: [(AnyPin, AnyChannel); COL_N],
               mapping: [[&'d str; COL_N];ROW_N]) -> Self {
        Self {
            rows: rows.map(init_row),
            cols: cols.map(init_col),
            buffer: 0,
            mapping,
        }
    }

    pub fn is_pressed(&self, row: usize, col: usize) -> bool {
        ((self.buffer >> (COL_N*row + col)) & 1) == 1
    }

    pub fn get_one_key(&self)->Option<Button>{
        for row in 0..ROW_N{
            for col in 0..COL_N{
                if self.is_pressed(row,col){
                    let t = self.mapping[row][col];
                    return Some(Button{
                        row: row as u8,
                        col: col as u8,
                        digit: t.parse().ok(),
                        string: t,
                    })
                }
            }
        }
        return None
    }
    pub async fn update(&mut self) {
        let mut mask = 1u32;
        for row in 0..ROW_N {
            self.rows[row].set_low();
            for col in 0..COL_N {
                // let b = 1u32 << ((row << 2) + col);
                if self.cols[col].is_low() {
                    self.buffer |= mask;
                } else {
                    self.buffer &= !mask;
                }
                mask <<= 1;
            }
            self.rows[row].set_high();
            Timer::after(Duration::from_micros(1)).await;
        }
    }
    pub async fn wait_none(&mut self){
        loop {
            self.update().await;
            if self.buffer.count_ones() == 0 {
                return
            }
            Timer::after(Duration::from_micros(10)).await;
        }
    }

    pub async fn wait_one(&mut self) -> Option<Button> {
        loop {
            self.update().await;
            if self.buffer.count_ones() == 1 {
                return self.get_one_key();
            }
            Timer::after(Duration::from_micros(10)).await;
        }
    }
    //
    // pub async fn wait_any(&mut self, spawner: Spawner) {
    //     for row in 0..ROW_N {
    //         self.rows[row].set_low();
    //     }
    //     for col in &mut self.cols{
    //         spawner.spawn(waiting_for_col(col)).unwrap();
    //     }
    //     // let t = self.cols.map();
    //     let t:[Future<Output=()>;COL_N];
    //     select::select_array(self.cols.iter_mut().map(|x:&mut ExtiInput<'_, AnyPin>| async{x.wait_for_falling_edge()})).await;
    //     // maybe  split_first_mut or unsafe
    //     if let Ok([col0, col1, col2, col3]) = self.cols.get_many_mut([0, 1, 2, 3]) {
    //         select::select4(
    //             col0.wait_for_falling_edge(),
    //             col1.wait_for_falling_edge(),
    //             col2.wait_for_falling_edge(),
    //             col3.wait_for_falling_edge(),
    //         ).await;
    //     }
    //     for row in 0..ROW_N {
    //         self.rows[row].set_high();
    //     }
    // }
}

