mod registers;
mod interrupts;
mod operation;

use std::collections::HashMap;

pub use crate::octu_cpu::registers::Register;
pub use crate::octu_cpu::interrupts::Interrupt;
pub use crate::octu_cpu::operation::*;

use bitlab::SingleBits;


#[derive(Debug)]
pub struct OctuCPU {
  stack: Vec<Operation>,
  registers: HashMap<Register, u8>,
  syscalls: HashMap<u8, Interrupt>,
}


// reading and setting registers
impl OctuCPU {
  pub fn get_register(&self, register: &Register) -> &u8 {
    self.registers.get(register).unwrap()
  }

  pub fn get_zf(&self) -> bool {
    { *self.get_register(&Register::F1) }.get_bit(7).unwrap()
  }

  
  pub fn set_register(&mut self, register: &Register, new_value: u8) {
    *self.registers.get_mut(register).unwrap() = new_value;
  }

  pub fn set_zf(&mut self, new_value: bool) {
    if new_value {
      { *self.registers.get_mut(&Register::F1).unwrap() }.set_bit(7).unwrap();
    } else {
      { *self.registers.get_mut(&Register::F1).unwrap() }.clear_bit(7).unwrap();
    }
  }
}

// stack stuff
impl OctuCPU {
  pub fn push(&mut self, to_push: Operation) {
    self.stack.push(to_push);
  }

  pub fn pop(&mut self) -> Operation {
    self.stack.pop().unwrap()
  }

  pub fn replace_stack(&mut self, new_stack: Vec<Operation>) {
    self.stack = new_stack;
  }

  pub fn is_stack_empty(&self) -> bool {
    self.stack.is_empty()
  }
}

// other stuff
impl OctuCPU {
  pub fn do_operation(&mut self) -> u8 {
    let exit_code = 0;

    exit_code
  }
}

impl Default for OctuCPU {
  fn default() -> Self {
    Self {
      stack: Vec::new(),
      registers: HashMap::from([
        (Register::A, 0),
        (Register::B, 0),
        (Register::C, 0),
        (Register::D, 0),
        (Register::F1, 0),
      ]),
      syscalls: HashMap::from([
        (0, Interrupt::Print)
      ]),
    }
  }
}

impl From<Vec<Operation>> for OctuCPU {
  fn from(stack: Vec<Operation>) -> Self {
    Self {
      stack,
      ..Default::default()
    }
  }
}
