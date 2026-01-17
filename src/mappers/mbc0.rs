use crate::mappers::Mapper;

// No Mapper
pub struct MBC0 {
    has_ram: bool,
}

impl MBC0 {
    pub fn new(has_ram: bool) -> Self {
        Self { has_ram }
    }
}

impl Mapper for MBC0 {
    fn map_rom(&self, addr: u16) -> usize {
        addr as usize
    }

    fn map_ram(&self, addr: u16) -> Option<usize> {
        if self.has_ram {
            let ram_index = (addr - 0xA000) as usize;
            Some(ram_index)
        } else {
            None
        }
    }

    // Intentionally Left Blank
    // No Action on Rom Writes
    fn write_control(&mut self, addr: u16, value: u8) {}
}
