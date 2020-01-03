use super::instruction_type::InstructionType;
use super::program::Program;
use super::program::ProgramInstance;
use super::program::StepError;

pub
fn run_intcode_program(
  mut program: &mut Program,
  inputs: &Option<Vec<InstructionType>>,
) -> Vec<InstructionType>
{
  run_intcode_program_optional_trace(&mut program, inputs, false)
}

pub
fn trace_intcode_program(
  mut program: &mut Program,
  inputs: &Option<Vec<InstructionType>>,
) -> Vec<InstructionType>
{
  run_intcode_program_optional_trace(&mut program, inputs, true)
}

fn run_intcode_program_optional_trace(
  mut program: &mut Program,
  inputs: &Option<Vec<InstructionType>>,
  trace: bool,
) -> Vec<InstructionType>
{
  let mut program_instance = ProgramInstance::from_ref(&mut program);
  *program_instance.context.borrow_mut().inputs.borrow_mut() = inputs.clone();
  program_instance.context.borrow_mut().trace = trace;
  let mut outputs = Vec::new();
  loop {
    let mut next_counter;
    match program_instance.step() {
      Ok((counter, x)) => {
        next_counter = counter;
        match x {
          Some(output) => outputs.push(output),
          None => continue,
        }
      },
      Err(x) => match x {
        StepError::NeedInput => panic!("unexpected request for input and given inputs exhausted"),
        StepError::EndOfProgram => break,
        StepError::Error(msg) => panic!("unexpected error: {}", msg),
      }
    };
    program_instance.context.borrow_mut().counter.set(next_counter);
  }
  outputs
}

#[cfg(test)]
mod run_intcode_program_tests {
  use super::*;

  #[test]
  fn empty_program() {
    let program = Program { instructions: vec![] };
    let outputs = run_intcode_program(&mut program.clone(), &mut Some(Vec::new()));
    assert!(outputs.is_empty());
  }

  #[test]
  fn simple_halting_program() {
    let program = Program { instructions: vec![99] };
    let outputs = run_intcode_program(&mut program.clone(), &mut Some(Vec::new()));
    assert!(outputs.is_empty());
  }

  #[test]
  fn aoc_day02_examples() {
    {
      let mut program = Program::from_string("1,0,0,0,99");
      run_intcode_program(&mut program, &mut Some(Vec::new()));
      assert!(program.to_string().starts_with("2,"));
    }
    {
      let mut program = Program::from_string("2,3,0,3,99");
      run_intcode_program(&mut program, &mut Some(Vec::new()));
      assert!(program.to_string().starts_with("2,3,0,6,"));
    }
    {
      let mut program = Program::from_string("2,4,4,5,99,0");
      run_intcode_program(&mut program, &mut Some(Vec::new()));
      assert!(program.to_string().starts_with("2,4,4,5,99,9801"));
    }
    {
      let mut program = Program::from_string("1,1,1,4,99,5,6,0,99");
      run_intcode_program(&mut program, &mut Some(Vec::new()));
      assert!(program.to_string().starts_with("30,"));
    }
    {
      let mut program = Program::from_string("
1,12,2,3,1,1,2,3,1,3,4,3,1,5,0,3,2,13,1,19,1,19,6,23,1,23,6,27,1,13,27,31,2,13,
31,35,1,5,35,39,2,39,13,43,1,10,43,47,2,13,47,51,1,6,51,55,2,55,13,59,1,59,10,
63,1,63,10,67,2,10,67,71,1,6,71,75,1,10,75,79,1,79,9,83,2,83,6,87,2,87,9,91,1,
5,91,95,1,6,95,99,1,99,9,103,2,10,103,107,1,107,6,111,2,9,111,115,1,5,115,119,
1,10,119,123,1,2,123,127,1,127,6,0,99,2,14,0,0");
      run_intcode_program(&mut program, &mut Some(Vec::new()));
      assert!(program.to_string().starts_with("12490719,"));
    }
  }

  #[test]
  fn aoc_day05_examples() {
    let problem_input_program = Program::from_string("
3,225,1,225,6,6,1100,1,238,225,104,0,1102,46,47,225,2,122,130,224,101,-1998,
224,224,4,224,1002,223,8,223,1001,224,6,224,1,224,223,223,1102,61,51,225,102,
32,92,224,101,-800,224,224,4,224,1002,223,8,223,1001,224,1,224,1,223,224,223,
1101,61,64,225,1001,118,25,224,101,-106,224,224,4,224,1002,223,8,223,101,1,224,
224,1,224,223,223,1102,33,25,225,1102,73,67,224,101,-4891,224,224,4,224,1002,
223,8,223,1001,224,4,224,1,224,223,223,1101,14,81,225,1102,17,74,225,1102,52,
67,225,1101,94,27,225,101,71,39,224,101,-132,224,224,4,224,1002,223,8,223,101,
5,224,224,1,224,223,223,1002,14,38,224,101,-1786,224,224,4,224,102,8,223,223,
1001,224,2,224,1,223,224,223,1,65,126,224,1001,224,-128,224,4,224,1002,223,8,
223,101,6,224,224,1,224,223,223,1101,81,40,224,1001,224,-121,224,4,224,102,8,
223,223,101,4,224,224,1,223,224,223,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,
1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,
1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,
1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,
300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,1008,677,226,
224,1002,223,2,223,1005,224,329,1001,223,1,223,107,677,677,224,102,2,223,223,
1005,224,344,101,1,223,223,1107,677,677,224,102,2,223,223,1005,224,359,1001,
223,1,223,1108,226,226,224,1002,223,2,223,1006,224,374,101,1,223,223,107,226,
226,224,1002,223,2,223,1005,224,389,1001,223,1,223,108,226,226,224,1002,223,2,
223,1005,224,404,1001,223,1,223,1008,677,677,224,1002,223,2,223,1006,224,419,
1001,223,1,223,1107,677,226,224,102,2,223,223,1005,224,434,1001,223,1,223,108,
226,677,224,102,2,223,223,1006,224,449,1001,223,1,223,8,677,226,224,102,2,223,
223,1006,224,464,1001,223,1,223,1007,677,226,224,1002,223,2,223,1006,224,479,
1001,223,1,223,1007,677,677,224,1002,223,2,223,1005,224,494,1001,223,1,223,
1107,226,677,224,1002,223,2,223,1006,224,509,101,1,223,223,1108,226,677,224,
102,2,223,223,1005,224,524,1001,223,1,223,7,226,226,224,102,2,223,223,1005,
224,539,1001,223,1,223,8,677,677,224,1002,223,2,223,1005,224,554,101,1,223,
223,107,677,226,224,102,2,223,223,1006,224,569,1001,223,1,223,7,226,677,224,
1002,223,2,223,1005,224,584,1001,223,1,223,1008,226,226,224,1002,223,2,223,
1006,224,599,101,1,223,223,1108,677,226,224,102,2,223,223,1006,224,614,101,1,
223,223,7,677,226,224,102,2,223,223,1005,224,629,1001,223,1,223,8,226,677,
224,1002,223,2,223,1006,224,644,101,1,223,223,1007,226,226,224,102,2,223,223,
1005,224,659,101,1,223,223,108,677,677,224,1002,223,2,223,1006,224,674,1001,
223,1,223,4,223,99,226
");
    {
      let mut program = problem_input_program.clone();
      let outputs = run_intcode_program(&mut program, &mut Some(vec![1]));
      assert!(outputs.len() > 0);
      assert_eq!(*(outputs.last().unwrap()), 12896948_i32);
    }
    {
      let program = Program::from_string("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");
      {
        let outputs = run_intcode_program(&mut program.clone(), &mut Some(vec![0]));
        assert!(outputs.len() == 1);
        assert_eq!(outputs[0], 0_i32);
      }
      {
        let outputs = run_intcode_program(&mut program.clone(), &mut Some(vec![1]));
        assert!(outputs.len() == 1);
        assert_eq!(outputs[0], 1_i32);
      }
    }
    {
      let program = Program::from_string("3,3,1105,-1,9,1101,0,0,12,4,12,99,1");
      {
        let outputs = run_intcode_program(&mut program.clone(), &mut Some(vec![0]));
        assert!(outputs.len() == 1);
        assert_eq!(outputs[0], 0_i32);
      }
      {
        let outputs = run_intcode_program(&mut program.clone(), &mut Some(vec![1]));
        assert!(outputs.len() == 1);
        assert_eq!(outputs[0], 1_i32);
      }
    }
    {
      let program = Program::from_string("
3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99
");
      {
        let outputs = trace_intcode_program(&mut program.clone(), &mut Some(vec![7]));
        assert!(outputs.len() == 1);
        assert_eq!(outputs[0], 999_i32);
      }
      {
        let outputs = trace_intcode_program(&mut program.clone(), &mut Some(vec![8]));
        assert!(outputs.len() == 1);
        assert_eq!(outputs[0], 1000_i32);
      }
      {
        let outputs = trace_intcode_program(&mut program.clone(), &mut Some(vec![9]));
        assert!(outputs.len() == 1);
        assert_eq!(outputs[0], 1001_i32);
      }
    }
    {
      let program = problem_input_program.clone();
      let outputs = trace_intcode_program(&mut program.clone(), &mut Some(vec![5]));
      assert!(outputs.len() == 1);
      assert_eq!(outputs[0], 7704130);
    }
  }
}
