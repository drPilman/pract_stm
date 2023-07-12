use core::any::Any;
use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{Peripheral, into_ref};
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed, Pin, AnyPin, Flex};
use embassy_stm32::spi::{Config, Spi, MODE_0, MODE_1, MODE_2, MODE_3, BitOrder, Instance, Error};
use embassy_time::{Duration, Timer};
use embassy_stm32::time::Hertz;
use {defmt_rtt as _, panic_probe as _};


pub struct TMI1638<'d, STB: Pin, CLK: Pin, DIO: Pin> {
    stb: Output<'d, STB>,
    clk: Output<'d, CLK>,
    dio: Flex<'d, DIO>,
}

trait Bits: Copy + core::ops::ShrAssign<u8>
{
    const COUNT: usize;
    fn last_bit(&self)->u8;
}

impl Bits for u8 {
    const COUNT: usize = 8;
    fn last_bit(&self)->u8{
        return *self & 1u8;
    }
}

struct Packet<DataType: Bits> {
    command: u8,
    data: [DataType],
}

// <'d, STB: Pin, CLK: Pin, DIO: Pin, DataType: Bits, const BITS_N: u8 = { DataType::COUNT }>

impl<DataType: Bits> Packet<DataType>{
    async fn send<'d, STB: Pin, CLK: Pin, DIO: Pin>(&self, display: &mut TMI1638<'d, STB, CLK, DIO>)
        where [(); { DataType::COUNT }]:{
        println!("send");
        display.stb.set_low();
        Timer::after(Duration::from_micros(500)).await;
        display.send_byte(self.command).await;
        Timer::after(Duration::from_micros(500)).await;
        for byte in &self.data {
            self.send_bits(byte, display);
        }
        display.stb.set_high();
        Timer::after(Duration::from_micros(500)).await;
    }
    fn send_bits<'d, STB: Pin, CLK: Pin, DIO: Pin>(&self, bits: &DataType, display: &mut TMI1638<'d, STB, CLK, DIO>)
        where [(); { DataType::COUNT }]: {
        // const T:u8 = ;
        display.send_bits::<DataType, {DataType::COUNT}>(*bits);
    }
}

impl<'d, STB: Pin, CLK: Pin, DIO: Pin> TMI1638<'d, STB, CLK, DIO> {
    pub fn new(stb: impl Peripheral<P=STB> + 'd, clk: impl Peripheral<P=CLK> + 'd, dio: impl Peripheral<P=DIO> + 'd) -> Self {
        into_ref!(stb, clk, dio);

        let mut stb = Output::new(stb, Level::High, Speed::Low);
        let mut clk = Output::new(clk, Level::High, Speed::Low);
        let mut dio = Flex::new(dio);
        dio.set_as_input_output(Speed::Low, Pull::Up);
        dio.set_low();

        Self {
            stb,
            clk,
            dio,
        }
    }
    async fn send_command(&mut self, command: u8) {
        println!("send");
        self.stb.set_low();
        Timer::after(Duration::from_micros(500)).await;
        self.send_byte(command).await;
        Timer::after(Duration::from_micros(500)).await;
        self.stb.set_high();
        Timer::after(Duration::from_micros(500)).await;
    }
    async fn send_command_with_data(&mut self, command: u8, data: &[u8]) {
        println!("send_with_data");
        self.stb.set_low();
        Timer::after(Duration::from_micros(500)).await;
        self.send_byte(command).await;
        for byte in data {
            self.send_byte(*byte);
        }
        Timer::after(Duration::from_micros(500)).await;
        self.stb.set_high();
        Timer::after(Duration::from_micros(500)).await;
    }

    async fn send_bits<DataType:Bits, const N_BITS: usize = {DataType::COUNT}>(&mut self, mut byte: DataType)
    {
        for _ in 0..N_BITS {
            self.clk.set_low();
            if byte.last_bit()==0 {
                println!("0");
                self.dio.set_low();
            } else {
                println!("1");
                self.dio.set_high();
            }
            byte >>= 1;
            Timer::after(Duration::from_micros(500)).await;
            self.clk.set_high();
            Timer::after(Duration::from_micros(500)).await;
        }
    }
    async fn send_byte(&mut self, mut byte: u8) {
        byte.type_id();
        for i in 0..8u8 {
            self.clk.set_low();
            if byte & 1 == 0 {
                println!("0");
                self.dio.set_low();
            } else {
                println!("1");
                self.dio.set_high();
            }
            byte >>= 1;
            Timer::after(Duration::from_micros(500)).await;
            self.clk.set_high();
            Timer::after(Duration::from_micros(500)).await;
        }
    }
    pub async fn turn_on(&mut self) {
        self.send_command(0b10_00_1_111u8).await;
    }
    pub async fn turn_off(&mut self) {
        self.send_command(0b10_00_0_000u8).await;
    }
}