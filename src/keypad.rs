use cortex_m::asm::delay;
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
    rows: Rows<'d, R0, R1, R2, R3, R4>,
    cols: Cols<'d, C0, C1, C2, C3>,
    buffer: u32,
}

struct Rows<'d, R0: Pin, R1: Pin, R2: Pin, R3: Pin, R4: Pin> {
    r0: OutputOpenDrain<'d, R0>,
    r1: OutputOpenDrain<'d, R1>,
    r2: OutputOpenDrain<'d, R2>,
    r3: OutputOpenDrain<'d, R3>,
    r4: OutputOpenDrain<'d, R4>,
}

struct Cols<'d, T0: Pin, T1: Pin, T2: Pin, T3: Pin> {
    c0: Input<'d, T0>,
    c1: Input<'d, T1>,
    c2: Input<'d, T2>,
    c3: Input<'d, T3>,
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
            rows: Rows {
                r0: init_row(r0),
                r1: init_row(r1),
                r2: init_row(r2),
                r3: init_row(r3),
                r4: init_row(r4),
            },
            cols: Cols {
                c0: init_col(c0),
                c1: init_col(c1),
                c2: init_col(c2),
                c3: init_col(c3),
            },
            buffer: 0,
        }
    }

    fn read(&mut self){
        self.rows.r0.set_low();
        self.read_row( 0 );
        self.rows.r0.set_high();
        delay(1);
        // self.read_row( 1 );
        // self.read_row( 2 );
        // self.read_row( 3 );
        // self.read_row( 4 );
    }
    fn read_col(&mut self, is_low: bool, n_row:u32, n_col:u32) {
        let b = 1u32<<((n_row<<2) + n_col);
        if is_low{
            self.buffer|=b;
        }else{
            self.buffer&=!b;
        }
    }
    fn read_row(&mut self, n_row:u32) {
        self.read_col(self.cols.c0.is_low(), n_row, 0 );
        self.read_col(self.cols.c1.is_low(), n_row, 1 );
        self.read_col(self.cols.c2.is_low(), n_row, 2 );
        self.read_col(self.cols.c3.is_low(), n_row, 3 );
    }
}

