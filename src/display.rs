use defmt::{panic, unreachable, println};
use embassy_stm32::{Peripheral, into_ref};
use embassy_stm32::gpio::{Level, Output, Pull, Speed, Pin, Flex};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

mod timings {
    pub const PW_CLK:u64=1;
}

#[non_exhaustive]
pub struct TMI1638<'d, STB: Pin, CLK: Pin, DIO: Pin, const IS_FIXED: bool> {
    stb: Output<'d, STB>,
    clk: Output<'d, CLK>,
    dio: Flex<'d, DIO>,
}

pub mod command {
    #[repr(u8)]
    #[derive(Copy, Clone)]
    pub enum Control {
        TurnOn = 0b10_00_1_111u8,
        TurnOff = 0b10_00_0_000u8,
    }

    #[repr(u8)]
    #[derive(Copy, Clone)]
    pub enum Mode {
        Fixed = 0b01_00_01_00u8,
        Auto = 0b01_00_00_00u8,
        ReadFixed = 0b01_00_01_10,
        ReadAuto = 0b01_00_00_10,
    }
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Char {
    D0 = 0b11_1111,
    D1 = 0b110,
    D2 = 0b101_1011,
    D3 = 0b100_1111,
    D4 = 0b110_0110,
    D5 = 0b110_1101,
    D6 = 0b111_1101,
    D7 = 0b111,
    D8 = 0b111_1111u8,
    D9 = 0b110_1111u8,
    None = 0b0000_0000,
}

impl From<char> for Char {
    fn from(value: char) -> Self {
        match value {
            '0' => Char::D0,
            '1' => Char::D1,
            '2' => Char::D2,
            '3' => Char::D3,
            '4' => Char::D4,
            '5' => Char::D5,
            '6' => Char::D6,
            '7' => Char::D7,
            '8' => Char::D8,
            '9' => Char::D9,
            _ => Char::None,
        }
    }
}

impl From<u8> for Char {
    fn from(value: u8) -> Self {
        Char::from(((value % 10) + 48) as char) // 0=>'0'
    }
}
#[derive(Copy, Clone)]
pub struct FullChar {
    pub _char: Char,
    pub dot: bool,
}
#[derive(Copy, Clone)]
pub struct LED {
    pub is_on: bool,
}
#[derive(Copy, Clone)]
pub struct FullCharLED {
    pub full_char: FullChar,
    pub led: LED,
}

pub trait Byte {
    const OFFSET: u8 = 0;
    fn as_byte(&self) -> u8;
    fn to(&self, pos: u8, buffer: &mut DisplayBuffer) {
        // println!("pos:{:?} data:{:?}", (pos << 1 + Self::OFFSET) as usize, self.as_byte());
        buffer.data[((pos << 1) + Self::OFFSET) as usize] = self.as_byte();
    }
}

impl Byte for Char {
    fn as_byte(&self) -> u8 { *self as u8 }
}

impl Byte for FullChar {
    fn as_byte(&self) -> u8 {
        let byte = self._char as u8;
        if self.dot { byte | 0b1000_0000u8 } else { byte }
    }
}

impl Byte for LED {
    const OFFSET: u8 = 1;
    fn as_byte(&self) -> u8 {
        if self.is_on { 0xff } else { 0 }
    }
}

impl Byte for FullCharLED {
    fn as_byte(&self) -> u8 {
        unreachable!()
    }
    fn to(&self, pos: u8, buffer: &mut DisplayBuffer) {
        buffer.data[(pos << 1) as usize] = self.full_char.as_byte();
        buffer.data[((pos << 1) + 1) as usize] = self.led.as_byte();
    }
}


const NULL_ADDRESS: u8 = 0b1100_0000;

pub trait PrintAt<'d, STB: Pin, CLK: Pin, DIO: Pin, const IS_FIXED: bool, T: Byte>: Send {
    async fn print_at(&mut self, pos: u8, symbol: &T) {
        if pos > 7 { panic!("Can't write at pos {}. Position must be in 0..8", pos) }
        self.send(NULL_ADDRESS + (pos << 1) + T::OFFSET, &[symbol.as_byte()]).await;
    }
}

/// impl PrintAt for TMI1638 in Fixed Mode
impl<'d, STB: Pin, CLK: Pin, DIO: Pin> PrintAt<'d, STB, CLK, DIO, true, Char> for TMI1638<'d, STB, CLK, DIO, true> {}

impl<'d, STB: Pin, CLK: Pin, DIO: Pin> PrintAt<'d, STB, CLK, DIO, true, FullChar> for TMI1638<'d, STB, CLK, DIO, true> {}

impl<'d, STB: Pin, CLK: Pin, DIO: Pin> PrintAt<'d, STB, CLK, DIO, true, LED> for TMI1638<'d, STB, CLK, DIO, true> {}

impl<'d, STB: Pin, CLK: Pin, DIO: Pin> PrintAt<'d, STB, CLK, DIO, true, FullCharLED> for TMI1638<'d, STB, CLK, DIO, true> {
    async fn print_at(&mut self, pos: u8, symbol: &FullCharLED) {
        if pos > 7 { panic!("Can't write at pos {}. Position must be in 0..8", pos) }
        self.send(NULL_ADDRESS + (pos << 1),
                  &[symbol.full_char.as_byte()]).await;
        self.send(NULL_ADDRESS + (pos << 1) + 1,
                  &[symbol.led.as_byte()]).await;
    }
}

pub struct DisplayBuffer {
    pub data: [u8; 16],
}

impl DisplayBuffer {
    pub fn new() -> Self {
        Self { data: [0u8; 16] }
    }
}

pub struct ButtonsBuffer {
    pub data: [u8; 4],
}

impl ButtonsBuffer {
    pub fn new() -> Self {
        Self { data: [0u8; 4] }
    }
    pub fn is_pressed(&self, pos: usize) -> bool {
        if pos < 4 {
            self.data[pos] & 1 != 0
        } else if pos < 8 {
            self.data[pos - 4] & 16u8 != 0
        } else {
            panic!("pos for button read must be in 0..8")
        }
    }
    pub fn is_any_pressed(&self) -> bool {
        let mut sum = 0u8;
        for byte in &self.data {
            sum |= byte;
        }
        sum != 0
    }
}

impl<'d, STB: Pin, CLK: Pin, DIO: Pin> TMI1638<'d, STB, CLK, DIO, false> {
    pub async fn new(stb: impl Peripheral<P=STB> + 'd, clk: impl Peripheral<P=CLK> + 'd, dio: impl Peripheral<P=DIO> + 'd) -> Self {
        into_ref!(stb, clk, dio);
        let stb = Output::new(stb, Level::High, Speed::Low);
        let clk = Output::new(clk, Level::High, Speed::Low);
        let mut dio = Flex::new(dio);
        dio.set_as_input_output(Speed::Low, Pull::Up);
        dio.set_high();

        let mut display = Self {
            stb,
            clk,
            dio,
        }.init().await;
        display.send(NULL_ADDRESS, &[0u8; 16]).await;
        display
    }
    async fn init(mut self) -> TMI1638<'d, STB, CLK, DIO, false> {
        self.send(command::Mode::Auto as u8, &[]).await;
        self
    }

    pub async fn to_fixed(self) -> TMI1638<'d, STB, CLK, DIO, true> {
        TMI1638::<STB, CLK, DIO, true> {
            stb: self.stb,
            clk: self.clk,
            dio: self.dio,
        }.init().await
    }
    pub async fn println(&mut self, buffer: &DisplayBuffer) {
        self.send(NULL_ADDRESS, &buffer.data).await
    }
}


impl<'d, STB: Pin, CLK: Pin, DIO: Pin> TMI1638<'d, STB, CLK, DIO, true> {
    async fn init(mut self) -> TMI1638<'d, STB, CLK, DIO, true> {
        self.send(command::Mode::Fixed as u8, &[]).await;
        self
    }
    pub async fn to_auto(self) -> TMI1638<'d, STB, CLK, DIO, false> {
        TMI1638::<STB, CLK, DIO, false> {
            stb: self.stb,
            clk: self.clk,
            dio: self.dio,
        }.init().await
    }
}

pub trait Send {
    async fn send(&mut self, _command: u8, _data: &[u8]) {}
}

impl<'d, STB: Pin, CLK: Pin, DIO: Pin, const IS_FIXED: bool> Send for TMI1638<'d, STB, CLK, DIO, IS_FIXED> {
    async fn send(&mut self, command: u8, data: &[u8]) {
        // println!("send");
        self.stb.set_low();
        // Timer::after(Duration::from_micros(timings::PW_CLK)).await;
        self.send_byte(command).await;
        for byte in data {
            // Timer::after(Duration::from_micros(timings::PW_CLK)).await;
            self.send_byte(*byte).await;
            // Timer::after(Duration::from_micros(timings::PW_CLK)).await;
        }
        // Timer::after(Duration::from_micros(timings::PW_CLK)).await;
        self.stb.set_high();
        // Timer::after(Duration::from_micros(timings::PW_CLK)).await;
    }
}

impl<'d, STB: Pin, CLK: Pin, DIO: Pin, const IS_FIXED: bool> TMI1638<'d, STB, CLK, DIO, IS_FIXED> {
    pub async fn read(&mut self, buffer: &mut ButtonsBuffer) {
        // println!("read");
        self.stb.set_low();
        // Timer::after(Duration::from_micros(timings::PW_CLK)).await;
        self.send_byte(if IS_FIXED { command::Mode::ReadFixed } else { command::Mode::ReadAuto } as u8).await;
        self.dio.set_high();

        Timer::after(Duration::from_micros(timings::PW_CLK )).await; // T_wait

        for i in 0..4 {
            for j in 0..8u8 {
                self.clk.set_low();

                // Timer::after(Duration::from_micros(timings::PW_CLK)).await;
                if self.dio.is_high() {
                    buffer.data[i] |= 1 << j
                } else {
                    buffer.data[i] &= !(1 << j);
                }
                self.clk.set_high();

                // Timer::after(Duration::from_micros(timings::PW_CLK)).await;
            }
        }

        // Timer::after(Duration::from_micros(timings::PW_CLK)).await;
        self.stb.set_high();
        // Timer::after(Duration::from_micros(timings::PW_CLK)).await;
    }
    async fn send_byte(&mut self, mut byte: u8) {
        for _ in 0..8 {
            self.clk.set_low();
            if byte & 1 == 0 {
                self.dio.set_low();
            } else {
                self.dio.set_high();
            }
            byte >>= 1;
            // Timer::after(Duration::from_micros(timings::PW_CLK)).await;
            self.clk.set_high();
            // Timer::after(Duration::from_micros(timings::PW_CLK)).await;
        }
    }
    pub async fn exec(&mut self, cmd: command::Control) {
        self.send(cmd as u8, &[]).await;
    }

    pub async fn turn_on_with_brightness(&mut self, brightness: u8) {
        self.send(command::Control::TurnOn as u8 | (brightness % 0b111), &[]).await;
    }
}