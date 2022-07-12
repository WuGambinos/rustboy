pub struct OamEntry {

    ///Sprite's vertical position on the screen + 16
    y: u8,

    ///Sprite's horizontal position on the screen + 8
    x: u8,

    //
    tile: u8,
}


impl OamEntry {
    pub fn new() {}
}

pub struct PPU {
    vram: [u8; 0x2000],
}

impl PPU {
    pub fn new() -> Self {
        Self { vram: [0; 0x2000] }
    }

    pub fn tick() {}
}
