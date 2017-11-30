use stm32f103xx::{SPI1, SPI2, RCC, GPIOA, GPIOB}

pub enum SpiError {
    ConfigError,
}

type SpiResult<T> = Result<T, SpiError>;

#[derive(Copy, Clone)]
pub enum Periph {
    Spi1,
    Spi2,
}

#[derive(Copy, Clone)]
pub enum PinSet {
    First,
    Second,
}

#[derive(Copy, Clone)]
pub enum DataFrameFormat {
    Bits8,
    Bits16,
}

#[derive(Copy, Clone)]
pub enum DirFrameFormat {
    MsbFirst,
    LsbFirst,
}

#[derive(Copy, Clone)]
pub enum FreqPrescaler {
    Div2,
    Div4,
    Div8,
    Div16,
    Div32,
    Div64,
    Div128,
    Div256,
}

#[derive(Copy, Clone)]
pub enum ClockPolarity {
    Low,
    High,
}

#[derive(Copy, Clone)]
pub enum ClockPhase {
    FirstEdge,
    SecondEdge,
}

#[derive(Copy, Clone)]
pub enum DataMode {
    FullDuplex,
    FullDuplexRecv,
    HalfDuplexRecv,
    HalfDuplexSend,
}

pub struct SpiConfig {
    periph : SpiPeriph,
    pin_set : u8,
    data_frame_format : DataFrameFormat,
    data_dir : DirFrameFormat,
    prescaler : FreqPrescaler,
    master : bool,
    clk_pol : ClockPolarity,
    clk_pha : ClockPhase,
    data_mode : DataMode,
}

pub struct Spi {
    periph : SpiPeriph,
    pin_set : u8,
    data_frame_format : DataFrameFormat,
    data_dir : DirFrameFormat,
    prescaler : FreqPrescaler,
    master : bool,
    clk_pol : ClockPolarity,
    clk_pha : ClockPhase,
    data_mode : DataMode,
}

impl SpiConfig {
    pub fn new() -> SpiConfig {
        SpiConfig {
            periph : SpiPeriph::Spi1,
            pin_set : 1,
            data_frame_format : DataFrameFormat::Bits8,
            data_dir : DirFrameFormat::MsbFirst,
            prescaler : FreqPrescaler::Div4,
            master : true,
            clk_pol : ClockPolarity::Low,
            clk_pha : ClockPhase::FirstEdge,
            data_mode : DataMode::FullDuplex,
        }
    }

    pub fn instance(self, p : SpiPeriph) -> self {
        self.periph = p;
        self
    }

    pub fn pin_set(self, ps : u8) -> self {
        self.pin_set = ps;
        self
    }

    pub fn data_frame_format(self, dff : DataFrameFormat) -> self {
        self.data_frame_format = dff;
        self
    }

    pub fn dir_frame_format(self, dff : DirFrameFormat) -> self {
        self.dir_frame_format = dff;
        self
    }

    pub fn prescaler(self, psc : FreqPrescaler) -> self {
        self.prescaler = psc;
        self
    }

    pub fn clock_polarity(self, cp : ClockPolarity) -> self {
        self.clk_pol = cp;
        self
    }

    pub fn clock_phase(self, cp : ClockPhase) -> self {
        self.clk_pha = cp;
        self
    }

    pub fn master(self, en : bool) -> self {
        self.master = en;
        self
    }

    pub fn data_mode(self, dm : DataMode) -> self {
        self.data_mode = dm;
        self
    }

    pub fn configure(&mut self) -> SpiResult<Spi> {

        Ok(
            Spi {
                periph : self.periph,
                pin_set : self.pin_set,
                data_frame_format : self.data_frame_format,
                data_dir : DirFrameFormat,
                prescaler : FreqPrescaler,
                master : bool,
                clk_pol : ClockPolarity,
                clk_pha : ClockPhase,
                data_mode : DataMode,
            }
        )
    }
}

