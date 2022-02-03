use std::fs;
use std::collections::HashMap;

use octu::octu_mem::instructions::Instruction;
use octu::octu_cpu::registers::Register;


const MAIN_FILE: &str = "main.octu";

fn main() {
  let octu_main = fs::read_to_string(MAIN_FILE).expect(&*format!("cannot find file {}", MAIN_FILE));

  parse(octu_main);
}


#[derive(Debug)]
struct Operation {
  instruction: Instruction,
  lhs: Value,
  rhs: Value,
}

impl Operation {
  pub fn new(instruction: Instruction, lhs: Value, rhs: Value) -> Self {
    Self {
      instruction,
      lhs,
      rhs,
    }
  }
}

#[derive(Debug)]
enum Value {
  Literal(Literal),
  Register(Register),
}

#[derive(Debug)]
enum Literal {
  Str(String),
  Int(u8),
  // todo add floating point stuff
}

fn parse(contents: String) -> Vec<Operation> {
  let registers_table = HashMap::from([
    ("a", Register::A),
    ("b", Register::B),
    ("c", Register::C),
    ("d", Register::D),
    ("f1", Register::F1),
  ]);
  let instructions_table = HashMap::from([
    ("push", Instruction::Push),
    ("pop", Instruction::Pop),
    ("mov", Instruction::Mov),
    
    ("jmp", Instruction::Jmp),
    ("jz", Instruction::Jz),
    ("jnz", Instruction::Jnz),
    ("ret", Instruction::Ret),
    ("int", Instruction::Int),

    ("inc", Instruction::Inc),
    ("dec", Instruction::Dec),
    ("add", Instruction::Add),
    ("sub", Instruction::Sub),
    ("mul", Instruction::Mul),
    ("div", Instruction::Div),
  ]);
  
  let octu_asm_main = "main";
  let octu_asm_constants = "constants";
  
  
  let mut instructions_vec = Vec::new();
  //let mut operation_vec = Vec::new();
  let mut string_literals = String::new();
  let mut lexeme = String::new();
  let mut instruction = None;
  //let mut lhs = None;
  //let mut rhs = None;

  let mut in_string = false;
  let mut main_found = false;
  let mut constants_found = false;
  for character in contents.chars() {
    if character == '\"' {
      if in_string {
        in_string = false;
      } else {
        in_string = true;
      }
      continue;
    }
    if in_string {
      string_literals.push(character);
    } else if character != ' ' {
      lexeme.push(character);
    } else {
      if main_found && !constants_found {
        if instructions_table.contains_key(&*lexeme) {
          if instruction.is_some() {
            panic!("syntax error");
          } else {
            instruction = Some(instructions_table.get(&*lexeme).unwrap());
            lexeme = "".to_string();
          }
        } else {
          panic!("unknown lexeme");
        }
      } else if !main_found && constants_found {
        //constants parsing
      }
      
      
      if &*lexeme == octu_asm_main {
        main_found = true;
        lexeme = "".to_string();
      } else if &*lexeme == octu_asm_constants {
        constants_found = true;
        lexeme = "".to_string();
      }
    }

    
  }
  

  instructions_vec
}
