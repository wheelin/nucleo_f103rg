use stm32f103xx::{USART2, GPIOA, RCC};

use cortex_m::interrupt;

use core::fmt;

use clocks::*;

pub struct Serial;

#[derive(Copy, Clone)]
pub enum BaudRate {
    Br9600      = 9600,
    Br14400     = 14400,
    Br19200     = 19200,
    Br28800     = 28800,
    Br38400     = 38400,
    Br56000     = 56000,
    Br57600     = 57600,
    Br115200    = 115200,
}

#[derive(Copy, Clone)]
pub enum DataLength {
    DataLen8bits,
    DataLen9Bits,
}

#[derive(Copy, Clone)]
pub enum StopBits {
    Stop1,
    Stop05,
    Stop2,
    Stop15,
}

pub struct SerialConfig {
    baud_rate : Option<BaudRate>,
    data_length : Option<DataLength>,
    stop_bits : Option<StopBits>,
}

impl SerialConfig {
    pub fn new() -> SerialConfig {
        SerialConfig {
            baud_rate : None,
            data_length : None,
            stop_bits : None,
        }
    }

    pub fn baud_rate(mut self, br : BaudRate) -> SerialConfig {
        self.baud_rate = Some(br);
        self
    }

    pub fn data_length(mut self, len : DataLength) -> SerialConfig {
        self.data_length = Some(len);
        self
    }

    pub fn stop_bits(mut self, stop : StopBits) -> SerialConfig {
        self.stop_bits = Some(stop);
        self
    }

    pub fn configure(self) -> Serial {
        interrupt::free(|cs| {
            let rcc = RCC.borrow(cs);
            let gpio = GPIOA.borrow(cs);
            let uart = USART2.borrow(cs);

            rcc.apb2enr.modify(|_, w| w.iopaen().bit(true));
            rcc.apb1enr.modify(|_, w| w.usart2en().bit(true));

            gpio.crl.modify(|_, w| {
                w.cnf2().bits(0b10)
                .mode2().bits(0b11)
                .cnf3().bits(0b01)
                .mode3().bits(0b00)
            });

            let bd_reg = match self.baud_rate.clone() {
                Some(b) => {
                    let brr = SerialConfig::br_conv(ClockConfig::get_speeds().apb1_clk, b as u32);
                    ((brr << 4) & 0x0000FFF0) as u32
                },
                None => {
                    let brr = SerialConfig::br_conv(ClockConfig::get_speeds().apb1_clk, 9600);
                    ((brr << 4) & 0x0000FFF0) as u32
                },
            };

            uart.brr.write(|w| unsafe {
                w.bits(bd_reg)
            });


            match self.data_length.clone() {
                Some(d) => {
                    match d {
                        DataLength::DataLen8bits => uart.cr1.modify(|_, w| w.m().bit(false)),
                        DataLength::DataLen9Bits => uart.cr1.modify(|_, w| w.m().bit(true)),
                    };
                },
                None => uart.cr1.modify(|_, w| w.m().bit(false)),
            };

            if let Some(s) = self.stop_bits.clone() {
                match s {
                    StopBits::Stop1  => uart.cr2.modify(|_, w| unsafe {
                        w.stop().bits(0b00)
                    }),
                    StopBits::Stop2  => uart.cr2.modify(|_, w| unsafe {
                        w.stop().bits(0b10)
                    }),
                    StopBits::Stop05 => uart.cr2.modify(|_, w| unsafe {
                        w.stop().bits(0b01)
                    }),
                    StopBits::Stop15 => uart.cr2.modify(|_, w| unsafe {
                        w.stop().bits(0b11)
                    }),
                }
            } else {
                uart.cr2.modify(|_, w| unsafe {
                    w.stop().bits(0b00)
                });
            }

            uart.cr1.modify(|_, w| {
                w.te().bit(true)
                    .ue().bit(true)
            });
        });

        Serial
    }

    fn br_conv(periph_freq : u32, br : u32) -> u32 {
        periph_freq / (16 * br)
    }
}


impl fmt::Write for Serial {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        unsafe{
            for c in s.chars() {
                while !(*USART2.get()).sr.read().txe().bit() {}
                (*USART2.get()).dr.write(|w| unsafe {
                    w.bits((c as u32) & 0x000000FF)
                });
            }
        }
        Ok(())
    }
}
