use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::Peripherals;
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed, Pin, AnyPin};
use embassy_stm32::spi::{Config, Spi, MODE_0, MODE_1, MODE_2, MODE_3, BitOrder, Instance, Error};
use embassy_time::{Duration, Timer};
use embassy_stm32::time::Hertz;
use {defmt_rtt as _, panic_probe as _};



pub struct TMI1638<'d, 'e, E: Instance, Tx, Rx>{
    stb: Output<'e, AnyPin>,
    spi: Spi<'d, E, Tx, Rx>,
}

#[repr(u8)]
#[derive(Copy, Clone)]
enum Symbol {
    D0=0b11_1111,
    D1=0b110,
    D2=0b101_1011,
    D3=0b100_1111,
    D4=0b110_0110,
    D5=0b110_1101,
    D6=0b111_1101,
    D7=0b111,
    D8=0b111_1111u8,
    None=0b0000_0000,
}

impl From<char> for Symbol{
    fn from(value: char) -> Self {
        match value {
            '0'=> Symbol::D0,
            '1'=> Symbol::D1,
            '2'=> Symbol::D2,
            '3'=> Symbol::D3,
            '4'=> Symbol::D4,
            '5'=> Symbol::D5,
            '6'=> Symbol::D6,
            '7'=> Symbol::D7,
            '8'=> Symbol::D8,
            _ => Symbol::None,
        }
    }
}
impl From<u8> for Symbol{
    fn from(value: u8) -> Self {
        Symbol::from(((value%10)+48) as char) // 0=>'0'
    }
}

struct FullSymbol {
    symbol: Symbol,
    dot: bool,
    extra_led: bool,
}

impl FullSymbol{
    fn to_spi(&self)->(u8,u8){
        return ( if self.dot {self.symbol as u8 | 0b1000_0000} else {self.symbol as u8},
                 if self.extra_led { 1 } else { 0 })
    }
}





impl<'d,'e,  E: Instance, Tx:embassy_stm32::spi::TxDma<E>, Rx:embassy_stm32::spi::RxDma<E>> TMI1638<'d, 'e, E, Tx, Rx>{
    pub fn new(p:Peripherals)-> Self{
        let mut config = Config::default();
        config.mode = MODE_0;
        config.bit_order = BitOrder::LsbFirst;
        let mut spi:Spi::<E, Tx, Rx> = Spi::new_txonly(
            p.SPI1,
            p.PA5, //SPI1_SCK
            p.PA7, // SPI1_MOSI
            p.DMA2_CH2,
            p.DMA2_CH0,
            Hertz(500_000),
            config,
        );
        cortex_m::asm::delay(100_000);
        let mut stb = Output::new(p.PB10, Level::High, Speed::VeryHigh);
        Self{
            stb,
            spi,
        }
    }
    pub async fn send(&mut self, data: &[u8])->Result<(), Error>{
        self.stb.set_low();
        let result = self.spi.write(&data).await;
        self.stb.set_high();
        cortex_m::asm::delay(100_000);
        return result
    }

    pub async fn turn_on(&mut self, brightness: u8){
        //  displayControl_unused0_on_brightness(0..0b111);
        let write = 0b10_00_1_000u8 + (brightness&0b111u8);
        self.send(&[write]);
    }

    pub async fn turn_off(&mut self){
        //  displayControl_unused0_off_brightness(0..0b111);
        let write = 0b10_00_0_000u8;
        self.send(&[write]);
    }
}