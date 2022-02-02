#[derive(Hash, Eq, PartialEq, Debug)]
pub enum Register {
  A,       // scratch registers
  B,
  C,
  D,
  F1,      // flag 1, 0x01 is the zero flag
}
