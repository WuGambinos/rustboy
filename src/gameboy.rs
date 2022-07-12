use crate::Cpu;
use crate::interconnect::Interconnect;

pub struct GameBoy {
    pub cpu: Cpu,
    pub interconnect: Interconnect,
}

impl GameBoy {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            interconnect: Interconnect::new(),
        }
    }
}
