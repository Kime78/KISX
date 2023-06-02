use crate::cpu::Cpu;

type ExecuteFunc = fn(&Cpu);
pub struct Instruction {
    op: u32,
    exec: ExecuteFunc
}
impl Instruction {
    pub fn new(&self, op: u32, exec: ExecuteFunc) -> Instruction{
        Instruction { 
            op: 0,
            exec: nop
        }
    }

    pub fn execute(&self, cpu: &Cpu) {
        (self.exec)(cpu);
    }
}

fn nop(cpu: &Cpu) {

}