use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use crate::mmu::MMU;
use std::io;
use std::path::{Path};

#[macro_export]
macro_rules! log_println {
    () => (println!("[GAMEBOY]"));
    ($fmt:expr, $($arg:tt)*) => (println!(concat!("[GAMEBOY] ", $fmt), $($arg)*));
    ($fmt:expr) => (println!(concat!("[GAMEBOY] ", $fmt)));
}

#[macro_export]
macro_rules! log_print {
    () => (print!("[GAMEBOY]"));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!("[GAMEBOY] ", $fmt), $($arg)*));
    ($fmt:expr) => (print!(concat!("[GAMEBOY] ", $fmt)));
}

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

    pub fn load_boot_rom(&mut self, boot_path: &Path) -> io::Result<()> {
        log_println!("Reading boot rom from {:?}", boot_path);
        let data = std::fs::read(boot_path)?;

        self.mmu.set_boot_rom(data);

        Ok(())
    }

    pub fn insert_cartridge(&mut self, cartridge_path: &Path) -> io::Result<()> {
        log_println!("Reading cartridge from {:?}", cartridge_path);
        let data = std::fs::read(cartridge_path)?;

        self.mmu.cart.set_cart_rom(data);

        Ok(())
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
