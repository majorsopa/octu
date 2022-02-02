pub use crate::octu_cpu::registers::Register;

#[derive(Debug)]
pub enum Instruction {
  Push(Value),
  Pop(Value),
  Mov(Register, Value),
  
  Jmp(u16),
  Jz(u16),
  Jnz(u16),
  Ret,
  
  Inc(Register),
  Dec(Register),
  Add(Register, Value),
  Sub(Register, Value),
  Mul(Register, Value),
  Div(Register, Value),
}

#[derive(Debug)]
pub enum Value {
  Register(Register),
  Literal(u8)
}
