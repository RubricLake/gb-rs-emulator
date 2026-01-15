pub struct Cartridge {
    rom: Vec<u8>,
    ram: Vec<u8>,
    // mapper: Mapper
}

impl Cartridge {
    pub fn new() -> Self {
        Self {
            rom: vec![],
            ram: vec![],
        }
    }

    // MBC 0
    pub fn read_rom(&self, addr: u16) -> u8 {
        self.rom[addr as usize]
    }

    // MBC 0
    pub fn read_ram(&self, addr: u16) -> u8 {
        let offset = 0xA000;
        self.ram[(addr - offset) as usize]
    }
    pub fn write(&mut self, addr: u16, value: u8) {
        unimplemented!();
    }

    pub fn set_cart_rom(&mut self, cart_rom: Vec<u8>) {
        self.rom = cart_rom;
    }
}
