pub struct CPU {
    pub reg: Registers,
    running: bool,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            reg: Default::default(),
            running: false,
        }
    }

    pub fn debug_print(&self) {
        let reg = &self.reg;
        let flags = &self.reg.f;

        println!("{:-^25}", "START");
        print!("REGISTERS:\n");
        print!("a: {:X}\n", reg.a);
        print!("b: {:X}\n", reg.b);
        print!("c: {:X}\n", reg.c);
        print!("d: {:X}\n", reg.d);
        print!("e: {:X}\n", reg.e);
        print!("h: {:X}\n", reg.h);
        print!("l: {:X}\n\n", reg.l);

        print!("POINTERS:\n");
        print!("sp: {:X}\n", reg.sp);
        print!("pc: {:X}\n\n", reg.pc);

        print!("FLAGS:\n");
        print!("z: {:}\n", flags.z.get() as u8);
        print!("n: {:}\n", flags.n.get() as u8);
        print!("h: {:}\n", flags.h.get() as u8);
        print!("c: {:}\n\n", flags.c.get() as u8);
        println!("{:-^25}", "END");
    }
}
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

#[derive(Default)]
struct Flags {
    z: Flag,
    n: Flag,
    h: Flag,
    c: Flag,
}

#[derive(Default)]
struct Flag {
    value: bool,
}

impl Flag {
    fn get(&self) -> bool {
        self.value
    }

    fn set(&mut self) {
        self.value = true;
    }

    fn unset(&mut self) {
        self.value = false;
    }

    fn toggle(&mut self) {
        self.value = !self.value;
    }

    fn set_if(&mut self, cond: bool) {
        self.value = cond;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
