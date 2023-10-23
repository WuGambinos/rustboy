use crate::constants::{MAIN_SCREEN_HEIGHT, MAIN_SCREEN_WIDTH, SCALE, SCREEN_HEIGHT, SCREEN_WIDTH};
use imgui::{Condition, DrawListMut, ImColor32, Ui};
use rustboy::constants::{TILE_COLORS, X_RESOLUTION, Y_RESOLUTION};
use rustboy::gameboy::GameBoy;
use rustboy::interconnect::Interconnect;

pub fn memory_viewer(ui: &mut Ui, gameboy: &GameBoy) {
    let rom_size = 0xFFFF;

    let mut row = String::new();
    for i in 0..rom_size{
        row.push_str(format!("{:X} ", gameboy.interconnect.read_mem(i)).as_str());
        if i % 16 == 0 && i != 0 {
            ui.text(row.clone());
            row.clear();
            row.push_str(format!("{:#X}0: ", i / 16).as_str());
        }
    }
}

pub fn debug_window(ui: &mut Ui, gameboy: &GameBoy) {
    ui.window("Debug Window")
        .position([200.0, 500.0], Condition::FirstUseEver)
        .size([150.0, 200.0], Condition::FirstUseEver)
        .build(|| {
            let pc = format!("PC: {:#X}", gameboy.cpu.pc);
            let sp = format!("SP: {:#X}", gameboy.cpu.sp);
            let opcode = format!("OPCODE: {:#X}", gameboy.cpu.opcode);
            let a = format!("A: {:#X}", gameboy.cpu.registers.a);
            let b = format!("B: {:#X}", gameboy.cpu.registers.b);
            let c = format!("C: {:#X}", gameboy.cpu.registers.c);
            let d = format!("D: {:#X}", gameboy.cpu.registers.d);
            let e = format!("E: {:#X}", gameboy.cpu.registers.e);
            let h = format!("H: {:#X}", gameboy.cpu.registers.h);
            let l = format!("L: {:#X}", gameboy.cpu.registers.l);
            let flags = format!("Flags: {:#X}", gameboy.cpu.registers.f.data);

            ui.text(pc);
            ui.text(sp);
            ui.text(opcode);
            ui.text(a);
            ui.text(b);
            ui.text(c);
            ui.text(d);
            ui.text(e);
            ui.text(h);
            ui.text(l);
            ui.text(flags);
        });
}

pub fn display_info(ui: &mut Ui, gameboy: &GameBoy) {
    ui.window("Info")
        .size([200.0, 400.0], Condition::FirstUseEver)
        .position([400.0, 600.0], Condition::FirstUseEver)
        .build(|| {
            let title = format!("TITLE: {}", &gameboy.interconnect.cartridge.title);
            let cart_type = format!(
                "CART TYPE: {:?}",
                &gameboy.interconnect.cartridge.cartridge_type
            );

            ui.text(title);
            ui.text(cart_type);
        });
}

pub fn display_emulator(ui: &mut Ui, gameboy: &GameBoy) {
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

pub fn draw_tiles(ui: &mut Ui, interconnect: &Interconnect) {
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
                        x_draw + x * SCALE,
                        y_draw + y * SCALE,
                    );
                    x_draw += 8 * SCALE;
                    tile_num += 1;
                }
                y_draw += 8 * SCALE;
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

            let new_x = (x + ((7 - bit) * SCALE)) as f32;
            let new_y = (y + ((tile_y as i32) / 2 * SCALE)) as f32;

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

/*
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
*/

/*
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

*/
