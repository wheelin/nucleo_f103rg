use stm32f103xx::RCC;

#[derive(Debug)]
pub enum ClockError {
    SysClkOverClocking,
    AhbOverClocking,
    Apb1OverClocking,
    Apb2OverClocking,

    PllSettingFault,
    HsiSettingFault,
    HseSettingFault,
    SysClkSettingFault,
}

type ClockResult<U> = Result<U, ClockError>;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum SysClockSrc {
    HighSpeedInternal,
    PllClock,
    HighSpeedExternal,
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum PllMul {
    Mul2  = 0b0000,
    Mul3  = 0b0001,
    Mul4  = 0b0010,
    Mul5  = 0b0011,
    Mul6  = 0b0100,
    Mul7  = 0b0101,
    Mul8  = 0b0110,
    Mul9  = 0b0111,
    Mul10 = 0b1000,
    Mul11 = 0b1001,
    Mul12 = 0b1010,
    Mul13 = 0b1011,
    Mul14 = 0b1100,
    Mul15 = 0b1101,
    Mul16 = 0b1110,
    Mul16Bis = 0b1111,
}

impl PllMul {
    pub fn as_val(&self) -> u32 {
        (*self as u32) + 2
    }

    pub fn as_code(&self) -> u8 {
        (*self as u8)
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum HsePllPre {
    HseDiv1,
    HseDiv2,
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum PllSrc {
    Hsi,
    Hse,
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum ApbPre {
    Pre1  = 0b000,
    Pre2  = 0b100,
    Pre4  = 0b101,
    Pre8  = 0b110,
    Pre16 = 0b111,
}

impl ApbPre {
    pub fn as_val(&self) -> u32 {
        match *self {
            ApbPre::Pre1 => 1,
            ApbPre::Pre2 => 2,
            ApbPre::Pre4 => 4,
            ApbPre::Pre8 => 8,
            ApbPre::Pre16 => 16,
        }
    }

    pub fn as_code(&self) -> u8 {
        (*self as u8)
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum AhbPre {
    Pre1    = 0b0000,
    Pre2    = 0b1000,
    Pre4    = 0b1001,
    Pre8    = 0b1010,
    Pre16   = 0b1011,
    Pre64   = 0b1100,
    Pre128  = 0b1101,
    Pre256  = 0b1110,
    Pre512  = 0b1111,
}

impl AhbPre {
    pub fn as_val(&self) -> u32 {
        match *self {
            AhbPre::Pre1 => 1,
            AhbPre::Pre2 => 2,
            AhbPre::Pre4 => 4,
            AhbPre::Pre8 => 8,
            AhbPre::Pre16 => 16,
            AhbPre::Pre64 => 64,
            AhbPre::Pre128 => 128,
            AhbPre::Pre256 => 256,
            AhbPre::Pre512 => 512,
        }
    }

    pub fn as_code(&self) -> u8 {
        (*self as u8)
    }
}

pub struct ClockSpeeds {
    pub sys_clk : u32,
    pub ahb_clk : u32,
    pub apb1_clk : u32,
    pub apb2_clk : u32,
}

pub struct ClockConfig {
    sysclk_src : SysClockSrc,

    pll_src : PllSrc,
    pll_mul : PllMul,
    pll_div : HsePllPre,

    ahb_pre : AhbPre,
    apb1_pre: ApbPre,
    apb2_pre: ApbPre,
}

pub const EXT_CLK_FREQ : u32 = 8_000_000;
pub const INT_CLK_FREQ : u32 = 8_000_000;
pub const CLK_SETTING_TIMEOUT : u32 = 100;
pub const MAX_SYS_FREQ : u32 = 72_000_000;
pub const MAX_AHB_FREQ : u32 = MAX_SYS_FREQ;
pub const MAX_APB2_FREQ : u32 = MAX_SYS_FREQ;
pub const MAX_APB1_FREQ : u32 = 36_000_000;

impl ClockConfig {
    pub fn new() -> ClockConfig {
        ClockConfig {
            sysclk_src : SysClockSrc::HighSpeedExternal,
            pll_src : PllSrc::Hse,
            pll_mul : PllMul::Mul2,
            pll_div : HsePllPre::HseDiv2,
            ahb_pre : AhbPre::Pre1,
            apb1_pre: ApbPre::Pre2,
            apb2_pre: ApbPre::Pre1,
        }
    }

    pub fn sys_clk_src(mut self, scr : SysClockSrc) -> ClockConfig {
        self.sysclk_src = scr;
        self
    }

    pub fn pll_src(mut self, ps : PllSrc) -> ClockConfig {
        self.pll_src = ps;
        self
    }

    pub fn pll_mul(mut self, pm : PllMul) -> ClockConfig {
        self.pll_mul = pm;
        self
    }

    pub fn pll_div(mut self, pd : HsePllPre) -> ClockConfig {
        self.pll_div = pd;
        self
    }

    pub fn ahb_pre(mut self, ahbp : AhbPre) -> ClockConfig {
        self.ahb_pre = ahbp;
        self
    }

    pub fn apb1_pre(mut self, app : ApbPre) -> ClockConfig {
        self.apb1_pre = app;
        self
    }

    pub fn apb2_pre(mut self, app : ApbPre) -> ClockConfig {
        self.apb2_pre = app;
        self
    }

    pub fn configure(&self) -> ClockResult<ClockConfig> {
        // the system starts with the hsi clock.
        let mut current_sys_freq = INT_CLK_FREQ;
        unsafe {
            // pll config first
            // disable pll for config
            (*RCC.get()).cr.modify(|_, w| w.pllon().bit(false));
            // then, configure pll source
            if self.pll_src == PllSrc::Hse{
                (*RCC.get()).cfgr.modify(|_, w| w.pllsrc().bit(true));
                // if hse is used, select whether hse is divided by two or not
                if self.pll_div == HsePllPre::HseDiv2 {
                    (*RCC.get()).cfgr.modify(|_, w| w.pllxtpre().bit(true));
                    current_sys_freq /= 2;
                } else {
                    (*RCC.get()).cfgr.modify(|_, w| w.pllxtpre().bit(false));
                }
            } else {
                (*RCC.get()).cfgr.modify(|_, w| w.pllsrc().bit(false));
            }
            // then configure multiplication, but check first the system will not be overclocked
            if current_sys_freq * self.pll_mul.as_val() > MAX_SYS_FREQ {
                return Err(ClockError::SysClkOverClocking);
            }
            current_sys_freq *= self.pll_mul.as_val();
            (*RCC.get()).cfgr.modify(|_, w| w.pllmul().bits(self.pll_mul.as_code()));

            // configure busses frequencies
            // check for overclocking
            if current_sys_freq / self.ahb_pre.as_val() > MAX_AHB_FREQ {
                return Err(ClockError::AhbOverClocking);
            }

            current_sys_freq /= self.ahb_pre.as_val();

            if current_sys_freq / self.apb1_pre.as_val() > MAX_APB1_FREQ {
                return Err(ClockError::Apb1OverClocking);
            }

            if current_sys_freq / self.apb2_pre.as_val() > MAX_APB2_FREQ {
                return Err(ClockError::Apb2OverClocking);
            }

            // ahb
            (*RCC.get()).cfgr.modify(|_, w| w.hpre().bits(self.ahb_pre.as_code()));
            // apb1
            (*RCC.get()).cfgr.modify(|_, w| w.ppre1().bits(self.apb1_pre.as_code()));
            // apb2
            (*RCC.get()).cfgr.modify(|_, w| w.ppre2().bits(self.apb2_pre.as_code()));

            let mut sws_data = 0b00;

            // configure sysclk
            match self.sysclk_src {
                SysClockSrc::PllClock => {
                    if self.pll_src == PllSrc::Hse {
                        // enable hse if not ready
                        (*RCC.get()).cr.modify(|_, w| w.hseon().bit(true));
                        let mut timeout : i32 = CLK_SETTING_TIMEOUT as i32;
                        while timeout > 0 && (*RCC.get()).cr.read().hserdy().bit() == false {
                            timeout -= 1;
                        }
                        if timeout <= 0 {
                            return Err(ClockError::HseSettingFault);
                        }
                    } else {
                        // enable hsi if not ready
                        (*RCC.get()).cr.modify(|_, w| w.hsion().bit(true));
                        let mut timeout : i32 = CLK_SETTING_TIMEOUT as i32;
                        while timeout > 0 && (*RCC.get()).cr.read().hsirdy().bit() == false {
                            timeout -= 1;
                        }
                        if timeout <= 0 {
                            return Err(ClockError::HsiSettingFault);
                        }
                    }

                    // enable pll
                    (*RCC.get()).cr.modify(|_, w| w.pllon().bit(true));
                    let mut timeout : i32 = CLK_SETTING_TIMEOUT as i32;
                    while timeout > 0 && (*RCC.get()).cr.read().pllrdy().bit() == false {
                        timeout -= 1;
                    }
                    if timeout <= 0 {
                        return Err(ClockError::PllSettingFault);
                    }
                    (*RCC.get()).cfgr.modify(|_, w| w.sw().pll());
                    sws_data = 0b10;
                },
                SysClockSrc::HighSpeedInternal => {
                    // enable hsi if not ready
                    (*RCC.get()).cr.modify(|_, w| w.hsion().bit(true));
                    let mut timeout : i32 = CLK_SETTING_TIMEOUT as i32;
                    while timeout > 0 && (*RCC.get()).cr.read().hsirdy().bit() == false {
                        timeout -= 1;
                    }
                    if timeout <= 0 {
                        return Err(ClockError::HsiSettingFault);
                    }
                    (*RCC.get()).cfgr.modify(|_, w| w.sw().hsi());
                    sws_data = 0b00;
                },
                SysClockSrc::HighSpeedExternal => {
                    // enable hse if not ready
                    (*RCC.get()).cr.modify(|_, w| w.hseon().bit(true));
                    let mut timeout : i32 = CLK_SETTING_TIMEOUT as i32;
                    while timeout > 0 && (*RCC.get()).cr.read().hserdy().bit() == false {
                        timeout -= 1;
                    }
                    if timeout <= 0 {
                        return Err(ClockError::HseSettingFault);
                    }
                    (*RCC.get()).cfgr.modify(|_, w| w.sw().hse());
                    sws_data = 0b01;
                }
            };

            let mut timeout : i32 = CLK_SETTING_TIMEOUT as i32;
            while timeout > 0 && (*RCC.get()).cfgr.read().sws().bits() != sws_data {
                timeout -= 1;
            }
            if timeout <= 0 {
                return Err(ClockError::SysClkSettingFault);
            }
        }

        Ok(ClockConfig{
            sysclk_src : self.sysclk_src,
            pll_mul : self.pll_mul,
            pll_div : self.pll_div,
            pll_src : self.pll_src,
            ahb_pre : self.ahb_pre,
            apb1_pre : self.apb1_pre,
            apb2_pre : self.apb2_pre,
        })
    }

    fn get_speeds_ancient(&self) -> ClockSpeeds {
        let sc = match self.sysclk_src {
                SysClockSrc::HighSpeedExternal => EXT_CLK_FREQ,
                SysClockSrc::HighSpeedInternal => INT_CLK_FREQ,
                SysClockSrc::PllClock => {
                    let mut input = EXT_CLK_FREQ;
                    match self.pll_src {
                        PllSrc::Hse => {
                            match self.pll_div {
                                HsePllPre::HseDiv1 => input = input / 2,
                                HsePllPre::HseDiv2 => input = input / 1,
                            }
                        },
                        PllSrc::Hsi => input = INT_CLK_FREQ / 2,
                    };
                    input * (self.pll_mul as u32)
                },
        };
        let ahb = sc / (self.ahb_pre as u32);
        let apb1 = ahb / (self.apb1_pre as u32);
        let apb2 = ahb / (self.apb2_pre as u32);
        ClockSpeeds {
            sys_clk : sc,
            ahb_clk : ahb,
            apb1_clk : apb1,
            apb2_clk : apb2
        }
    }

    pub fn get_speeds() -> ClockSpeeds {
        let mut sys_clk = 0;
        let mut ahb = 0;
        let mut apb1 = 0;
        let mut apb2 = 0;
        unsafe {
            if (*RCC.get()).cfgr.read().sws().is_hsi() {
                sys_clk = INT_CLK_FREQ;
            } else if (*RCC.get()).cfgr.read().sws().is_hse() {
                sys_clk = EXT_CLK_FREQ;
            } else if (*RCC.get()).cfgr.read().sws().is_pll() {
                if (*RCC.get()).cfgr.read().pllsrc().is_internal() {
                    sys_clk = INT_CLK_FREQ / 2;
                } else {
                    if (*RCC.get()).cfgr.read().pllxtpre().is_div1() {
                        sys_clk = EXT_CLK_FREQ;
                    } else {
                        sys_clk = EXT_CLK_FREQ / 2;
                    }
                }
                sys_clk *= ((*RCC.get()).cfgr.read().pllmul().bits() + 2) as u32;
            }

            if (*RCC.get()).cfgr.read().hpre().is_div1() {
                ahb = sys_clk;
            } else if (*RCC.get()).cfgr.read().hpre().is_div2() {
                ahb = sys_clk / 2;
            } else if (*RCC.get()).cfgr.read().hpre().is_div4() {
                ahb = sys_clk / 4;
            } else if (*RCC.get()).cfgr.read().hpre().is_div8() {
                ahb = sys_clk / 8;
            } else if (*RCC.get()).cfgr.read().hpre().is_div16() {
                ahb = sys_clk / 16;
            } else if (*RCC.get()).cfgr.read().hpre().is_div64() {
                ahb = sys_clk / 64;
            } else if (*RCC.get()).cfgr.read().hpre().is_div128() {
                ahb = sys_clk / 128;
            } else if (*RCC.get()).cfgr.read().hpre().is_div256() {
                ahb = sys_clk / 256;
            } else if (*RCC.get()).cfgr.read().hpre().is_div512() {
                ahb = sys_clk / 512;
            }

            if (*RCC.get()).cfgr.read().ppre1().is_div1() {
                apb1 = ahb;
            } else if (*RCC.get()).cfgr.read().ppre1().is_div2() {
                apb1 = ahb / 2;
            } else if (*RCC.get()).cfgr.read().ppre1().is_div4() {
                apb1 = ahb / 4;
            } else if (*RCC.get()).cfgr.read().ppre1().is_div8() {
                apb1 = ahb / 8;
            } else if (*RCC.get()).cfgr.read().ppre1().is_div16() {
                apb1 = ahb / 16;
            }

            if (*RCC.get()).cfgr.read().ppre2().is_div1() {
                apb2 = ahb;
            } else if (*RCC.get()).cfgr.read().ppre2().is_div2() {
                apb2 = ahb / 2;
            } else if (*RCC.get()).cfgr.read().ppre2().is_div4() {
                apb2 = ahb / 4;
            } else if (*RCC.get()).cfgr.read().ppre2().is_div8() {
                apb2 = ahb / 8;
            } else if (*RCC.get()).cfgr.read().ppre2().is_div16() {
                apb2 = ahb / 16;
            }
        }
        ClockSpeeds {
            sys_clk,
            ahb_clk : ahb,
            apb1_clk: apb1,
            apb2_clk: apb2,
        }
    }
}