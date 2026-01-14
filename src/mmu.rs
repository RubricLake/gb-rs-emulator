use crate::cartridge::Cartridge;

pub struct MMU {
    pub cart: Cartridge,
    boot_rom: Vec<u8>,
    boot_active: bool,
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
            boot_rom: vec![],
            boot_active: true,
            vram: [0; 0x2000],
            wram: [0; 0x2000],
            io: [0; 0x80],
            hram: [0; 0x7F],
            ie: 0,
        }
    }

    pub fn set_boot_rom(&mut self, boot_rom: Vec<u8>) {
        self.boot_rom = boot_rom;
    }

    pub fn read_u8(&self, addr: u16) -> u8 {
        0
    }

    pub fn write_u8(&self, addr: u16, data: u8) {}
}
