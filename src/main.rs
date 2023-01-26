pub mod constants;
pub mod cpu;
pub mod gameboy;
pub mod interconnect;
pub mod lcd;
pub mod mmu;
pub mod ppu;
pub mod window;

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
