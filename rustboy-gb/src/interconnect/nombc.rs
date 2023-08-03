use super::cartridge::Mbc;

#[derive(Debug)]
pub struct NoMbc {
    pub rom: Vec<u8>,
}

impl NoMbc {
    pub fn new(rom: &[u8]) -> NoMbc {
        NoMbc { rom: rom.to_vec() }
    }
}

impl Mbc for NoMbc {
    fn read(&self, addr: u16) -> u8 {
        self.rom[addr as usize]
    }

    fn write(&mut self, addr: u16, value: u8) {
        self.rom[addr as usize] = value;
    }
}
