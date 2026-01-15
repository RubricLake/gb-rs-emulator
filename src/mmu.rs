use crate::cartridge::Cartridge;

pub struct MMU {
    pub cart: Cartridge,
    boot_rom: Vec<u8>,
    boot_active: bool,
    vram: [u8; 0x2000], // 8 KB  [8000 - 9FFF]
    wram: [u8; 0x2000], // 8 KB  [C000 - DFFF]
    oam: [u8; 0xA0],    // 160 B [FE00 - FE9F]
    io: [u8; 0x80],     // 128 B [FF00 - FF7F]
    hram: [u8; 0x7F],   // 127 B [FF80 - FFFE]

    ie: u8, // Interrupt Enable [FFFF]
}

impl MMU {
    pub fn new() -> Self {
        Self {
            cart: Cartridge::new(),
            boot_rom: vec![],
            boot_active: true,
            vram: [0; 0x2000],
            wram: [0; 0x2000],
            oam: [0; 0xA0],
            io: [0; 0x80],
            hram: [0; 0x7F],
            ie: 0,
        }
    }

    pub fn set_boot_rom(&mut self, boot_rom: Vec<u8>) {
        self.boot_rom = boot_rom;
    }

    #[allow(unreachable_patterns)]
    pub fn read_u8(&self, addr: u16) -> u8 {
        match addr {
            // 16 KiB ROM BANK 00
            // From cartridge, usually a fixed bank
            0..=0x7FFF => self.cart.read_rom(addr), // Let mapper handle it

            // 16 KiB ROM BANK 01-NN
            // From cartridge, switchable bank via mapper (if any)
            // 0x4000..=0x7FFF => self.cart.read_rom(addr),

            // 8 KiB Video RAM (VRAM)
            // In CGB Mode, Switchable Bank 0/1
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize],

            // 8 KiB External RAM
            // From cartridge, switchable bank if any
            0xA000..=0xBFFF => self.cart.read_ram(addr), // Let mapper handle it

            // 4 KiB Work RAM (WRAM)
            0xC000..=0xDFFF => self.wram[(addr - 0xC000) as usize],

            // 4 Kib Work RAM (WRAM)
            // In CGB Mode, Switchable Bank 1-7
            // 0xD000..=0xDFFF => self.wram[(addr - 0xC000) as usize],

            // Echo RAM (Mirror of C000-DDFF)
            // Nintendo says use of this area is prohibited
            0xE000..=0xFDFF => self.read_u8(addr - 0x2000),

            // Object Attribute Memory
            0xFE00..=0xFE9F => self.oam[(addr - 0xFE00) as usize],

            // Not Usable
            // Nintendo says use of this area is prohibited
            0xFEA0..=0xFEFF => 0xFF,

            // IO Registers
            0xFF00..=0xFF7F => self.io[(addr - 0xFF00) as usize],

            // High RAM (HRAM)
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize],

            // Interrupt Enable Register (IE)
            0xFFFF => self.ie,

            _ => unreachable!(),
        }
    }

    pub fn write_u8(&mut self, addr: u16, value: u8) {}

    pub fn read_u16(&self, addr: u16) -> u16 {
        let low: u8 = self.read_u8(addr);
        let high: u8 = self.read_u8(addr.wrapping_add(1));
        u16::from_le_bytes([low, high])
    }

    pub fn write_u16(&mut self, addr: u16, value: u16) {
        let [low, high] = value.to_le_bytes();

        self.write_u8(addr, low);
        self.write_u8(addr.wrapping_add(1), high);
    }
}
