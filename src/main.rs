use rustboy::gameboy::GameBoy;

use anyhow::Error;
use anyhow::Result;
use std::env;

fn main() -> Result<(), Error> {
    // Logger
    pretty_env_logger::init();

    // Command Line Arguments
    let args: Vec<String> = env::args().collect();
    let game = args[1].as_str();
    let mut gameboy = GameBoy::new();
    gameboy.start_up(game)
}
