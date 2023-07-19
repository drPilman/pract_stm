#[repr(u8)]
#[derive(Copy, Clone)]
pub(crate) enum Control {
    TurnOn = 0b10_00_1_111u8,
    TurnOff = 0b10_00_0_000u8,
}
#[repr(u8)]
#[derive(Copy, Clone)]
pub(crate) enum Mode {
    Fixed = 0b01_00_01_00u8,
    Auto = 0b01_00_00_00u8,
}
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Char {
    D0 = 0b011_1111,
    D1 = 0b000_0110,
    D2 = 0b101_1011,
    D3 = 0b100_1111,
    D4 = 0b110_0110,
    D5 = 0b110_1101,
    D6 = 0b111_1101,
    D7 = 0b000_0111,
    D8 = 0b111_1111,
    D9 = 0b110_1111,
    None = 0b0000_0000,
}
#[derive(Copy, Clone)]
pub enum FullChar {
    Dot(Char),
    No(Char),
    Custom(u8),
}
impl FullChar {
    pub(crate) fn as_byte(&self) -> u8 {
        match self {
            FullChar::Dot(c) => { (*c as u8) | 0b1000_0000 }
            FullChar::No(c) => { *c as u8 }
            FullChar::Custom(c) => { *c }
        }
    }
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
impl<T> From<T> for FullChar where Char: From<T>{
    fn from(value: T) -> Self {
        FullChar::No(Char::from(value))
    }
}
pub(crate) const NULL_ADDRESS: u8 = 0b1100_0000;