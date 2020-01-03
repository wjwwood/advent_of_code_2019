use std::cell::Cell;
use std::cell::RefCell;
use std::fs::File;
use std::io::prelude::*;
use std::rc::Rc;

use super::execute_instruction::execute_instruction_at;
use super::instruction_type::InstructionType;

#[derive(Debug, Clone)]
pub
struct Program {
  pub instructions: Vec<InstructionType>,
}

#[derive(Debug)]
pub
struct ProgramContext {
  pub counter: Cell<usize>,
  pub inputs: RefCell<Option<Vec<InstructionType>>>,
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

#[derive(Debug)]
pub
struct ProgramInstance<'a> {
  owned_program: Option<Rc<Program>>,
  ref_program: Option<&'a mut Program>,
  pub context: RefCell<ProgramContext>,
}

#[derive(Debug)]
pub
enum StepError {
  NeedInput,
  EndOfProgram,
  Error(&'static str),
}

impl<'a> ProgramInstance<'a> {

  pub
  fn new(program: Program) -> ProgramInstance<'a> {
    let rc: Rc<Program> = Rc::new(program);
    ProgramInstance {
      owned_program: Some(rc.clone()),
      ref_program: None,
      context: RefCell::new(ProgramContext {
        counter: Cell::new(0),
        inputs: RefCell::new(None),
        trace: false,
      }),
    }
  }

  pub
  fn from_ref(program_ref: &'a mut Program) -> ProgramInstance<'a> {
    ProgramInstance {
      owned_program: None,
      ref_program: Some(program_ref),
      context: RefCell::new(ProgramContext {
        counter: Cell::new(0),
        inputs: RefCell::new(None),
        trace: false,
      }),
    }
  }

  pub
  fn get_program_mut(&'a mut self) -> &'a mut Program {
    match &mut self.owned_program {
      Some(x) => Rc::get_mut(x).unwrap(),
      None => {
        match &mut self.ref_program {
          Some(x) => x,
          None => panic!("neither owned nor ref program available"),
        }
      },
    }
  }

  pub
  fn get_program(&'a self) -> &'a Program {
    match & self.owned_program {
      Some(x) => &*x,
      None => {
        match & self.ref_program {
          Some(x) => x,
          None => panic!("neither owned nor ref program available"),
        }
      },
    }
  }

  fn program_has_ended(& self) -> bool {
    self.context.borrow().counter.get() >= self.get_program().instructions.len()
  }

  pub
  fn step(&'a mut self) -> Result<(usize, Option<InstructionType>), StepError> {
    if self.program_has_ended() {
      // reached the end of the program
      return Err(StepError::EndOfProgram)
    }
    execute_instruction_at(&mut self.get_program_mut(), &*self.context.borrow())
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

