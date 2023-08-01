mod gui;
mod constants;

use constants::*;

use anyhow::Result;
use clap::*;
use env_logger::*;
use rustboy::gameboy::*;
use sdl2::event::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to rom file
    #[arg(short, long)]
    rom: String,

    /// Determines whether GUI will be run or not
    #[arg(long, default_value = "false")]
    headless: bool,

    #[arg(long, default_value = "false")]
    skip_boot: bool,
}

fn main() -> Result<(), anyhow::Error> {
    // Command Line Arguments
    let args = Args::parse();
    println!("ROM: {}", args.rom);
    println!("HEADLESS: {}", args.headless);

    // Logger
    let mut logger = Builder::from_default_env();
    logger.target(Target::Stdout);
    logger.init();

    let mut gameboy = GameBoy::new();
    gameboy.boot(args.rom.as_str(), args.skip_boot)?;
    run_sdl(&mut gameboy, args.headless)?;
    Ok(())
}

#[cfg(target_os = "linux")]
pub fn run_sdl(gb: &mut GameBoy, headless: bool) -> Result<(), Error> {
    use sdl2::keyboard::Keycode;

    if headless {
        loop {
            gb.cpu.run(&mut gb.interconnect);
        }
    } else {
        let sdl_context = sdl2::init().expect("Failed to start SDL");
        let mut debug_window = gui::init_window(&sdl_context, SCREEN_WIDTH, SCREEN_HEIGHT);
        let mut event_pump = sdl_context.event_pump().expect("Failed to get event pump");

        let mut main_window = gui::init_window(&sdl_context, MAIN_SCREEN_WIDTH, MAIN_SCREEN_HEIGHT);

        'running: loop {
            gb.cpu.run(&mut gb.interconnect);

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }
            gui::debug_window(&mut debug_window, &gb.interconnect);
            gui::main_window(&mut main_window, &gb.interconnect);
        }
    }
    Ok(())
}
