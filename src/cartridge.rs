use std::fs::File;
use std::path::PathBuf;

#[derive(Default)]
pub struct Cartridge {}

impl Cartridge {
    pub fn new() -> Self {
        Cartridge::default()
    }

    pub fn read_cartridge(&mut self, cartridge_path: PathBuf) {
        // Read In Cartridge

    }
}
