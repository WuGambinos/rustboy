use crate::{
    constants::{RAM_BANK_SIZE, ROM_BANK_SIZE},
    interconnect::cartridge_info::CartridgeType,
};

#[derive(Debug)]
pub struct Cartridge {
    pub title: String,
    pub cartridge_type: CartridgeType,
    pub valid_checksum: bool,
    pub mbc: Mbc1,
}

impl Cartridge {
    pub fn empty() -> Cartridge {
        Cartridge {
            title: String::new(),
            cartridge_type: CartridgeType::ROMOnly,
            valid_checksum: false,
            mbc: Mbc1::empty(),
        }
    }

    pub fn new(rom: &Vec<u8>, ram: &Vec<u8>) -> Cartridge {
        Cartridge {
            title: String::new(),
            cartridge_type: CartridgeType::ROMOnly,
            valid_checksum: false,
            mbc: Mbc1::new(rom, ram),
        }
    }

    pub fn checksum(&mut self) -> bool {
        let mut check_sum: i16 = 0;

        for addr in 0x134..=0x14C {
            check_sum = check_sum
                .wrapping_sub(self.mbc.rom[addr] as i16)
                .wrapping_sub(1);
        }

        self.valid_checksum = (check_sum & 0xF) == (self.mbc.rom[0x14D] & 0xF) as i16;

        self.valid_checksum
    }
}

#[derive(Debug)]
pub enum BankingMode {
    Rom,
    Ram,
}

#[derive(Debug)]
pub struct Mbc1 {
    pub rom: Vec<u8>,
    pub ram: Vec<u8>,
    rom_bank_number: usize,
    ram_bank_number: usize,
    ram_enabled: bool,
    banking_mode: BankingMode,
}

impl Mbc1 {
    pub fn new(rom: &[u8], ram: &[u8]) -> Mbc1 {
        Mbc1 {
            rom: rom.to_vec(),
            ram: ram.to_vec(),
            ram_enabled: false,
            banking_mode: BankingMode::Rom,
            rom_bank_number: 1,
            ram_bank_number: 0,
        }
    }

    pub fn empty() -> Mbc1 {
        Mbc1 {
            rom: vec![0; 0xFFFF],
            ram: vec![0; 0x8000],
            ram_enabled: false,
            banking_mode: BankingMode::Rom,
            rom_bank_number: 1,
            ram_bank_number: 0,
        }
    }

    pub fn get_rom_address(&self, addr: u16, bank: usize) -> usize {
        let offset = bank * ROM_BANK_SIZE;
        let real_address = (addr & 0x3FFF) as usize + offset;
        real_address & (self.rom.len() - 1)
    }

    pub fn get_ram_address(&self, addr: u16, bank: usize) -> usize {
        let offset = bank * RAM_BANK_SIZE;
        let real_address = (addr & 0x1FFF) as usize + offset;
        real_address & (self.ram.len() - 1)
    }

    pub fn get_lower_rom_bank(&self) -> usize {
        match self.banking_mode {
            BankingMode::Rom => 0,
            BankingMode::Ram => (self.ram_bank_number << 5) as usize,
        }
    }

    pub fn get_upper_rom_bank(&self) -> usize {
        (self.rom_bank_number | self.ram_bank_number << 5) as usize
    }

    pub fn get_ram_bank(&self) -> usize {
        match self.banking_mode {
            BankingMode::Rom => 0,
            BankingMode::Ram => self.ram_bank_number as usize,
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x3FFF => {
                let bank = self.get_lower_rom_bank();
                let new_addr = self.get_rom_address(addr, bank);
                self.rom[new_addr as usize]
            }
            0x4000..=0x7FFF => {
                let bank = self.get_upper_rom_bank();
                let new_addr = self.get_rom_address(addr, bank);
                /*
                let address_into_bank = addr as usize - ROM_BANK_SIZE;
                let bank_offset = ROM_BANK_SIZE * self.rom_bank_number;
                let address_in_rom = bank_offset + address_into_bank as usize;
                */
                self.rom[new_addr]
            }

            0xA000..=0xBFFF => {
                if self.ram.len() == 0 || !self.ram_enabled {
                    return 0xFF;
                }
                let bank = self.get_ram_bank();
                let ram_addr = self.get_ram_address(addr, bank);

                /*
                let offset_into_ram = RAM_BANK_SIZE * self.ram_bank_number;
                let address_in_ram = (addr - 0xA000) as usize + offset_into_ram;
                */
                self.ram[ram_addr]
            }
            _ => panic!("NOT A REACHABLE ADDRESS"),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x000..=0x1FFF => self.ram_enabled = value == 0xA,
            0x2000..=0x3FFF => {
                if value == 0x0 {
                    self.rom_bank_number = 0x1;
                    return;
                } else if value == 0x20 {
                    self.rom_bank_number = 0x21;
                    return;
                } else if value == 0x40 {
                    self.rom_bank_number = 0x41;
                    return;
                } else if value == 0x60 {
                    self.rom_bank_number = 0x61;
                    return;
                }

                let rom_bank_bits = value & 0x1F;
                self.rom_bank_number = rom_bank_bits as usize;
            }

            0x4000..=0x5FFF => {
                let data = value & 0b11;
                self.ram_bank_number = data as usize;
            }

            0x6000..=0x7FFF => {
                self.banking_mode = if (value & 1) > 0 {
                    BankingMode::Ram
                } else {
                    BankingMode::Rom
                }
            }

            0xA000..=0xBFFF => {
                if !self.ram_enabled {
                    return;
                }

                let offset_into_ram = RAM_BANK_SIZE * self.ram_bank_number;

                let address_into_ram = (addr - 0xA000) as usize + offset_into_ram;

                self.ram[address_into_ram] = value;
            }
            _ => panic!("NOT A REACHABLE ADDRESS ADDR: {:#X}", addr),
        }
    }
}
