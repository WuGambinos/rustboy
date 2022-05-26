use crate::{Cpu, Timer};

use crate::Mmu;

struct Gameboy {
    cpu: Cpu,
    mmu: Mmu,
    timer: Timer,
}
