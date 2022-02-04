use crate::octu_cpu::{Interrupt, Register};
use crate::octu_mem::Instruction;

#[derive(Debug)]
pub struct Operation {
  instruction_type: InstructionType,
  instruction: Instruction,
  lhs: Option<Value>,
  rhs: Option<Value>,
}

impl Operation {
  pub fn new(instruction_type: InstructionType, instruction: Instruction, lhs: Option<Value>, rhs: Option<Value>) -> Self {
    Self {
      instruction_type,
      instruction,
      lhs,
      rhs,
    }
  }
}

#[derive(Debug)]
pub enum Value {
  Literal(Literal),
  Register(Register),
  Interrupt(Interrupt),
}

#[derive(Debug)]
pub enum Literal {
  Str(String),
  UInt(u8),
  IInt(i8),
  // todo add floating point stuff
}

#[derive(Debug, PartialEq)]
pub enum InstructionType {
  Solo,
  Unary,
  Binary,
}
