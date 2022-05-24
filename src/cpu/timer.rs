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
    fn new() -> Self {
        Self {
            div: 0xAC00,
            tima: 0,
            tma: 0,
            tac: 0,
        }
    }

    fn timer_tick(&self) {


    }

    /// Read u8 valeu from Timer/Divider register at addr
    fn timer_read(&self, mmu: &Mmu) -> u8 {
        match addr {
            0xFF04 => return ((self.div as u16) >> 8) as u8,

            0xFF05 => return self.tima,

            0xFF06 => return self.tma,

            0xFF07 => return self.tac,

            _ => -1,
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

            _ => "Not  a Timer/Divider Regsiter"
        }

    }

}

