mod cpu;

use cpu::CPU;

fn main() {
    let cpu = CPU::new();
    cpu.debug_print();
}
