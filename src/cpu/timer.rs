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
    pub(crate) internal_ticks: u32,

    tima_speed: u32,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            div: 0xAC00,
            tima: 0,
            tma: 0,
            tac: 0,
            internal_ticks: 0,
            tima_speed: 256,
        }
    }

    pub fn update(&mut self) {
        match self.tac & 0x3 {
            0x0 => self.tima_speed = 256,
            0x1 => self.tima_speed = 4,
            0x2 => self.tima_speed = 16,
            0x3 => self.tima_speed = 64,

            _ => (),
        }
    }

    pub(crate) fn timer_tick(&mut self) {
        let prev_div: u16 = self.div;

        //Increment Div Register
        self.div = self.div.wrapping_add(1);

        let mut timer_update: bool = false;

        //Get Timer Clock
        match self.tac & 0b11 {
            0b00 => timer_update = ((prev_div & (1 << 9)) == 1) && ((!(self.div & (1 << 9))) == 1),

            0b01 => timer_update = ((prev_div & (1 << 3)) == 1) && ((!(self.div & (1 << 3))) == 1),

            0b10 => timer_update = ((prev_div & (1 << 5)) == 1) && ((!(self.div & (1 << 5))) == 1),

            0b11 => timer_update = ((prev_div & (1 << 7)) == 1) && ((!(self.div & (1 << 7))) == 1),

            _ => (),
        }

        let cond: u8 = self.tac & (1 << 2);

        //If Clock is enabled
        if timer_update && cond == 1 {
            //Increment Counter by 1
            self.tima = self.tima.wrapping_add(1);

            //If Counter Overflows, request interrupt
            if self.tima == 0 {
                self.tima = self.tma;
                //Request Timer interrupt TODO
            }
        }
    }

    /// Read u8 valeu from Timer/Divider register at addr
    fn timer_read(&self, addr: u16) -> u8 {
        match addr {
            0xFF04 => return ((self.div as u16) >> 8) as u8,

            0xFF05 => return self.tima,

            0xFF06 => return self.tma,

            0xFF07 => return self.tac,

            _ => 123,
        }
    }

    /// Write u8 value to Timer/Divider register at addr
    fn timer_write(&mut self, addr: u16, value: u8) {
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
