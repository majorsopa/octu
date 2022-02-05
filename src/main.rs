use std::fs;
use std::collections::HashMap;

use octu::octu_mem::Instruction;
use octu::octu_cpu::*;
use octu::OctuVM;


const MAIN_FILE: &str = "main.octu";

fn main() {
  let octu_main = fs::read_to_string(MAIN_FILE).expect(&*format!("cannot find file {}", MAIN_FILE));

  let mut octu_vm = OctuVM::new_default_memory({
    let parsed = parse(octu_main);
    process(&parsed);
    parsed
  });

  let exit_code = octu_vm.run();
  
  println!("OctuVM finished with exit code {}", exit_code);
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
  let binary_instructions = HashMap::from([
    ("mov", Instruction::Mov),

    ("add", Instruction::Add),
    ("sub", Instruction::Sub),
    ("mul", Instruction::Mul),
    ("div", Instruction::Div),
  ]);
  let unary_instructions = HashMap::from([
    ("push", Instruction::Push),
    ("pop", Instruction::Pop),

    ("jmp", Instruction::Jmp),
    ("jz", Instruction::Jz),
    ("jnz", Instruction::Jnz),

    ("inc", Instruction::Inc),
    ("dec", Instruction::Dec),
  ]);
  let solo_instructions = HashMap::from([
    ("int", Instruction::Int),

    ("ret", Instruction::Ret),
  ]);
  let interrupts = HashMap::from([
    ("print", Interrupt::Print)
  ]);
  
  let octu_asm_main = "main";
  let octu_asm_constants = "constants";
  let u8_keyword = "u8";
  let i8_keyword = "i8";
  
  
  let mut operation_vec = Vec::new();
  let mut lexeme = String::new();
  let mut instruction = None;
  let mut lhs = None;
  let mut rhs = None;
  let mut instruction_type = None;
  let mut constant_name = None;
  let mut constant_value = None;
  let mut need_constant_value = false;
  let mut ready_for_operation_counter = 0;

  let mut in_comment = false;
  let mut next_is_uint = false;
  let mut next_is_iint = false;
  let mut in_string = false;
  let mut main_found = false;
  let mut constants_found = false;
  let mut main_first = true;
  for character in contents.chars() {

    if character == ';' {
      in_comment = !in_comment;
      continue;
    } else if character == '\r' || character == '\n' {
      in_comment = false;
    } else if character == '\"' {
      if constants_found && !(main_found ^ main_first) && need_constant_value {
        in_string = !in_string;
        continue;
      } else if !in_comment { panic!("unexpected string"); }
    }

    if in_comment { continue; }

    if lexeme == i8_keyword {
      next_is_iint = true;
      lexeme = "".to_string();
      continue;
    } else if lexeme == u8_keyword {
      next_is_uint = true;
      lexeme = "".to_string();
      continue;
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
      } else if &*lexeme == u8_keyword {
        
      }
      if lexeme == "" { continue; }
      
      
      if main_found && !constants_found && main_first {
        if {
          solo_instructions.contains_key(&*lexeme) ||
          unary_instructions.contains_key(&*lexeme) ||
          binary_instructions.contains_key(&*lexeme)
        } {
          if instruction.is_some() {
            panic!("syntax error");
          } else {
            instruction = Some(*{
              ready_for_operation_counter += 1;
              if solo_instructions.contains_key(&*lexeme) {
                instruction_type = Some(InstructionType::Solo);
                solo_instructions.get(&*lexeme)
              } else if unary_instructions.contains_key(&*lexeme) {
                instruction_type = Some(InstructionType::Unary);
                unary_instructions.get(&*lexeme)
              } else {
                instruction_type = Some(InstructionType::Binary);
                binary_instructions.get(&*lexeme)
              }
            }.unwrap());
            lexeme = "".to_string();
          }
        } else if instruction.is_some() {
          ready_for_operation_counter += 1;
          let hs = || {
            Some({
              if registers_table.contains_key(&*lexeme) {
                Value::Register(*registers_table.get(&*lexeme).unwrap())
              } else if interrupts.contains_key(&*lexeme) {
                Value::Interrupt(*interrupts.get(&*lexeme).unwrap())
              } else {
                Value::Constant(lexeme)
              }
            })
          };
          if lhs.is_none() {
            lhs = hs();
            lexeme = "".to_string();
          } else if rhs.is_none() {
            rhs = hs();
            lexeme = "".to_string();
          } else {
            panic!("syntax error");
          }
        } else {
          panic!("syntax error");
        }
      } else if constants_found {
        if constant_name.is_none() {
          assert!(!{
            solo_instructions.contains_key(&*lexeme) ||
            unary_instructions.contains_key(&*lexeme) ||
            binary_instructions.contains_key(&*lexeme) ||
            registers_table.contains_key(&*lexeme) ||
            interrupts.contains_key(&*lexeme)
          }, "constant name `{}` is a taken keyword", lexeme);
          
          constant_name = Some(Value::Constant(lexeme));
          lexeme = "".to_string();
          need_constant_value = true;
        } else {
          constant_value = Some(Value::Literal(if next_is_uint {
            next_is_uint = false;
            Literal::UInt(lexeme.parse::<u8>().expect(&*format!("invalid u8 `{}`", lexeme)))
          } else if next_is_iint {
            next_is_iint = false;
            Literal::IInt(lexeme.parse::<i8>().expect(&*format!("invalid i8 `{}`", lexeme)))
          } else {
            Literal::Str(lexeme)
          }));
          lexeme = "".to_string();
          need_constant_value = false;
        }
      }
    }

    if {
      (ready_for_operation_counter == 1 && instruction_type == Some(InstructionType::Solo)) ||
      (ready_for_operation_counter == 2 && instruction_type == Some(InstructionType::Unary)) ||
      (ready_for_operation_counter == 3 && instruction_type == Some(InstructionType::Binary))
    } {
      operation_vec.push(
        Operation::new(
          instruction_type.unwrap(), 
          instruction.unwrap(), 
          lhs, 
          rhs
        )
      );
      (instruction_type, instruction, lhs, rhs) = (None, None, None, None);
      ready_for_operation_counter = 0;
    }
    if constant_name.is_some() && constant_value.is_some() {
      operation_vec.push(
        Operation::new(InstructionType::Binary, Instruction::SetConstant, constant_name, constant_value)
      );
      (constant_name, constant_value) = (None, None);
    }
  }

  assert!(!{
    instruction.is_some() || 
    rhs.is_some() || 
    lhs.is_some() || 
    instruction_type.is_some() ||
    constant_name.is_some() || 
    constant_value.is_some()
  }, "syntax error");

  println!("{:#?}", operation_vec);
  operation_vec
}

fn process(operations: &Vec<Operation>) {
  let mut defined_constants = Vec::new();
  let mut used_constants = Vec::new();
  for operation in operations {
    let possible_name = operation.if_constant_get_name();
    if possible_name.is_some() {
      if operation.is_set_constant() {
        defined_constants.push(possible_name.unwrap());
      } else {
        used_constants.push(possible_name.unwrap());
      }
    }
  }
  
  let dedup = |slice: &Vec<String>| {
    (1..slice.len()).any(|i| slice[i..].contains(&slice[i - 1]))
  };

  // maybe take advantage of how it's already sorted to do the above faster, if I feel like it
  defined_constants.sort_unstable();
  used_constants.sort_unstable();
  
  assert!(!dedup(&defined_constants), "duplicated constant declaration");

  for constant in used_constants {
    assert!(defined_constants.contains(&constant), "unknown constant `{}`", constant);
  }
}
