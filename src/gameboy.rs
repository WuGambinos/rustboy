use crate::{Cpu, Timer};

use crate::Mmu;

pub struct GameBoy {
    cpu: Cpu,
    mmu: Mmu,
    timer: Timer,
}

impl GameBoy {
    pub fn new(cpu: &mut Cpu, mmu: &mut Mmu, timer: &mut Timer) -> Self {
        Self {
            cpu: Cpu::new(),
            mmu: Mmu::new(),
            timer: Timer::new(),
        }
    }
}
