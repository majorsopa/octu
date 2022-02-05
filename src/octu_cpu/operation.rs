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

  pub fn if_constant_get_name(&self) -> Option<String> {
    if self.instruction == Instruction::SetConstant {
      match &self.lhs {
        Some(val) => match val {
          Value::Literal(_) => None,
          Value::Register(_) => None,
          Value::Interrupt(_) => None,
          Value::Constant(name) => Some(name.to_string()),
        }
        None => None,
      }
    } else {
      None
    }
  }

  pub fn is_set_constant(&self) -> bool {
    self.instruction == Instruction::SetConstant
  }
}

#[derive(Debug)]
pub enum Value {
  Literal(Literal),
  Register(Register),
  Interrupt(Interrupt),
  Constant(String),
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
