use crate::mmu::MMU;
use crate::{log_print, log_println};

#[derive(Default)]
struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,

    pub sp: u16,
    pub pc: u16,

    f: Flags,
}

impl Registers {
    pub fn af(&self) -> u16 {
        ((self.a as u16) << 8) | self.f.as_u8() as u16
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f.load_u8(value as u8);
    }

    pub fn bc(&self) -> u16 {
        ((self.b as u16) << 8) | self.c as u16
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = value as u8;
    }

    pub fn de(&self) -> u16 {
        ((self.d as u16) << 8) | self.e as u16
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = value as u8;
    }

    pub fn hl(&self) -> u16 {
        ((self.h as u16) << 8) | self.l as u16
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = value as u8;
    }
}

#[derive(Default)]
struct Flags {
    f: u8,
}

impl Flags {
    const Z: u8 = 0b1000_0000;
    const N: u8 = 0b0100_0000;
    const H: u8 = 0b0010_0000;
    const C: u8 = 0b0001_0000;

    pub fn z(&self) -> bool {
        self.f & Self::Z != 0
    }

    pub fn n(&self) -> bool {
        self.f & Self::N != 0
    }

    pub fn h(&self) -> bool {
        self.f & Self::H != 0
    }

    pub fn c(&self) -> bool {
        self.f & Self::C != 0
    }

    pub fn set_z(&mut self, value: bool) {
        self.set(Self::Z, value);
    }
    pub fn set_n(&mut self, value: bool) {
        self.set(Self::N, value);
    }
    pub fn set_h(&mut self, value: bool) {
        self.set(Self::H, value);
    }
    pub fn set_c(&mut self, value: bool) {
        self.set(Self::C, value);
    }

    fn set(&mut self, mask: u8, value: bool) {
        if value {
            self.f |= mask;
        } else {
            self.f &= !mask;
        }

        self.f &= 0xF0;
    }

    pub fn as_u8(&self) -> u8 {
        self.f & 0xF0
    }

    pub fn load_u8(&mut self, value: u8) {
        self.f = value & 0xF0;
    }
    pub fn clear(&mut self) {
        self.f = 0;
    }
}
pub struct CPU {
    pub reg: Registers,
    pub ime: bool,
    pub halt: bool,
    pub halt_bug: bool,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            reg: Default::default(),
            ime: false,
            halt: false,
            halt_bug: false,
        }
    }

    pub fn step(&mut self, mmu: &mut MMU) -> u64 {
        if self.halt {
            return 4;
        }

        // Step Through Instruction
        // self.debug_print();

        // Return Cycles it Took
        0
    }

    pub fn debug_print(&self) {
        let reg = &self.reg;
        let flags = &self.reg.f;

        log_println!("{:-^25}", "START");
        log_print!("REGISTERS:\n");
        log_print!("a: {:02X}\n", reg.a);
        log_print!("b: {:02X}\n", reg.b);
        log_print!("c: {:02X}\n", reg.c);
        log_print!("d: {:02X}\n", reg.d);
        log_print!("e: {:02X}\n", reg.e);
        log_print!("h: {:02X}\n", reg.h);
        log_print!("l: {:02X}\n\n", reg.l);

        log_print!("POINTERS:\n");
        log_print!("sp: {:04X}\n", reg.sp);
        log_print!("pc: {:04X}\n\n", reg.pc);

        log_print!("FLAGS:\n");
        log_print!("z: {:}\n", flags.z() as u8);
        log_print!("n: {:}\n", flags.n() as u8);
        log_print!("h: {:}\n", flags.h() as u8);
        log_print!("c: {:}\n", flags.c() as u8);
        log_println!("{:-^25}", "END");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
