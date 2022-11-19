use crate::Cpu;
use crate::interconnect::Interconnect;

///
/// Struct that represents the gameboy system
///
/// Contains the CPU and Interconnect
pub struct GameBoy {
    pub cpu: Cpu,
    pub interconnect: Interconnect,
}

impl GameBoy {
    /// Create new instance of Gameboy
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            interconnect: Interconnect::new(),
        }
    }
}
