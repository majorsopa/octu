#[derive(Debug, Copy, Clone)]
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

  SetConstant,
}
