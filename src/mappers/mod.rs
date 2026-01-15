pub trait Mapper {
    fn map_rom(&self, addr: u16) -> usize;
    fn map_ram(&self, addr: u16) -> Option<usize>;

    fn write_rom(&mut self, addr: u16, value: u8);
}
pub mod mbc0;
