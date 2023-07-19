mod reg;

use core::cmp::min;
use cortex_m::asm::delay;
use embassy_stm32::{Peripheral, into_ref};
use embassy_stm32::gpio::{Level, Output, Pull, Speed, Pin, Flex, AnyPin};
pub use reg::{Char, FullChar};
use reg::Control;

#[non_exhaustive]
pub struct TMI1638<'d> {
    stb: Output<'d, AnyPin>,
    clk: Output<'d, AnyPin>,
    dio: Flex<'d, AnyPin>,
    leds: u8,
    buttons: u8,
    digits: [u8; 8],
    mode: reg::Mode,
}

impl<'d> TMI1638<'d> {
    pub fn new<STB: Pin, CLK: Pin, DIO: Pin>(stb: impl Peripheral<P=STB> + 'd, clk: impl Peripheral<P=CLK> + 'd, dio: impl Peripheral<P=DIO> + 'd) -> Self {
        into_ref!(stb, clk, dio);
        let stb = Output::new(stb, Level::High, Speed::Low).degrade();
        let clk = Output::new(clk, Level::High, Speed::Low).degrade();
        let mut dio = Flex::new(dio).degrade();
        dio.set_as_input_output(Speed::Low, Pull::Up);
        dio.set_high();
        let mut display = Self {
            stb,
            clk,
            dio,
            leds: 0x00,
            buttons: 0,
            digits: [0u8; 8],
            mode: reg::Mode::Auto,
        };
        display.send(display.mode as u8, &[]);
        display.update();
        display.turn_on(7);
        display
    }
    fn update(&mut self) {
        self.set_mode(reg::Mode::Auto);
        let mut buffer = [0u8; 16];
        for i in 0..8 {
            buffer[i << 1] = self.digits[i];
            buffer[(i << 1) + 1] = (self.leds >> i) & 1;
        }
        self.send(reg::NULL_ADDRESS as u8, &buffer)
    }
    fn set_mode(&mut self, mode: reg::Mode) {
        if self.mode as u8 != mode as u8 {
            self.send(mode as u8, &[]);
            self.mode = mode;
        }
    }
    fn send(&mut self, command: u8, data: &[u8]) {
        self.stb.set_low();
        self.send_byte(command);
        for byte in data {
            self.send_byte(*byte);
        }
        self.stb.set_high();
    }
    pub fn update_buttons(&mut self) {
        let mut buffer = [0u8; 4];
        self.stb.set_low();
        self.send_byte((self.mode as u8) | 0b10);
        self.dio.set_high();
        delay(10); // ???
        for i in 0..4 {
            for j in 0..8u8 {
                self.clk.set_low();
                if self.dio.is_high() {
                    buffer[i] |= 1 << j
                } else {
                    buffer[i] &= !(1 << j);
                }
                self.clk.set_high();
            }
        }
        self.stb.set_high();
        self.buttons = 0;
        for i in 0..4 {
            self.buttons |= (buffer[i] & 1) << i;
            self.buttons |= ((buffer[i] >> 4) & 1) << (i + 4);
        }
    }
    pub fn is_button_pressed(&self, pos: u8) -> bool {
        (self.buttons >> (pos & 0b111)) & 1 == 1
    }
    pub fn is_any_button_pressed(&self) -> bool {
        self.buttons != 0
    }
    fn send_byte(&mut self, byte: u8) {
        for i in 0..8u8 {
            self.clk.set_low();
            self.dio.set_level(Level::from(byte & (1 << i) != 0));
            self.clk.set_high();
        }
    }
    pub fn print_at(&mut self, symbol: impl Into<FullChar>, pos: usize) {
        let pos = pos & 0b111;
        self.set_mode(reg::Mode::Fixed);
        let char = symbol.into().as_byte();
        self.digits[pos] = char;
        self.send(reg::NULL_ADDRESS + (pos << 1) as u8, &[char]);
    }
    pub fn set_led(&mut self, is_on: bool, pos: usize) {
        let mask = 1u8 << (pos as u8);
        self.set_mode(reg::Mode::Fixed);
        if is_on {
            self.leds |= mask;
        } else {
            self.leds &= !mask;
        }
        self.send(reg::NULL_ADDRESS + ((pos as u8) << 1u8) + 1, &[is_on as u8]);
    }
    pub fn print_number(&mut self, mut number: u8) {
        self.digits = [0u8; 8];
        for i in 0..8 {
            self.digits[7 - i] = FullChar::from(number % 10).as_byte();
            number /= 10;
            if number == 0 { break; }
        }
        self.update();
    }
    pub fn turn_on(&mut self, brightness: u8) {
        self.send(Control::TurnOn as u8 | (brightness & 0b111), &[]);
    }
    pub fn turn_off(&mut self) {
        self.send(Control::TurnOff as u8, &[]);
    }
}

pub trait Println<T> {
    fn println(&mut self, _symbols: T, pos: usize) {
        if pos > 7 { panic!("Wrong pos to write {}", pos) }
        self._println(_symbols, pos)
    }
    fn _println(&mut self, _symbols: T, pos: usize) {}
}

impl<'d> Println<&'static str> for TMI1638<'d> {
    fn _println(&mut self, symbols: &'static str, pos: usize) {
        // self.digits = [0u8; 8];
        let t = symbols.as_bytes();
        for i in pos..min( t.len(), 8 - pos) {
            self.digits[pos + i] = <char as Into<FullChar>>::into(t[i] as char).as_byte();
        }
        self.update();
    }
}

impl<'d, T: Into<FullChar> + Copy> Println<&[T]> for TMI1638<'d> {
    fn _println(&mut self, symbols: &[T], pos: usize) {
        // self.digits = [0u8; 8];
        for i in 0..min(symbols.len(), 8 - pos) {
            self.digits[pos + i] = symbols[i].into().as_byte();
        }
        self.update();
    }
}