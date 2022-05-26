use crate::Mmu;


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
    internal_ticks: u32,
}


impl Timer {
    pub fn new() -> Self {
        Self {
            div: 0xAC00,
            tima: 0,
            tma: 0,
            tac: 0,
            internal_ticks: 0,
        }
    }

    fn timer_tick(&mut self) {

        let prev_div:  u16 = self.div;

        self.div += 1;

        let mut timer_update: bool = false;

        match self.tac & 0b11 {
            0b00 => timer_update = ((prev_div & (1 << 9)) == 1) && ((!(self.div & (1 << 9))) == 1),

            0b01 => timer_update = ((prev_div & (1 << 3)) == 1) && ((!(self.div & (1 << 3))) == 1),

            0b10 => timer_update = ((prev_div & (1 << 5)) == 1) && ((!(self.div & (1 << 5))) == 1),

            0b11 => timer_update = ((prev_div & (1 << 7)) == 1) && ((!(self.div & (1 << 7))) == 1) ,

            _ => (),
        }

        let cond: u8 = self.tac & (1 << 2);

        if timer_update && cond == 1 {
            self.tima += 1;


            //If Overflow request interrupt
            if self.tima == 0xFF {
                self.tima = self.tima

                //Request Timer interrupt


            }
        }


    }



}

