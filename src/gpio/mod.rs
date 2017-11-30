use stm32f103xx::{RCC, GPIOA, GPIOB, GPIOC};

#[derive(Debug)]
pub enum GpioError {
    ReservedConfig,
    WriteOnInput,
}

type GpioResult<T> = Result<T, GpioError>;

static mut interrupt_callback : Option<fn()> = None;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Conf {
    AnalogIn,
    FloatingIn,
    PullUpDownIn,

    PushPullOut,
    OpenDrainOut,
    AltFnPushPullOut,
    AltFnOpenDrainOut,
}

impl Conf {
    pub fn val(&self) -> u32 {
        match *self {
            Conf::AnalogIn => 0b00,
            Conf::FloatingIn => 0b01,
            Conf::PullUpDownIn => 0b10,
            Conf::PushPullOut => 0b00,
            Conf::OpenDrainOut => 0b01,
            Conf::AltFnPushPullOut => 0b10,
            Conf::AltFnOpenDrainOut => 0b11,
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Mode {
    Input,
    Output10MHz,
    Output2MHz,
    Output50MHz,
}

impl Mode {
    pub fn val(&self) -> u32 {
        match *self {
            Mode::Input => 0b00,
            Mode::Output10MHz => 0b01,
            Mode::Output2MHz => 0b10,
            Mode::Output50MHz => 0b11,
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Port {
    A,
    B,
    C,
}

#[derive(Copy, Clone)]
pub struct Pin(pub u32);

impl Pin {
    pub fn number(&self) -> u32 {
        self.0
    }

    pub fn code(&self) -> u16 {
        (1 << self.0) as u16
    }
}

pub struct GpioConfig {
    conf : Conf,
    mode : Mode,
    pin  : Pin,
    port : Port,
}

impl GpioConfig {
    pub fn new() -> GpioConfig {
        GpioConfig {
            conf : Conf::AnalogIn,
            mode : Mode::Input,
            pin  : Pin(0),
            port : Port::A,
        }
    }

    pub fn pin(mut self, pin : Pin) -> GpioConfig {
        self.pin = pin;
        self
    }

    pub fn port(mut self, port : Port) -> GpioConfig {
        self.port = port;
        self
    }

    pub fn conf(mut self, conf : Conf) -> GpioConfig {
        self.conf = conf;
        self
    }

    pub fn mode(mut self, mode : Mode) -> GpioConfig {
        self.mode = mode;
        self
    }

    pub fn configure(&self) -> GpioResult<Gpio> {
        unsafe {
            // enable clock for the current gpio
            match self.port {
                Port::A => {
                    (*RCC.get()).apb2enr.modify(|_, w| w.iopaen().bit(true));

                    if self.conf == Conf::AltFnOpenDrainOut &&
                        self.mode == Mode::Input {
                        return Err(GpioError::ReservedConfig);
                    }

                    if self.pin.number() > 7 {
                        (*GPIOA.get()).crh.modify(|_, w| {
                           w.bits(self.mode.val() << ((self.pin.number() - 8) * 4) |
                               self.conf.val() << (((self.pin.number() - 8) * 4) + 2))
                        });
                    } else {
                        (*GPIOA.get()).crl.modify(|_, w| {
                            w.bits(self.mode.val() << (self.pin.number() * 4) |
                                self.conf.val() << ((self.pin.number() * 4) + 2))
                        });
                    }
                },
                Port::B => {
                    (*RCC.get()).apb2enr.modify(|_, w| w.iopben().bit(true));

                    if self.conf == Conf::AltFnOpenDrainOut &&
                        self.mode == Mode::Input {
                        return Err(GpioError::ReservedConfig);
                    }

                    if self.pin.number() > 7 {
                        (*GPIOB.get()).crh.modify(|_, w| {
                            w.bits(self.mode.val() << ((self.pin.number() - 8) * 4) |
                                self.conf.val() << (((self.pin.number() - 8) * 4) + 2))
                        });
                    } else {
                        (*GPIOB.get()).crl.modify(|_, w| {
                            w.bits(self.mode.val() << (self.pin.number() * 4) |
                                self.conf.val() << ((self.pin.number() * 4) + 2))
                        });
                    }
                },
                Port::C => {
                    (*RCC.get()).apb2enr.modify(|_, w| w.iopcen().bit(true));

                    if self.conf == Conf::AltFnOpenDrainOut &&
                        self.mode == Mode::Input {
                        return Err(GpioError::ReservedConfig);
                    }

                    if self.pin.number() > 7 {
                        (*GPIOC.get()).crh.modify(|_, w| {
                            w.bits((self.mode.val() as u32) << ((self.pin.number() - 8) * 4) |
                                (self.conf.val() as u32) << (((self.pin.number() - 8) * 4) + 2))
                        });
                    } else {
                        (*GPIOC.get()).crl.modify(|_, w| {
                            w.bits((self.mode.val() as u32) << ((self.pin.number()) * 4) |
                                (self.conf.val() as u32) << (((self.pin.number()) * 4) + 2))
                        });
                    }
                },
            };
        }

        Ok(Gpio {
            conf : self.conf,
            mode : self.mode,
            pin  : self.pin,
            port : self.port,
        })
    }
}

pub struct Gpio {
    conf : Conf,
    mode : Mode,
    pin  : Pin,
    port : Port,
}

#[derive(Eq, PartialEq)]
pub enum State {
    High,
    Low,
}

impl Gpio {
    pub fn set(&self, ns : State) -> GpioResult<()> {
        if self.mode == Mode::Input {
            return Err(GpioError::WriteOnInput);
        }

        let wr : u32 = match ns {
            State::High => 1 << self.pin.number(),
            State::Low  => 1 << (self.pin.number() + 16),
        };
        unsafe {
            match self.port {
                Port::A => (*GPIOA.get()).bsrr.write(|w| w.bits(wr)),
                Port::B => (*GPIOB.get()).bsrr.write(|w| w.bits(wr)),
                Port::C => (*GPIOC.get()).bsrr.write(|w| w.bits(wr)),
            }
        }

        Ok(())
    }

    pub fn get(&self) -> State {
        unsafe {
            if self.mode == Mode::Input {
                match self.port {
                    Port::A => {
                        if (*GPIOA.get()).idr.read().bits() & (self.pin.code() as u32) != 0 {
                            return State::High;
                        } else {
                            return State::Low;
                        }
                    },
                    Port::B =>  {
                        if (*GPIOB.get()).idr.read().bits() & (self.pin.code() as u32) != 0 {
                            return State::High;
                        } else {
                            return State::Low;
                        }
                    },
                    Port::C =>  {
                        if (*GPIOC.get()).idr.read().bits() & (self.pin.code() as u32) != 0 {
                            return State::High;
                        } else {
                            return State::Low;
                        }
                    },
                }
            } else {
                match self.port {
                    Port::A => {
                        if (*GPIOA.get()).odr.read().bits() & (self.pin.code() as u32) != 0 {
                            return State::High;
                        } else {
                            return State::Low;
                        }
                    },
                    Port::B =>  {
                        if (*GPIOB.get()).odr.read().bits() & (self.pin.code() as u32) != 0 {
                            return State::High;
                        } else {
                            return State::Low;
                        }
                    },
                    Port::C =>  {
                        if (*GPIOC.get()).odr.read().bits() & (self.pin.code() as u32) != 0 {
                            return State::High;
                        } else {
                            return State::Low;
                        }
                    },
                }
            }
        }

    }


    pub fn enable_interrupt(&self, callback : fn()) -> GpioResult<()> {
        unsafe {
            self::interrupt_callback = Some(callback);
        }
        Ok(())
    }

    pub fn disable_interrupt(&self) -> GpioResult<()> {
        Ok(())
    }
}