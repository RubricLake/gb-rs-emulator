mod cartridge;
mod cpu;
mod emulator;
mod mmu;

use emulator::Emulator;
use std::env;
use std::path::{Path, PathBuf};

fn get_cartridge_path() -> PathBuf {
    env::args()
        .nth(1)
        .map(PathBuf::from)
        .expect("Usage: emulator <rom.gb>")
}

fn main() {
    // Debug Path For Now
    let cartridge_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("rom")
        .join("Tetris.gb");

    let mut emu = Emulator::new();
    
    // Load Cartridge From .gb File
    emu.load_cartridge(cartridge_path);

    // Halt Bug Present on Gameboy CPU
    emu.set_halt_bug();

    // Start Executing
    emu.start();
}
