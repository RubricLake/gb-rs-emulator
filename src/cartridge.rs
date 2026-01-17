use crate::mappers::{Mapper, mbc0::MBC0};

pub struct Cartridge {
    rom: Vec<u8>,
    ram: Vec<u8>,
    mapper: Box<dyn Mapper>,
}

impl Cartridge {
    pub fn new(rom: Vec<u8>) -> Self {
        let cart_type = rom[0x147];
        let ram_code = rom[0x149];

        let ram_size = Self::decode_ram_size(ram_code) * 0x400;
        let has_ram = ram_size > 0;
        let ram = vec![0; ram_size];

        let mapper: Box<dyn Mapper> = match cart_type {
            0x00 => Box::new(MBC0::new(has_ram)),
            _ => unimplemented!("Mapper Unimplmented Yet."),
        };

        Self { rom, ram, mapper }
    }

    fn decode_ram_size(ram_code: u8) -> usize {
        match ram_code {
            0x00 => 0,      // No RAM
            0x01 => 2,      // Listed as 2 KiB in various unofficial docs; unused.
            0x02 => 8,      // 1 Bank
            0x03 => 32,     // 4 Banks of 8 KiB each
            0x04 => 128,    // 16 Banks of 8 KiB each
            0x05 => 64,     // 8 Banks of 8 Kib each
            _ => panic!("Invalid RAM code from cartridge byte {:#04x}", ram_code),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        let ram_addr = self.mapper.map_ram(addr);

        if let Some(addr) = ram_addr {
            self.ram[addr] = value;
        } else if addr <= 0x7FFF {
            self.mapper.write_control(addr, value);
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        let ram_addr = self.mapper.map_ram(addr);
        let rom_addr = self.mapper.map_rom(addr);

        if let Some(addr) = ram_addr {
            self.ram[addr]
        } else if addr <= 0x7FFF {
            self.rom[rom_addr]
        } else {
            0x00FF
        }
    }
}
