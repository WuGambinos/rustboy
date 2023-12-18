use crate::interconnect::cartridge_info::CartridgeType;

use super::mbc1::Mbc1State;
use super::mbc2::Mbc2;
use super::mbc3::Mbc3;
use super::mbc5::Mbc5;
use super::nombc::NoMbc;

pub enum TestMbc {
    NoMbc(NoMbc),
    Mbc1(Mbc1State),
    Mbc2(Mbc2),
    Mbc3(Mbc3),
    Mbc5(Mbc5),
}

impl TestMbc {
    pub fn read(&self, addr: u16) -> u8 {
        match self {
            TestMbc::NoMbc(mbc) => mbc.read(addr),
            TestMbc::Mbc1(mbc) => mbc.read(addr),
            TestMbc::Mbc2(mbc) => mbc.read(addr),
            TestMbc::Mbc3(mbc) => mbc.read(addr),
            TestMbc::Mbc5(mbc) => mbc.read(addr),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match self {
            TestMbc::NoMbc(mbc) => mbc.write(addr, value),
            TestMbc::Mbc1(mbc) => mbc.write(addr, value),
            TestMbc::Mbc2(mbc) => mbc.write(addr, value),
            TestMbc::Mbc3(mbc) => mbc.write(addr, value),
            TestMbc::Mbc5(mbc) => mbc.write(addr, value),
        }
    }
}

pub struct Cartridge {
    pub title: String,
    pub cartridge_type: CartridgeType,
    pub valid_checksum: bool,
    pub mbc: TestMbc,
}

impl Cartridge {
    pub fn empty() -> Cartridge {
        Cartridge {
            title: String::new(),
            cartridge_type: CartridgeType::ROMOnly,
            valid_checksum: false,
            mbc: TestMbc::NoMbc(NoMbc::new(&[])),
        }
    }

    pub fn new(rom: &Vec<u8>, ram: &Vec<u8>, cart_type: &CartridgeType) -> Cartridge {
        let mbc_test: TestMbc = match cart_type {
            CartridgeType::ROMOnly => TestMbc::NoMbc(NoMbc::new(rom)),
            CartridgeType::MBC1 => TestMbc::Mbc1(Mbc1State::new(rom, ram)),
            CartridgeType::MBC1RAM => TestMbc::Mbc1(Mbc1State::new(rom, ram)),
            CartridgeType::MBC1RAMBattery => TestMbc::Mbc1(Mbc1State::new(rom, ram)),
            CartridgeType::MBC2 => TestMbc::Mbc2(Mbc2::new(rom)),
            CartridgeType::MBC2Battery => TestMbc::Mbc2(Mbc2::new(rom)),
            CartridgeType::MBC3 => TestMbc::Mbc3(Mbc3::new(rom, ram)),
            CartridgeType::MBC3RAM => TestMbc::Mbc3(Mbc3::new(rom, ram)),
            CartridgeType::MBC3RAMBattery => TestMbc::Mbc3(Mbc3::new(rom, ram)),
            CartridgeType::MBC3TimerBattery => TestMbc::Mbc3(Mbc3::new(rom, ram)),
            CartridgeType::MBC3TimerRAMBattery => TestMbc::Mbc3(Mbc3::new(rom, ram)),
            CartridgeType::MBC5 => TestMbc::Mbc5(Mbc5::new(rom, ram)),
            CartridgeType::MBC5RAM => TestMbc::Mbc5(Mbc5::new(rom, ram)),
            CartridgeType::MBC5RAMBattery => TestMbc::Mbc5(Mbc5::new(rom, ram)),
            CartridgeType::MBC5Rumble => TestMbc::Mbc5(Mbc5::new(rom, ram)),
            CartridgeType::MBC5RumbleRAM => TestMbc::Mbc5(Mbc5::new(rom, ram)),
            CartridgeType::MBC5RumbleRAMBattery => TestMbc::Mbc5(Mbc5::new(rom, ram)),
            _ => TestMbc::NoMbc(NoMbc::new(rom)),
        };

        Cartridge {
            title: String::new(),
            cartridge_type: CartridgeType::ROMOnly,
            valid_checksum: false,
            mbc: mbc_test,
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

/*
pub trait Mbc {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, value: u8);
}
*/
