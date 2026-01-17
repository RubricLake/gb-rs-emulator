use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use crate::log_println;
use crate::mmu::MMU;
use std::io;
use std::path::Path;

const CPU_HZ: u64 = 4_194_304;
const NS_PER_SEC: u64 = 1_000_000_000;
const NS_PER_CYCLE: u64 = NS_PER_SEC / CPU_HZ;

pub struct Emulator {
    cpu: CPU,
    mmu: MMU,
    // ppu: PPU,
    running: bool,
    paused: bool,
}

impl Emulator {
    pub fn new() -> Self {
        Self {
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
        let rom = std::fs::read(cartridge_path)?;

        let cartridge = Cartridge::new(rom);

        self.mmu.insert_cartridge(cartridge);

        Ok(())
    }

    pub fn set_halt_bug(&mut self) {
        self.cpu.halt_bug = true;
    }

    pub fn start(&mut self) {
        self.running = true;

        let mut cycles_accumulated: u64 = 0;
        let mut last_sync = std::time::Instant::now();

        while self.running {
            if self.paused {
                std::thread::sleep(std::time::Duration::from_millis(1));
                continue;
            }

            // Tick CPU
            let cycles: u64 = self.cpu.step(&mut self.mmu);
            cycles_accumulated += cycles;

            // Handle Timers
            // self.timers.tick(cycles);

            // Handle ppu
            // self.ppu.tick(cycles);

            // Handle Interrupts
            // self.handle_interrupts();

            // Sync Time
            let elapsed = last_sync.elapsed();
            let emulated_ns = (cycles_accumulated * NS_PER_CYCLE) as u128;

            if elapsed.as_nanos() < emulated_ns {
                let sleep_ns = emulated_ns - elapsed.as_nanos();
                std::thread::sleep(std::time::Duration::from_nanos(sleep_ns as u64));
            } else {
                // Emulator needs to catch up
            }

            last_sync = std::time::Instant::now();
            cycles_accumulated = 0;
        }
    }
}
