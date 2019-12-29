use std::fs::File;
use std::io::prelude::*;

use super::instruction_type::InstructionType;

#[derive(Debug, Clone)]
pub
struct Program {
  pub instructions: Vec<InstructionType>,
}

#[derive(Debug)]
pub
struct ProgramContext {
  pub counter: usize,
  pub outputs: Vec<InstructionType>,
  pub inputs: Option<Vec<InstructionType>>,
  pub trace: bool,
}

impl Program {
  pub
  fn from_string(input_string: &str) -> Program {
    let lines = input_string.split(",");
    let mut intcodes = Vec::new();
    for mut line in lines {
      line = line.trim();
      if line.is_empty() {
        continue;
      }
      intcodes.push(line.parse::<i32>().unwrap());
    }
    Program { instructions: intcodes }
  }

  pub
  fn from_file(input_file: &str) -> Program {
    let mut file = File::open(input_file).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    Program::from_string(&content)
  }

  pub
  fn to_string(self) -> String {
    let mut output = String::new();
    for instruction in self.instructions {
      output += &format!("{},", instruction);
    }
    if output.ends_with(",") {
      output.pop();
    }
    output
  }
}

#[cfg(test)]
mod run_intcode_program_tests {
  use super::*;

  #[test]
  fn program_to_and_from_string() {
    let program = Program::from_string("1,2,3");
    assert_eq!(program.to_string(), "1,2,3");
  }
}

