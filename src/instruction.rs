use crate::cpu::Cpu;

type ExecuteFunc = fn(&mut Cpu, &Instruction);
pub struct Instruction {
    pub op: u32,
    exec: ExecuteFunc,
    pub function: u32,
    pub subfunction: u32,
    imm: u32,
    t: u32,
    s: u32,
    d: u32,
    imm_se: u32,
    shift: u32,
}
impl Instruction {
    pub fn new(op: u32) -> Instruction {
        Instruction {
            op,
            exec: nop,
            function: op >> 26,
            imm: op & 0xffff,
            t: (op >> 16) & 0x1f,
            s: (op >> 21) & 0x1f,
            d: (op >> 15) & 0x1f,
            subfunction: op & 0x3f,
            imm_se: ((op & 0xffff) as i16) as u32,
            shift: (op >> 6) & 0x1f,
        }
    }

    pub fn set_exec(&mut self, exec: ExecuteFunc) {
        self.exec = exec;
    }

    pub fn execute(&self, cpu: &mut Cpu) {
        (self.exec)(cpu, self);
    }
}

pub fn op_lui(cpu: &mut Cpu, instruction: &Instruction) {
    cpu.set_reg(instruction.t, instruction.t << 16);
}

pub fn op_ori(cpu: &mut Cpu, instruction: &Instruction) {
    cpu.set_reg(instruction.t, cpu.reg(instruction.s) | instruction.imm);
}

pub fn op_sw(cpu: &mut Cpu, instruction: &Instruction) {
    let addr = cpu.reg(instruction.s).wrapping_add(instruction.imm_se);
    let v = cpu.reg(instruction.t);

    cpu.store32(addr, v);
}

pub fn op_sll(cpu: &mut Cpu, instruction: &Instruction) {
    cpu.set_reg(instruction.d, cpu.reg(instruction.t) << instruction.shift);
}

fn nop(_cpu: &mut Cpu, _instruction: &Instruction) {}
