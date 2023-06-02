use interconnect::Interconnect;

use crate::{interconnect, instruction::Instruction};
pub struct Cpu {
    pc: u32,
    inter: Interconnect,
}

impl Cpu {
    pub fn new(inter: Interconnect) -> Cpu {
        Cpu {
            pc: 0xbfc00000,
            inter: inter,
        }
    }

    pub fn fetch_instruction(&mut self) -> u32 { //to do return type of Instruction
        let pc = self.pc;
        let instruction = self.load32(pc);
        self.pc = pc.wrapping_add(4);
        return instruction;
    }

    pub fn load32(&self, addr: u32) -> u32 {
        self.inter.load32(addr)
    }

    pub fn cycle(&self) {
        self.load32(self.pc).decode().execute(self);
    }
}
trait Decode {
    fn decode(&self) -> Instruction;
}

impl Decode for u32 {
    fn decode(&self) -> Instruction {
        match *self {
            _ => panic!("Unhandled Instruction {:x}", *self)
        }
    }
}