pub mod instructions;

use crate::octu_mem::instructions::Instruction;

pub struct OctuMem(Vec<u8>);

#[derive(Debug)]
impl OctuMem {
  pub fn new(length_in_bytes: usize) -> Self {
    let mut memory_vec = Vec::new();
    for _ in 0..length_in_bytes {
      memory_vec.push(0);
    }
    Self(memory_vec)
  }

  pub fn get_byte(&self, address: u16) -> u8 {
    *self.0.get(address as usize).unwrap()
  }

  pub fn get_byte_mut(&mut self, address: u16) -> &mut u8 {
    self.0.get_mut(address as usize).unwrap()
  }

  pub fn set_byte(&mut self, address: u16, new_value: u8) {
    *self.0.get_mut(address as usize).unwrap() = new_value;
  }
}

impl Default for OctuMem {
  fn default() -> Self {
    Self::new(65535)
  }
}
