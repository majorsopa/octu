pub mod octu_cpu;
pub mod octu_mem;

pub use crate::octu_cpu::{OctuCPU, Operation};
pub use crate::octu_mem::OctuMem;


#[derive(Debug)]
pub struct OctuVM {
  octu_cpu: OctuCPU,
  octu_mem: OctuMem,
}

impl OctuVM {
  pub fn new(mem_length: usize, stack: Vec<Operation>) -> Self {
    Self {
      octu_cpu: OctuCPU::from(stack),
      octu_mem: OctuMem::new(mem_length),
    }
  }

  pub fn new_default_memory(stack: Vec<Operation>) -> Self {
    Self {
      octu_cpu: OctuCPU::from(stack),
      ..Default::default()
    }
  }

  pub fn run(&mut self) -> u8 {
    while !self.octu_cpu.is_stack_empty() {
      self.octu_cpu.do_operation();
    }
    self.octu_mem.get_byte(0)  // the exit code
  }

  pub fn replace_stack(&mut self, stack: Vec<Operation>) {
    self.octu_cpu.replace_stack(stack);
  }
}

impl Default for OctuVM {
  fn default() -> Self {
    Self {
      octu_cpu: OctuCPU::default(),
      octu_mem: OctuMem::default(),
    }
  }
}
