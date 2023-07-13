use defmt::{panic, unreachable, println};
use embassy_stm32::{Peripheral, into_ref};
use embassy_stm32::gpio::{Level, Input, Pull, Speed, Pin, OutputOpenDrain, AnyPin};
use embassy_stm32::peripherals::TIM4;
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};


/// 5 rows in OutputOpenDrain without pull.
/// - high=> hi-z
/// - low=> to vss
///
///
/// 4 cols in input with pull up
///
#[non_exhaustive]
pub struct KeyPad<'d, R0: Pin, R1: Pin, R2: Pin, R3: Pin, R4: Pin, C0: Pin, C1: Pin, C2: Pin, C3: Pin> {
    rows: [Rows<'d, R0, R1, R2, R3, R4>; 5],
    cols: [Cols<'d, C0, C1, C2, C3>; 4],
    buffer: u32,
}

enum Rows<'d, R0: Pin, R1: Pin, R2: Pin, R3: Pin, R4: Pin> {
    P0(OutputOpenDrain<'d, R0>),
    P1(OutputOpenDrain<'d, R1>),
    P2(OutputOpenDrain<'d, R2>),
    P3(OutputOpenDrain<'d, R3>),
    P4(OutputOpenDrain<'d, R4>),
}

enum Cols<'d, T0: Pin, T1: Pin, T2: Pin, T3: Pin> {
    P0(Input<'d, T0>),
    P1(Input<'d, T1>),
    P2(Input<'d, T2>),
    P3(Input<'d, T3>),
}

fn init_row<'d, T: Pin>(pin: impl Peripheral<P=T> + 'd) -> OutputOpenDrain<'d, T> {
    into_ref!(pin);
    OutputOpenDrain::new(pin,
                         Level::High,
                         Speed::Low,
                         Pull::None)
}

fn init_col<'d, T: Pin>(pin: impl Peripheral<P=T> + 'd) -> Input<'d, T> {
    into_ref!(pin);
    Input::new(pin, Pull::Up)
}

impl<'d, R0: Pin, R1: Pin, R2: Pin, R3: Pin, R4: Pin, C0: Pin, C1: Pin, C2: Pin, C3: Pin> KeyPad<'d, R0, R1, R2, R3, R4, C0, C1, C2, C3> {
    pub fn new(r0: impl Peripheral<P=R0> + 'd,
               r1: impl Peripheral<P=R1> + 'd,
               r2: impl Peripheral<P=R2> + 'd,
               r3: impl Peripheral<P=R3> + 'd,
               r4: impl Peripheral<P=R4> + 'd,
               c0: impl Peripheral<P=C0> + 'd,
               c1: impl Peripheral<P=C1> + 'd,
               c2: impl Peripheral<P=C2> + 'd,
               c3: impl Peripheral<P=C3> + 'd) -> Self {
        Self {
            rows: [
                Rows::P0(init_row(r0)),
                Rows::P1(init_row(r1)),
                Rows::P2(init_row(r2)),
                Rows::P3(init_row(r3)),
                Rows::P4(init_row(r4)),
            ],
            cols: [
                Cols::P0(init_col(c0)),
                Cols::P0(init_col(c1)),
                Cols::P0(init_col(c2)),
                Cols::P0(init_col(c3)),
            ],
            buffer: 0
        }
    }

    // async fn read(&mut self){
    //     for row in &self.rows{
    //
    //         for col
    //     }
    // }
    fn read_row<TT: >(){

    }

}

