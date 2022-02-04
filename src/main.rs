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
  // this is a hacky workaround but idc
  let mut contents = contents;
  contents = contents.trim().to_string();
  contents.push(' ');
  
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
  let mut operation_vec = Vec::new();
  let mut lexeme = String::new();
  let mut instruction = None;
  let mut lhs = None;
  let mut rhs = None;
  let mut constant_name = None;
  let mut constant_value = None;
  let mut need_constant_value = false;

  let mut in_string = false;
  let mut main_found = false;
  let mut constants_found = false;
  let mut main_first = true;
  for character in contents.chars() {

    if character == '\"' {
      if constants_found && !(main_found ^ main_first) && need_constant_value {
        if in_string {
          in_string = false;
        } else {
          in_string = true;
        }
        continue;
      } else { panic!("unexpected string"); }
    }
    
    if in_string {
      lexeme.push(character);
    } else if character != ' ' && character != '\r' && character != '\n' && character != '\t' {
      lexeme.push(character);
    } else {
      if lexeme == "" { continue; }
      if &*lexeme == octu_asm_main {
        main_found = true;
        lexeme = "".to_string();
      } else if &*lexeme == octu_asm_constants {
        constants_found = true;
        if !main_found { main_first = false; }
        lexeme = "".to_string();
      }
      if lexeme == "" { continue; }
      
      
      if main_found && !constants_found && main_first {
        if instructions_table.contains_key(&*lexeme) {
          if instruction.is_some() {
            panic!("syntax error");
          } else {
            instruction = Some(*instructions_table.get(&*lexeme).unwrap());
            lexeme = "".to_string();
          }
        } else if instruction.is_some() && registers_table.contains_key(&*lexeme) {
          if lhs.is_none() {
            lhs = Some(Value::Register(*registers_table.get(&*lexeme).unwrap()));
            lexeme = "".to_string();
          } else if rhs.is_none() {
            rhs = Some(Value::Register(*registers_table.get(&*lexeme).unwrap()));
            lexeme = "".to_string();
          } else {
            panic!("syntax error");
          }
        } else {
          panic!("unknown lexeme `{}`", lexeme);
        }
      } else if constants_found {
        if constant_name.is_none() {
          constant_name = Some(Value::Literal(Literal::Str(lexeme)));
          lexeme = "".to_string();
          need_constant_value = true;
        } else {
          constant_value = Some(Value::Literal(Literal::Str(lexeme)));
          lexeme = "".to_string();
          need_constant_value = false;
        }
      }

      
    }

    if instruction.is_some() && lhs.is_some() && rhs.is_some() {
      operation_vec.push(
        Operation::new(instruction.unwrap(), lhs.unwrap(), rhs.unwrap())
      );
      (instruction, lhs, rhs) = (None, None, None);
    }
    if constant_name.is_some() && constant_value.is_some() {
      operation_vec.push(
        Operation::new(Instruction::SetConstant, constant_name.unwrap(), constant_value.unwrap())
      );
      (constant_name, constant_value) = (None, None);
    }
  }
  
  println!("{:#?}", operation_vec);
  instructions_vec
}
