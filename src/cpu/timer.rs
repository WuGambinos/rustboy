use crate::Mmu;

#[derive(Debug)]
pub struct Timer {
    /// Divider Register - Incremented at rate of 16384Hz, Writing any vlaue to this register
    /// resets it to 0x00
    pub(crate) div: u16,

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

    pub div_counter: u32,

    pub tima_counter: u32,

    pub dividers: Vec<u32>,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            div: 0xABCC,
            tima: 0,
            tma: 0,
            tac: 0,
            internal_ticks: 0,
            div_counter: 0,
            tima_counter: 0,
            dividers: vec![1024, 16, 64, 256],
        }
    }

    /// Read u8 value from Timer/Divider register at addr
    pub fn timer_read(&self, addr: u16) -> u8 {
        match addr {
            0xFF04 => ((self.div as u16) >> 8) as u8,

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
            0xFF04 => self.div = 0,

            //TIMA
            0xFF05 => self.tima = value,

            //TMA
            0xFF06 => self.tma = value,

            //TAC
            0xFF07 => self.tac = value,

            _ => (),
        }
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}
