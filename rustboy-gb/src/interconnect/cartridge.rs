use crate::interconnect::cartridge_info::CartridgeType;
use crate::interconnect::mbc1::Mbc1;

use super::mbc2::Mbc2;
use super::mbc3::Mbc3;
use super::mbc5::Mbc5;
use super::nombc::NoMbc;

pub struct Cartridge {
    pub title: String,
    pub cartridge_type: CartridgeType,
    pub valid_checksum: bool,
    pub mbc: Box<dyn Mbc>,
}

impl Cartridge {
    pub fn empty() -> Cartridge {
        Cartridge {
            title: String::new(),
            cartridge_type: CartridgeType::ROMOnly,
            valid_checksum: false,
            mbc: Box::new(NoMbc::new(&[])),
        }
    }

    pub fn new(rom: &Vec<u8>, ram: &Vec<u8>, cart_type: &CartridgeType) -> Cartridge {
        let mbc: Box<dyn Mbc> = match cart_type {
            CartridgeType::ROMOnly => Box::new(NoMbc::new(rom)),
            CartridgeType::MBC1 => Box::new(Mbc1::new(rom, ram)),
            CartridgeType::MBC1RAM => Box::new(Mbc1::new(rom, ram)),
            CartridgeType::MBC1RAMBattery => Box::new(Mbc1::new(rom, ram)),
            CartridgeType::MBC2 => Box::new(Mbc2::new(rom)),
            CartridgeType::MBC2Battery => Box::new(Mbc2::new(rom)),
            CartridgeType::MBC3 => Box::new(Mbc3::new(rom, ram)),
            CartridgeType::MBC3RAM => Box::new(Mbc3::new(rom, ram)),
            CartridgeType::MBC3RAMBattery => Box::new(Mbc3::new(rom, ram)),
            CartridgeType::MBC3TimerBattery => Box::new(Mbc3::new(rom, ram)),
            CartridgeType::MBC3TimerRAMBattery => Box::new(Mbc3::new(rom, ram)),
            CartridgeType::MBC5 => Box::new(Mbc5::new(rom, ram)),
            CartridgeType::MBC5RAM => Box::new(Mbc5::new(rom, ram)),
            CartridgeType::MBC5RAMBattery => Box::new(Mbc5::new(rom, ram)),
            CartridgeType::MBC5Rumble => Box::new(Mbc5::new(rom, ram)),
            CartridgeType::MBC5RumbleRAM => Box::new(Mbc5::new(rom, ram)),
            CartridgeType::MBC5RumbleRAMBattery => Box::new(Mbc5::new(rom, ram)),
            _ => Box::new(NoMbc::new(&[])),
        };

        Cartridge {
            title: String::new(),
            cartridge_type: CartridgeType::ROMOnly,
            valid_checksum: false,
            mbc,
        }
    }

    pub fn checksum(&mut self) -> bool {
        let mut check_sum: i16 = 0;

        for addr in 0x134..=0x14C {
            check_sum = check_sum
                .wrapping_sub(self.mbc.read(addr) as i16)
                .wrapping_sub(1);
        }

        self.valid_checksum = (check_sum & 0xF) == (self.mbc.read(0x14D) & 0xF) as i16;
        self.valid_checksum
    }
}

pub trait Mbc {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, value: u8);
}
