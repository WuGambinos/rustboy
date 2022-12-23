mod constants;
mod cpu;
mod gameboy;
mod interconnect;
mod lcd;
mod mmu;
mod ppu;
mod window;

pub use cpu::Cpu;
pub use mmu::Mmu;

use cpu::timer::Timer;
use gameboy::GameBoy;

use std::env;

use anyhow::Error;
use anyhow::Result;

fn main() -> Result<(), Error> {
    // Command Line Arguments
    let args: Vec<String> = env::args().collect();
    let game = args[1].as_str();
    let mut gameboy = GameBoy::new();
    gameboy.start_up(game)
}


