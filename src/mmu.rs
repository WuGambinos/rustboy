/// Memory Mapped Unit
#[derive(Debug)]
pub struct Mmu {
    /// BOOT ROM
    pub boot: [u8; 0x100],

    /// ROM Bank
    pub rom_bank: [u8; 0x8000],

    /// IO Registers
    pub io: [u8; 0x80],

    /// High RAM (HRAM)
    pub hram: [u8; 0x7F],

    /// External RAM
    pub external_ram: [u8; 0x2000],

    /// Work RAM
    pub work_ram: [u8; 0x2000],

    // interrupt_enable
    pub interrupt_enable: u8,
}

impl Default for Mmu {
    fn default() -> Self {
        Self::new()
    }
}

impl Mmu {
    /// Constructor
    pub fn new() -> Self {
        Mmu {
            boot: [0; 0x100],
            rom_bank: [0; 0x8000],
            io: [0; 0x80],
            hram: [0; 0x7F],
            external_ram: [0; 0x2000],
            work_ram: [0; 0x2000],
            interrupt_enable: 0,
        }
    }


    pub fn write_boot(&mut self, addr: u16, value: u8) {
        self.boot[addr as usize] = value;
    }

    pub fn write_rom_bank(&mut self, addr: u16, value: u8) {
        self.rom_bank[addr as usize] = value;
    }

    pub fn write_io(&mut self, addr: u16, value: u8) {
        self.io[addr as usize] = value;
    }

    pub fn write_hram(&mut self, addr: u16, value: u8) {
        self.hram[addr as usize] = value;
    }

    pub fn write_external_ram(&mut self, addr: u16, value: u8) {
        self.external_ram[addr as usize] = value;
    }

    pub fn write_work_ram(&mut self, addr: u16, value: u8) {
        self.work_ram[addr as usize] = value;
    }

    pub fn enable_interrupt(&mut self, value: u8) {
        self.interrupt_enable = value;
    }

    pub fn read_boot(&self, addr: u16) -> u8 {
        self.boot[addr as usize]
    }

    pub fn read_rom_bank(&self, addr: u16) -> u8 {
        self.rom_bank[addr as usize]
    }

    pub fn read_io(&self, addr: u16) -> u8 {
        self.io[addr as usize]
    }

    pub fn read_hram(&self, addr: u16) -> u8 {
        self.hram[addr as usize]
    }

    pub fn read_external_ram(&self, addr: u16) -> u8 {
        self.external_ram[addr as usize]
    }

    pub fn read_work_ram(&self, addr: u16) -> u8 {
        self.work_ram[addr as usize]
    }

    pub fn read_interrupt_enable(&self) -> u8 {
        self.interrupt_enable
    }
}
