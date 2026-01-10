use crate::cpu::CPU;
use crate::mmu::MMU;
use std::path::PathBuf;

pub struct Emulator {
    cpu: CPU,
    mmu: MMU,
    // ppu: PPU,
    // cart: Cartridge,
    running: bool,
    paused: bool,
}

impl Emulator {
    pub fn new() -> Emulator {
        Emulator {
            cpu: CPU::new(),
            mmu: MMU::new(),
            running: false,
            paused: false,
        }
    }

    pub fn load_cartridge(&mut self, cartridge_path: PathBuf) {
        self.mmu.read_from_file(cartridge_path);
    }

    pub fn set_halt_bug(&mut self) {
        self.cpu.halt_bug = true;
    }

    pub fn start(&mut self) {
        self.running = true;

        while self.running {
            if !self.paused {
                // Tick CPU
                let cycles = self.cpu.step(&mut self.mmu);
                self.running = false;

                // Handle Timers

                // Handle Interrupts
            }
        }
    }
}
