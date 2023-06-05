use std::path::Path;

use bios::Bios;
use cpu::Cpu;
use interconnect::Interconnect;

mod bios;
mod cpu;
mod instruction;
mod interconnect;

fn main() {
    let bios = Bios::load_bios(Path::new("F:\\Projects\\KISX\\target\\debug\\SCPH1001.BIN"));
    let inter = Interconnect::new(bios.unwrap());
    let mut cpu = Cpu::new(inter);
    loop {
        cpu.cycle();
    }
}
