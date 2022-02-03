pub use crate::octu_cpu::registers::Register;

#[derive(Debug)]
pub enum Instruction {
  Push,
  Pop,
  Mov,
  
  Jmp,
  Jz,
  Jnz,
  Ret,
  Int,
  
  Inc,
  Dec,
  Add,
  Sub,
  Mul,
  Div,
}
