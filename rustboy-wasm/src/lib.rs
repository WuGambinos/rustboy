use lazy_static::*;
use rustboy::{
    gameboy::GameBoy,
    interconnect::{cartridge::Cartridge, cartridge_info::ram_size},
};
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use web_sys::console;

const PIXEL_SIZE: f64 = 4.;

const SCALE: i32 = 4;
const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 640;
const Y_RESOLUTION: u8 = 144;
const X_RESOLUTION: u8 = 160;

lazy_static! {
    static ref GB: Mutex<GameBoy> = {
        let gb = GameBoy::new();
        Mutex::new(gb)
    };
}

#[wasm_bindgen]
extern "C" {
    fn draw_pixel(x: i32, y: i32, w: f64, h: f64, s: &str);
}

#[wasm_bindgen]
pub fn load_rom(rom: &[u8]) {
    GB.lock().unwrap().interconnect.load_game_rom(rom);
}

#[wasm_bindgen]
pub fn boot(rom: &[u8]) {
    let game_rom = rom.to_vec();

    let cart_type = game_rom[0x147];
    let rom_size = game_rom[0x148];
    let ram_s = game_rom[0x149];
    console::log_1(&"BOOTING".into());
    console::log_2(&"CART TYPE: ".into(), &cart_type.into());
    console::log_2(&"ROM SIZE: ".into(), &rom_size.into());
    console::log_2(&"RAM SIZE: ".into(), &ram_s.into());
    let ram = vec![0x00; ram_size(ram_s) as usize];

    GB.lock().unwrap().interconnect.cartridge = Cartridge::new(&game_rom, &ram);
    GB.lock().unwrap().cpu.pc = 0x100;
}

#[wasm_bindgen]
pub fn run_gameboy() {
    GB.lock()
        .unwrap()
        .cpu
        .run(&mut GB.lock().unwrap().interconnect);
}

/*
#[wasm_bindgen]
pub struct WebGameBoy {
    gb: GameBoy,
}

#[wasm_bindgen]
impl WebGameBoy {
    pub fn new() -> WebGameBoy {
        WebGameBoy { gb: GameBoy::new() }
    }
}
*/

use std::f64;

#[wasm_bindgen]
pub fn start() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();

    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let red: &JsValue = &"#FF0000".into();
    let black: &JsValue = &"#000000".into();
    let white: &JsValue = &"#FFFFFF".into();

    for y in 0..Y_RESOLUTION {
        for x in 0..X_RESOLUTION {
            if y % 2 == 0 {
                if x % 2 == 0 {
                    context.set_fill_style(red);
                } else {
                    context.set_fill_style(white);
                }
            } else {
                if x % 2 == 0 {
                    context.set_fill_style(white);
                } else {
                    context.set_fill_style(red);
                }
            }
            context.fill_rect(x as f64, y as f64, PIXEL_SIZE, PIXEL_SIZE);
        }
    }
}

#[wasm_bindgen]
pub fn draw() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();

    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    let video_buffer = GB.lock().unwrap().interconnect.ppu.video_buffer;

    let red: &JsValue = &"#FF0000".into();
    let black: &JsValue = &"#000000".into();
    let white: &JsValue = &"#FFFFFF".into();

    for line_num in 0..Y_RESOLUTION {
        for x in 0..X_RESOLUTION {
            let new_x = u16::from(x) * (SCALE as u16);
            let new_y = u16::from(line_num) * (SCALE as u16);
            let w = SCALE as u32;
            let h = SCALE as u32;

            let index = u32::from(x) + (u32::from(line_num) * u32::from(X_RESOLUTION));
            let color = video_buffer[index as usize];

            //context.set_fill_style(&color.to_string());
            context.set_fill_style(red);
            context.fill_rect(new_x as f64, new_y as f64, w as f64, h as f64);
        }
    }
}
