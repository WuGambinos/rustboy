#[derive(Debug)]
pub struct Clock {
    pub period: u32,
    pub n: u32,
}

impl Clock {
    pub fn power_up(period: u32) -> Self {
        Self { period, n: 0x00 }
    }

    pub fn next(&mut self, cycles: u32) -> u32 {
        self.n += cycles;
        let res = self.n / self.period;
        self.n = self.n % self.period;
        res
    }
}

#[derive(Debug)]
pub struct Timer {
    /// Divider Register - Incremented at rate of 16384Hz, Writing any vlaue to this register
    /// resets it to 0x00
    pub(crate) div: u8,

    /// Timer Counter(R/W) - Incremented by clock frequency specified by the TAC register
    /// When the value overflows then it will be reset to value specified in TMA and interrupt
    /// will be request
    pub(crate) tima: u8,

    /// Timer Modulo (R/W)
    pub(crate) tma: u8,

    ///Timer Control (R/W)
    pub(crate) tac: u8,

    ///Internal Ticks
    pub(crate) internal_ticks: u64,

    pub div_clock: Clock,

    pub tma_clock: Clock,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
            internal_ticks: 0,
            div_clock: Clock::power_up(256),
            tma_clock: Clock::power_up(1024),
        }
    }
    pub fn print_timer(&self) {
        println!(
            "Ticks: {:#X} DIV: {} TIMA: {} TMA: {} TAC: {}",
            self.internal_ticks, self.div, self.tima, self.tma, self.tac
        );
    }

    /// Read u8 value from Timer/Divider register at addr
    pub fn timer_read(&self, addr: u16) -> u8 {
        match addr {
            0xFF04 => self.div,

            0xFF05 => self.tima,

            0xFF06 => self.tma,

            0xFF07 => self.tac,

            _ => 123,
        }
    }
    /// Write u8 value to Timer/Divider register at addr
    pub fn timer_write(&mut self, addr: u16, value: u8) {
        match addr {
            //DIV
            0xFF04 => {
                self.div = 0x00;
                self.div_clock.n = 0x00;
            }

            //TIMA
            0xFF05 => self.tima = value,

            //TMA
            0xFF06 => self.tma = value,

            //TAC
            0xFF07 => {
                if (self.tac & 0x03) != (value & 0x03) {
                    self.tma_clock.n = 0x00;
                    self.tma_clock.period = match value & 0x03 {
                        0x00 => 1024,
                        0x01 => 16,
                        0x02 => 64,
                        0x03 => 256,
                        _ => panic!(""),
                    };
                    self.tima = self.tma;
                }
                self.tac = value;
            }

            _ => (),
        }
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}
