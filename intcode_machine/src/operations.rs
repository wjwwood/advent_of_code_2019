use std::io;

use phf::phf_map;

use super::instruction_type::InstructionType;
use super::operation::Operation;
use super::operation::validate_operation;
use super::operation_instance::OperationInstance;
use super::parameter_mode::ParameterMode;
use super::program::ProgramContext;
use super::program::StepError;

pub
static OPERATIONS: phf::Map<InstructionType, Operation> = phf_map! {
  01i32 => Operation {
    name: "ADD",
    size: 4,
    execute: |program: &mut Vec<i32>, program_context: &ProgramContext, op_instance: &OperationInstance| -> Result<(usize, Option<InstructionType>), StepError> {
      let size = validate_operation(program, program_context.counter.get(), op_instance);
      let op1 = get_parameter_value(program, program_context.counter.get(), op_instance, 1);
      let op2 = get_parameter_value(program, program_context.counter.get(), op_instance, 2);
      assert!(op_instance.parameter3_mode != ParameterMode::ImmediateMode);
      let dst = program[program_context.counter.get() + 3];
      program[dst as usize] = op1 + op2;
      if program_context.trace {
        println!("  ADD: added '{}' + '{}' = '{}', stored in '{}'", op1, op2, op1 + op2, dst);
      }
      Ok((program_context.counter.get() + size, None))
    },
  },
  02i32 => Operation {
    name: "MULTIPLY",
    size: 4,
    execute: |program: &mut Vec<i32>, program_context: &ProgramContext, op_instance: &OperationInstance| -> Result<(usize, Option<InstructionType>), StepError> {
      let size = validate_operation(program, program_context.counter.get(), op_instance);
      let op1 = get_parameter_value(program, program_context.counter.get(), op_instance, 1);
      let op2 = get_parameter_value(program, program_context.counter.get(), op_instance, 2);
      assert!(op_instance.parameter3_mode != ParameterMode::ImmediateMode);
      let dst = program[program_context.counter.get() + 3];
      program[dst as usize] = op1 * op2;
      if program_context.trace {
        println!("  MULTIPLY: added '{}' * '{}' = '{}', stored in '{}'", op1, op2, op1 * op2, dst);
      }
      Ok((program_context.counter.get() + size, None))
    },
  },
  03i32 => Operation {
    name: "INPUT",
    size: 2,
    execute: |program: &mut Vec<i32>, program_context: &ProgramContext, op_instance: &OperationInstance| -> Result<(usize, Option<InstructionType>), StepError> {
      let size = validate_operation(program, program_context.counter.get(), op_instance);
      assert!(op_instance.parameter1_mode != ParameterMode::ImmediateMode);
      let dst = program[program_context.counter.get() + 1];
      let input: i32 = match &mut *program_context.inputs.borrow_mut() {
        Some(i) => {
          if i.is_empty() {
            return Err(StepError::NeedInput)
          }
          i.remove(0)
        },
        None => {
          let mut input = String::new();
          io::stdin().read_line(&mut input).unwrap();
          input = input.trim().to_string();
          input.parse::<i32>().unwrap()
        },
      };
      program[dst as usize] = input;
      if program_context.trace {
        println!("  INPUT: got '{}' and stored it at '{}'", input, dst);
      }
      Ok((program_context.counter.get() + size, None))
    },
  },
  04i32 => Operation {
    name: "PRINT",
    size: 2,
    execute: |program: &mut Vec<i32>, program_context: &ProgramContext, op_instance: &OperationInstance| -> Result<(usize, Option<InstructionType>), StepError> {
      let size = validate_operation(program, program_context.counter.get(), op_instance);
      let value_to_print = get_parameter_value(program, program_context.counter.get(), op_instance, 1);
      if program_context.trace {
        println!("  PRINT: printing '{}'", value_to_print);
      }
      Ok((program_context.counter.get() + size, Some(value_to_print)))
    },
  },
  05i32 => Operation {
    name: "JUMP-IF-TRUE",
    size: 3,
    execute: |program: &mut Vec<i32>, program_context: &ProgramContext, op_instance: &OperationInstance| -> Result<(usize, Option<InstructionType>), StepError> {
      let size = validate_operation(program, program_context.counter.get(), op_instance);
      let test = get_parameter_value(program, program_context.counter.get(), op_instance, 1);
      let dst_raw = get_parameter_value(program, program_context.counter.get(), op_instance, 2);
      if program_context.trace {
        println!("  JUMP-IF-TRUE: jumping to '{}' if '{}' is not '0': {}", dst_raw, test, test != 0);
      }
      if test != 0 {
        let dst: usize = validate_jump_destination(dst_raw, &program);
        return Ok((dst, None))
      }
      Ok((program_context.counter.get() + size, None))
    },
  },
  06i32 => Operation {
    name: "JUMP-IF-FALSE",
    size: 3,
    execute: |program: &mut Vec<i32>, program_context: &ProgramContext, op_instance: &OperationInstance| -> Result<(usize, Option<InstructionType>), StepError> {
      let size = validate_operation(program, program_context.counter.get(), op_instance);
      let test = get_parameter_value(program, program_context.counter.get(), op_instance, 1);
      let dst_raw = get_parameter_value(program, program_context.counter.get(), op_instance, 2);
      if program_context.trace {
        println!("  JUMP-IF-FALSE: jumping to '{}' if '{}' is '0': {}", dst_raw, test, test == 0);
      }
      if test == 0 {
        let dst: usize = validate_jump_destination(dst_raw, &program);
        return Ok((dst, None))
      }
      Ok((program_context.counter.get() + size, None))
    },
  },
  07i32 => Operation {
    name: "LESS-THAN",
    size: 4,
    execute: |program: &mut Vec<i32>, program_context: &ProgramContext, op_instance: &OperationInstance| -> Result<(usize, Option<InstructionType>), StepError> {
      let size = validate_operation(program, program_context.counter.get(), op_instance);
      let op1 = get_parameter_value(program, program_context.counter.get(), op_instance, 1);
      let op2 = get_parameter_value(program, program_context.counter.get(), op_instance, 2);
      assert!(op_instance.parameter3_mode != ParameterMode::ImmediateMode);
      let dst = program[program_context.counter.get() + 3];
      if op1 < op2 {
        program[dst as usize] = 1;
      } else {
        program[dst as usize] = 0;
      }
      if program_context.trace {
        println!("  LESS-THAN: assigning '1' to '{}' if '{}' < '{}': {}", dst, op1, op2, op1 < op2);
      }
      Ok((program_context.counter.get() + size, None))
    },
  },
  08i32 => Operation {
    name: "EQUALS",
    size: 4,
    execute: |program: &mut Vec<i32>, program_context: &ProgramContext, op_instance: &OperationInstance| -> Result<(usize, Option<InstructionType>), StepError> {
      let size = validate_operation(program, program_context.counter.get(), op_instance);
      let op1 = get_parameter_value(program, program_context.counter.get(), op_instance, 1);
      let op2 = get_parameter_value(program, program_context.counter.get(), op_instance, 2);
      assert!(op_instance.parameter3_mode != ParameterMode::ImmediateMode);
      let dst = program[program_context.counter.get() + 3];
      if op1 == op2 {
        program[dst as usize] = 1;
      } else {
        program[dst as usize] = 0;
      }
      if program_context.trace {
        println!("  EQUALS: assigning '1' to '{}' if '{}' == '{}': {}", dst, op1, op2, op1 == op2);
      }
      Ok((program_context.counter.get() + size, None))
    },
  },
  99i32 => Operation {
    name: "HALT",
    size: 1,
    execute: |program: &mut Vec<i32>, program_context: &ProgramContext, op_instance: &OperationInstance| -> Result<(usize, Option<InstructionType>), StepError> {
      if program_context.trace {
        println!("  HALT");
      }
      Err(StepError::EndOfProgram)
    },
  },
};

fn get_parameter_value(program: &Vec<i32>, counter: usize, op_instance: &OperationInstance, parameter_index: usize) -> i32 {
  let mode = match parameter_index {
    1 => {
      &op_instance.parameter1_mode
    }
    2 => {
      &op_instance.parameter2_mode
    }
    3 => {
      &op_instance.parameter3_mode
    }
    _ => {
      panic!("expected a parameter index of 1, 2, or 3, got '{}'", parameter_index);
    }
  };
  if *mode == ParameterMode::PositionMode {
    let address = program[counter + parameter_index] as usize;
    if address > program.len() - 1 {
      panic!("address out of bounds");
    }
    return program[address]
  } else {
    return program[counter + parameter_index]
  }
}

fn validate_jump_destination(jump_destination: i32, program: & Vec<i32>,) -> usize {
  if jump_destination < 0 {
    panic!("invalid jump operation, negative destination '{}'", jump_destination);
  }
  if jump_destination as usize > program.len() - 1 {
    panic!(
      "invalid jump operation, destination '{}' out of range for program of length '{}'",
      jump_destination,
      program.len());
  }
  jump_destination as usize
}
