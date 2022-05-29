use crate::{Mmu, Timer};
pub struct Interconnect {
    mmu: Mmu,
    timer: Timer,
}

impl Interconnect {
    pub fn new() -> Self {
        Self {
            mmu: Mmu::new(),
            timer: Timer::new(),
        }
    }
}
