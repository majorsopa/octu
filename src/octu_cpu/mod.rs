pub mod registers;
pub mod interrupts;

use std::collections::HashMap;

use crate::octu_cpu::registers::Register;
use crate::octu_cpu::interrupts::Interrupt;

use bitlab::SingleBits;


#[derive(Debug)]
pub struct OctuCPU {
  stack: Vec<u8>,
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
  pub fn push(&mut self, to_push: u8) {
    self.stack.push(to_push);
  }

  pub fn pop(&mut self) -> u8 {
    self.stack.pop().unwrap()
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
