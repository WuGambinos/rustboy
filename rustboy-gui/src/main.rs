mod constants;
mod gui;
mod support;

use constants::*;

use anyhow::Result;
use clap::*;
use env_logger::*;
use imgui::{Condition, DrawListMut, ImColor32, Ui};
use rustboy::constants::TILE_COLORS;
use rustboy::interconnect::Interconnect;
use rustboy::{
    constants::{X_RESOLUTION, Y_RESOLUTION},
    gameboy::*,
    interconnect::joypad::Key,
};
use sdl2::{event::*, keyboard::Keycode};

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
    /*
    run_sdl(&mut gameboy, args.headless)?;
    */

    let system = support::init(file!());
    system.main_loop(move |_, ui| {
        gameboy.cpu.run(&mut gameboy.interconnect);

        /*
        ui.window("Debug Window")
            .position([200.0, 500.0], Condition::FirstUseEver)
            .size([150.0, 400.0], Condition::FirstUseEver)
            .build(|| {
                let pc = format!("PC: {:#X}", gameboy.cpu.pc);
                let sp = format!("SP: {:#X}", gameboy.cpu.sp);
                ui.text(pc);
                ui.text(sp);
            });
            */

        draw_tiles(ui, &gameboy.interconnect);
        display_emulator(ui, &gameboy);
    });

    Ok(())
}

fn display_emulator(ui: &mut Ui, gameboy: &GameBoy) {
    ui.window("Gameboy Emualtor")
        .size(
            [
                (MAIN_SCREEN_WIDTH + 50) as f32,
                (MAIN_SCREEN_HEIGHT + 50) as f32,
            ],
            Condition::FirstUseEver,
        )
        .position([0.0, 0.0], Condition::FirstUseEver)
        .scroll_bar(false)
        .build(|| {
            let draw_list = ui.get_window_draw_list();
            let origin: [f32; 2] = ui.cursor_screen_pos();
            let video_buffer = gameboy.interconnect.ppu.video_buffer;

            for line_num in 0..Y_RESOLUTION {
                for x in 0..X_RESOLUTION {
                    let new_x = (x as u16 * SCALE as u16) as f32;
                    let new_y = (line_num as u16 * SCALE as u16) as f32;

                    let width = SCALE as f32;
                    let height = SCALE as f32;

                    let index =
                        (u32::from(x) + (u32::from(line_num) * u32::from(X_RESOLUTION))) as usize;
                    let color = video_buffer[index];
                    let (r, g, b) = color.get_rgb();
                    let mut top_left = [new_x, new_y];
                    top_left[0] += origin[0];
                    top_left[1] += origin[1];
                    let bottom_right = [top_left[0] + width, top_left[1] + height];

                    let color = ImColor32::from_rgb(r, g, b);
                    draw_list
                        .add_rect(top_left, bottom_right, color)
                        .filled(true)
                        .build();
                }
            }
        });
}

fn draw_tiles(ui: &mut Ui, interconnect: &Interconnect) {
    ui.window("TILES")
        .size(
            [SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32],
            Condition::FirstUseEver,
        )
        .position([600.0, 600.0], Condition::FirstUseEver)
        .build(|| {
            let draw_list = ui.get_window_draw_list();
            let origin: [f32; 2] = ui.cursor_screen_pos();
            let mut x_draw: i32 = 0;
            let mut y_draw: i32 = 0;
            let mut tile_num: u16 = 0;

            let top_left = origin;
            let bottom_right = [
                origin[0] + SCREEN_WIDTH as f32,
                origin[1] + SCREEN_HEIGHT as f32,
            ];
            let background = ImColor32::from_rgb(17, 17, 17);
            draw_list
                .add_rect(top_left, bottom_right, background)
                .filled(true)
                .build();

            let addr: u16 = 0x8000;
            for y in 0..24 {
                for x in 0..16 {
                    display_tile(
                        &draw_list,
                        origin,
                        interconnect,
                        addr,
                        tile_num,
                        x_draw + (x * SCALE as i32),
                        y_draw + (y * SCALE as i32),
                    );
                    x_draw += 8 * SCALE as i32;
                    tile_num += 1;
                }
                y_draw += 8 * SCALE as i32;
                x_draw = 0;
            }
        });
}

fn display_tile(
    draw_list: &DrawListMut,
    origin: [f32; 2],
    interconnect: &Interconnect,
    start_loc: u16,
    tile_num: u16,
    x: i32,
    y: i32,
) {
    for tile_y in (0..16).step_by(2) {
        let addr: u16 = start_loc + (tile_num * 16) + tile_y;

        let second_byte = interconnect.read_mem(addr);
        let first_byte = interconnect.read_mem(addr + 1);

        let mut color: u8;

        for bit in (0..8).rev() {
            let first_bit = (first_byte >> bit) & 1;
            let second_bit = (second_byte >> bit) & 1;

            if first_bit == 0 && second_bit == 0 {
                color = 0;
            } else if first_bit == 0 && second_bit == 1 {
                color = 1;
            } else if first_bit == 1 && second_bit == 0 {
                color = 2;
            } else {
                color = 3;
            }

            let new_x = (x + ((7 - bit) * SCALE as i32)) as f32;
            let new_y = (y + ((tile_y as i32) / 2 * (SCALE as i32))) as f32;

            let width = SCALE as f32;
            let height = SCALE as f32;

            let (r, g, b) = TILE_COLORS[color as usize].get_rgb();
            let mut top_left = [new_x, new_y];
            top_left[0] += origin[0];
            top_left[1] += origin[1];
            let bottom_right = [top_left[0] + width, top_left[1] + height];
            let color = ImColor32::from_rgb(r, g, b);
            draw_list
                .add_rect(top_left, bottom_right, color)
                .filled(true)
                .build();
        }
    }
}

fn keycode_to_key(keycode: Keycode) -> Option<Key> {
    match keycode {
        Keycode::Right | Keycode::D => Some(Key::Right),
        Keycode::Left | Keycode::A => Some(Key::Left),
        Keycode::Up | Keycode::W => Some(Key::Up),
        Keycode::Down | Keycode::S => Some(Key::Down),
        Keycode::Z => Some(Key::A),
        Keycode::X => Some(Key::B),
        Keycode::Space => Some(Key::Select),
        Keycode::Return => Some(Key::Start),
        _ => None,
    }
}

pub fn run_sdl(gb: &mut GameBoy, headless: bool) -> Result<(), Error> {
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
                    Event::KeyUp { keycode, .. } => {
                        if let Some(key) = keycode.and_then(keycode_to_key) {
                            gb.interconnect.key_up(key)
                        }
                    }

                    Event::KeyDown { keycode, .. } => {
                        if let Some(key) = keycode.and_then(keycode_to_key) {
                            gb.interconnect.key_down(key)
                        }
                    }

                    _ => {}
                }
            }
            gui::debug_window(&mut debug_window, &gb.interconnect);
            gui::main_window(&mut main_window, &gb.interconnect);
        }
    }
    Ok(())
}
