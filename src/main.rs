use std::path::Path;

use cpu::Cpu;
use bios::Bios;
use interconnect::Interconnect;

mod bios;
mod cpu;
mod interconnect;
mod instruction;

fn main() {
    let bios = Bios::load_bios(Path::new("/home/kime/Projects/KISX/target/debug/SCPH1001.BIN"));
    let inter = Interconnect::new(bios.unwrap());
    let cpu = Cpu::new(inter);
    cpu.cycle();
}
