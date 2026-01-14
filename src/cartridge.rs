#[derive(Default)]
pub struct Cartridge {
    rom: Vec<u8>,
    ram: Vec<u8>,
    // mapper: Mapper
}

impl Cartridge {
    pub fn new() -> Self {
        Cartridge::default()
    }

    pub fn read(&self, addr: u16) -> u8 {
        unimplemented!();
    }

    pub fn set_cart_rom(&mut self, cart_rom: Vec<u8>) {
        self.rom = cart_rom;
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        unimplemented!();
    }
}
