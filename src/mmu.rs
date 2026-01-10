use crate::cartridge::Cartridge;
use std::path::PathBuf;

pub struct MMU {
    cart: Cartridge,

    vram: [u8; 0x2000], // 8 KB  [8000 - 9FFF]
    wram: [u8; 0x2000], // 8 KB  [C000 - DFFF]
    io: [u8; 0x80],     // 128 B [FF00 - FF7F]
    hram: [u8; 0x7F],   // 127 B [FF80 - FFFE]

    ie: u8, // Interrupt Enable [FFFF]
}

impl MMU {
    pub fn new() -> MMU {
        MMU {
            cart: Cartridge::new(),
            vram: [0; 0x2000],
            wram: [0; 0x2000],
            io: [0; 0x80],
            hram: [0; 0x7F],
            ie: 0,
        }
    }
    pub fn read_from_file(&mut self, cartridge_path: PathBuf) {
        println!("Reading from file: {:?}", cartridge_path);
    }
    pub fn read_u8(&self, addr: u16) -> u8 {
        0
    }

    pub fn write_u8(&self, addr: u16, data: u8) {}
}
