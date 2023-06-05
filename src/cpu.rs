use interconnect::Interconnect;

use crate::{instruction::*, interconnect};
pub struct Cpu {
    pc: u32,
    inter: Interconnect,
    regs: [u32; 32],
}

impl Cpu {
    pub fn new(inter: Interconnect) -> Cpu {
        let mut regs = [0x0; 32];
        Cpu {
            pc: 0xbfc00000,
            inter: inter,
            regs: regs,
        }
    }

    pub fn fetch_instruction(&mut self) -> u32 {
        //to do return type of Instruction
        let pc = self.pc;
        let instruction = self.load32(pc);
        self.pc = pc.wrapping_add(4);
        return instruction;
    }

    pub fn load32(&self, addr: u32) -> u32 {
        self.inter.load32(addr)
    }

    pub fn cycle(&mut self) {
        self.load32(self.pc).decode().execute(self);
        self.pc = self.pc.wrapping_add(4);
    }

    pub fn reg(&self, index: u32) -> u32 {
        self.regs[index as usize]
    }

    pub fn set_reg(&mut self, index: u32, val: u32) {
        self.regs[index as usize] = val;

        self.regs[0] = 0;
    }
}
trait Decode {
    fn decode(self) -> Instruction;
}

impl Decode for u32 {
    fn decode(self) -> Instruction {
        let mut instr = Instruction::new(self);
        match instr.function {
            0b001111 => instr.set_exec(op_lui),
            0b001101 => instr.set_exec(op_ori),
            _ => panic!(
                "Unhandled Instruction {:x} with func {:b}",
                self, instr.function
            ),
        }

        instr
    }
}
