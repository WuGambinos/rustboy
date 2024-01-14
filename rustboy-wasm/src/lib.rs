use rustboy::constants::TILE_COLORS;
use rustboy::interconnect::joypad::Key;
use rustboy::interconnect::ppu::Rgb;
use rustboy::{
    gameboy::GameBoy,
    interconnect::{
        cartridge::cartridge_info::{ram_size, u8_to_cart_type},
        cartridge::Cartridge,
    },
};
use std::f64;
use wasm_bindgen::prelude::*;
use web_sys::console;

const SCALE: i32 = 4;
const Y_RESOLUTION: u8 = 144;
const X_RESOLUTION: u8 = 160;
const BUFFER_SIZE: usize = (X_RESOLUTION as usize * Y_RESOLUTION as usize);

pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        console::time_with_label(name);
        Timer { name }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        console::time_end_with_label(self.name);
    }
}

#[wasm_bindgen]
pub fn js_value_to_joypad_key(value: JsValue) -> Option<Key> {
    match value.as_string().unwrap().as_str() {
        "ArrowLeft" | "a" => Some(Key::Left),
        "ArrowRight" | "d" => Some(Key::Right),
        "ArrowUp" | "w" => Some(Key::Up),
        "ArrowDown" | "s" => Some(Key::Down),
        "z" => Some(Key::B),
        "x" => Some(Key::A),
        " " => Some(Key::Select),
        "q" => Some(Key::Start),
        _ => None,
    }
}

#[wasm_bindgen]
pub struct WebGameBoy {
    gb: GameBoy,
    prev_buffer: Option<[Rgb; BUFFER_SIZE]>,
}

#[wasm_bindgen]
impl WebGameBoy {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WebGameBoy {
        WebGameBoy {
            gb: GameBoy::new(),
            prev_buffer: None,
        }
    }

    pub fn reset(&mut self) {
        self.gb = GameBoy::new();
        self.prev_buffer = None;
    }

    pub fn boot(&mut self, rom: &[u8]) {
        let game_rom = rom.to_vec();

        let cart_type_value = game_rom[0x147];
        let rom_size_value = game_rom[0x148];
        let ram_size_value = game_rom[0x149];
        let cart_type = u8_to_cart_type(cart_type_value);

        /*
        let cart_type = format!("{}", u8_to_cart_type(cart_type_value));
        let cart_type_js = JsValue::from_str(&cart_type);

        console::log_1(&"BOOTING".into());
        console::log_2(&"CART TYPE: ".into(), &cart_type_js);
        console::log_2(&"ROM SIZE: ".into(), &rom_size_value.into());
        console::log_2(&"RAM SIZE: ".into(), &ram_size_value.into());
        */
        let ram = vec![0x00; ram_size(ram_size_value) as usize];

        self.gb.interconnect.cartridge = Cartridge::new(&game_rom, &ram, &cart_type);
        self.gb.cpu.pc = 0x100;
    }

    pub fn on_key_down(&mut self, value: JsValue) {
        let key_pressed = js_value_to_joypad_key(value);
        if let Some(pressed) = key_pressed {
            self.gb.interconnect.key_down(pressed);
        }
    }

    pub fn on_key_up(&mut self, value: JsValue) {
        let key_released = js_value_to_joypad_key(value);

        if let Some(released) = key_released {
            self.gb.interconnect.key_up(released);
        }
    }

    pub fn run(&mut self) {
        let interconnect = &mut self.gb.interconnect;
        self.gb.cpu.run(interconnect);
    }
    pub fn draw(&mut self) {
        let _timer = Timer::new("WebGameBoy::draw");
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

        let video_buffer = self.gb.interconnect.ppu.video_buffer;
        for line_num in 0..Y_RESOLUTION {
            for x in 0..X_RESOLUTION {
                let new_x = u16::from(x) * (SCALE as u16);
                let new_y = u16::from(line_num) * (SCALE as u16);
                let w = SCALE as u32;
                let h = SCALE as u32;
                let index =
                    (u32::from(x) + (u32::from(line_num) * u32::from(X_RESOLUTION))) as usize;
                let color = video_buffer[index as usize];
                context.set_fill_style(&color.to_string().into());
                context.fill_rect(new_x as f64, new_y as f64, w as f64, h as f64);
            }
        }
        /*
            if let Some(p_buffer) = self.prev_buffer {
                for line_num in 0..Y_RESOLUTION {
                    for x in 0..X_RESOLUTION {
                        let new_x = u16::from(x) * (SCALE as u16);
                        let new_y = u16::from(line_num) * (SCALE as u16);
                        let w = SCALE as u32;
                        let h = SCALE as u32;
                        let index =
                            (u32::from(x) + (u32::from(line_num) * u32::from(X_RESOLUTION))) as usize;
                        let color = video_buffer[index as usize];
                        /*
                        if p_buffer[index as usize] != video_buffer[index] {
                            context.set_fill_style(&color.to_string().into());
                            context.fill_rect(new_x as f64, new_y as f64, w as f64, h as f64);
                        }
                        */
                    }
                }
            }
        */
        self.prev_buffer = Some(self.gb.interconnect.ppu.video_buffer);
    }
}
