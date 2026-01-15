mod cartridge;
mod cpu;
mod emulator;
mod macros;
mod mappers;
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
    let cartridge_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("rom")
        .join("Tetris.gb");

    let boot_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("rom")
        .join("dmg_boot.bin");

    let mut emu = Emulator::new();

    // Load Boot ROM from file
    match emu.load_boot_rom(&boot_path) {
        Ok(_) => log_println!("Successfully loaded boot ROM"),
        Err(error) => log_println!("Failed to load boot ROM: {}", error),
    }

    // Load Cartridge ROM from file
    match emu.insert_cartridge(&cartridge_path) {
        Ok(_) => log_println!("Successfully inserted cartridge"),
        Err(error) => log_println!("Failed to insert cartridge: {}", error),
    }

    // Halt Bug Present on Gameboy CPU
    emu.set_halt_bug();

    // Start Executing
    emu.start();
}
