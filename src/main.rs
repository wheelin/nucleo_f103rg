#![feature(used)]
#![no_std]

extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;

extern crate stm32f103xx;

mod serial;
mod clocks;
mod gpio;
mod timing;
mod pwm;
mod i2c;
mod spi;
mod analog;

use clocks::*;
use gpio::*;
use timing::delay;

use core::fmt::Write;

use cortex_m::asm;
use cortex_m_semihosting::hio;

exception!(SYS_TICK, delay::ticks);

fn main() {

    let clock_freqs = ClockConfig::new()
        .sys_clk_src(SysClockSrc::PllClock)
        .pll_src(PllSrc::Hse)
        .pll_div(HsePllPre::HseDiv1)
        .pll_mul(PllMul::Mul6)
        .ahb_pre(AhbPre::Pre1)
        .apb1_pre(ApbPre::Pre2)
        .apb2_pre(ApbPre::Pre1)
        .configure();

    let mut ser = serial::SerialConfig::new()
        .baud_rate(serial::BaudRate::Br115200)
        .stop_bits(serial::StopBits::Stop1)
        .data_length(serial::DataLength::DataLen8bits)
        .configure();

    if clock_freqs.is_err() {
        writeln!(ser, "Error with clock configuration : {:?}", clock_freqs.err());
    }

    let led = match GpioConfig::new()
        .pin(Pin(5))
        .port(Port::A)
        .conf(Conf::PushPullOut)
        .mode(Mode::Output2MHz)
        .configure() {

        Ok(g) => g,
        Err(e) => {
            writeln!(ser, "Problem while configuring gpio : {:?}", e);
            return;
        }
    };

    delay::initialize();

    loop {
        led.set(State::High).unwrap();
        delay::ms(500);
        led.set(State::Low).unwrap();
        delay::ms(500);
    }
}

