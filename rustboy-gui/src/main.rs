mod constants;
mod gui;
mod support;

use constants::*;

use anyhow::Result;
use clap::*;
use env_logger::*;
use imgui::{Condition, DrawListMut, ImColor32, Ui};
use rustboy::constants::TILE_COLORS;
use rustboy::interconnect::joypad::Key;
use rustboy::interconnect::Interconnect;
use rustboy::{
    constants::{X_RESOLUTION, Y_RESOLUTION},
    gameboy::*,
};

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

fn imgui_key_to_gb_key(keycode: imgui::Key) -> Option<Key> {
    match keycode {
        imgui::Key::RightArrow | imgui::Key::D => Some(Key::Right),
        imgui::Key::LeftArrow | imgui::Key::A => Some(Key::Left),
        imgui::Key::UpArrow | imgui::Key::W => Some(Key::Up),
        imgui::Key::DownArrow | imgui::Key::S => Some(Key::Down),
        imgui::Key::Z => Some(Key::A),
        imgui::Key::X => Some(Key::B),
        imgui::Key::Space => Some(Key::Select),
        imgui::Key::Enter => Some(Key::Start),
        _ => None,
    }
}

const keys: [imgui::Key; 8] = [
    imgui::Key::RightArrow,
    imgui::Key::LeftArrow,
    imgui::Key::UpArrow,
    imgui::Key::DownArrow,
    imgui::Key::Z,
    imgui::Key::X,
    imgui::Key::Space,
    imgui::Key::Enter,
];

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

    let system = support::init(file!());
    system.main_loop(move |_, ui| {
        for key in keys {
            if ui.is_key_down(key) {
                if let Some(gb_key) = imgui_key_to_gb_key(key) {
                    gameboy.interconnect.key_down(gb_key);
                }
            } else if ui.is_key_released(key) {
                if let Some(gb_key) = imgui_key_to_gb_key(key) {
                    gameboy.interconnect.key_up(gb_key);
                }
            }
        }

        gameboy.cpu.run(&mut gameboy.interconnect);
        //gui::memory_viewer(ui, &gameboy);
        gui::display_info(ui, &gameboy);
        gui::draw_tiles(ui, &gameboy.interconnect);
        gui::display_emulator(ui, &gameboy);
        gui::debug_window(ui, &gameboy);
    });

    Ok(())
}
