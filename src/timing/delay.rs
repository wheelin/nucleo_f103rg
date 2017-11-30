use cortex_m::peripheral::{SYST, SystClkSource};
use cortex_m::asm;
use clocks::*;

static mut TICKS : u32 = 0;

pub fn initialize() {
    let systick_freq = ClockConfig::get_speeds().sys_clk;
    unsafe {
        (*SYST.get()).set_clock_source(SystClkSource::Core);
        (*SYST.get()).set_reload(systick_freq / 1000);
    }
}

pub fn ms(time : u32) {
    unsafe {
        (*SYST.get()).enable_interrupt();
        (*SYST.get()).enable_counter();
        let start = TICKS;
        while (TICKS - start) < time {
            asm::wfi();
        }
        (*SYST.get()).disable_counter();
        (*SYST.get()).disable_interrupt();
    }
}

pub fn ticks() {
    unsafe {
        TICKS += 1;
    }
}