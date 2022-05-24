use crate::Mmu;


pub struct Timer {
    /// Divider Register
    div: u16,
    /// Timer Counter(R/W)
    tima: u8,
    /// Timer Modulo (R/W)
    tma: u8,
    ///Timer Control (R/W)
    tac: u8,
}


impl Timer {
    pub fn new() -> Self {
        Self {
            div: 0xAC00,
            tima: 0,
            tma: 0,
            tac: 0,
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

    /// Read u8 valeu from Timer/Divider register at addr
    fn timer_read(&self, mmu: &Mmu, addr: u16) -> u8 {
        match addr {
            0xFF04 => return ((self.div as u16) >> 8) as u8,

            0xFF05 => return self.tima,

            0xFF06 => return self.tma,

            0xFF07 => return self.tac,

            _ => 123,
        }

    }

    /// Write u8 value to Timer/Divider register at addr
    fn timer_write(&mut self, mmu: &Mmu, addr: u16, value: u8){
        match addr{

            //DIV
            0xFF04 => self.div = 0,

            //TIMA
            0xFF05 => self.tima = value,

            //TMA
            0xFF06 => self.tma = value,

            //TAC
            0xFF07 => self.tac = value,

            _ => ()
        }

    }

}

