pub mod octu_cpu;
mod octu_mem;

pub use crate::octu_cpu::OctuCPU;
pub use crate::octu_mem::OctuMem;


#[derive(Debug)]
pub struct OctuVM {
  octu_cpu: OctuCPU,
  octu_mem: OctuMem,
}

impl OctuVM {
  pub fn with_mem_size(length_in_bytes: usize) -> Self {
    Self {
      octu_mem: OctuMem::new(length_in_bytes),
      ..Default::default()
    }
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
