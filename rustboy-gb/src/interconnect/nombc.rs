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
        if addr < 0x8000 {
            self.rom[addr as usize]
        } else {
            0xFF
        }
    }

    fn write(&mut self, addr: u16, value: u8) {}
}
