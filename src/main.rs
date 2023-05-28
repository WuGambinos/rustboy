use rustboy::gameboy::GameBoy;

use anyhow::Error;
use anyhow::Result;
use env_logger::*;
use std::env;

fn main() -> Result<(), Error> {
    // Logger
    let mut logger = Builder::from_default_env();
    logger.target(Target::Stdout);
    logger.init();

    // Command Line Arguments
    let args: Vec<String> = env::args().collect();
    let game = args[1].as_str();
    let headless = args.get(2);
    let mut gameboy = GameBoy::new();

    if let Some(_) = headless {
        gameboy.start_up(game, true)
    } else {
        gameboy.start_up(game, false)
    }
}
