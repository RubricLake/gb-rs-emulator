mod cpu;
mod emulator;
mod mmu;

use emulator::Emulator;

fn main() {
    let mut emu = Emulator::new();

    // Halt Bug Present on Gameboy CPU
    emu.set_halt_bug();

    // Start Executing
    emu.start();
}
