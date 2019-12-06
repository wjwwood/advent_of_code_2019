use std::fs::File;
use std::io::prelude::*;
use std::io;
use phf::phf_map;
use std::convert::TryFrom;

fn main() {
  // let mut program: Vec<i32> = load_program_from_comma_delimited_file("program1.csv");
  let mut program: Vec<i32> = load_program_from_comma_delimited_file("program2.csv");
  run_program(&mut program);
}

#[derive(PartialEq, Debug)]
enum ParameterMode {
  PositionMode,
  ImmediateMode,
}

struct Operation {
  name: &'static str,
  size: usize,
  execute: fn(program: &mut Vec<i32>, counter: &mut usize, op_instance: &OperationInstance) -> usize,
}

#[derive(Debug)]
struct OperationInstance {
  opcode: u32,
  parameter1_mode: ParameterMode,
  parameter2_mode: ParameterMode,
  parameter3_mode: ParameterMode,
}

fn parse_operation_intcode(operation_intcode: u32) -> OperationInstance {
  let digits = split_into_five_digits_right_to_left(operation_intcode);
  if !(0..=1).contains(&digits[2]) {
    panic!(
      "expected parameter mode for parameter 1 to be 0 or 1, got '{}' from whole intcode '{}'",
      digits[2],
      operation_intcode);
  }
  if !(0..=1).contains(&digits[3]) {
    panic!(
      "expected parameter mode for parameter 2 to be 0 or 1, got '{}' from whole intcode '{}'",
      digits[3],
      operation_intcode);
  }
  if !(0..=1).contains(&digits[4]) {
    panic!(
      "expected parameter mode for parameter 3 to be 0 or 1, got '{}' from whole intcode '{}'",
      digits[4],
      operation_intcode);
  }
  OperationInstance {
    opcode: digits[0] + (digits[1] * 10),
    parameter1_mode: if digits[2] == 0 { ParameterMode::PositionMode } else { ParameterMode::ImmediateMode },
    parameter2_mode: if digits[3] == 0 { ParameterMode::PositionMode } else { ParameterMode::ImmediateMode },
    parameter3_mode: if digits[4] == 0 { ParameterMode::PositionMode } else { ParameterMode::ImmediateMode },
  }
}

static OPERATIONS: phf::Map<u32, Operation> = phf_map! {
  01u32 => Operation {
    name: "ADD",
    size: 4,
    execute: |program: &mut Vec<i32>, counter: &mut usize, op_instance: &OperationInstance| -> usize {
      let size = validate_operation(program, *counter, op_instance);
      let op1 = get_parameter_value(program, *counter, op_instance, 1);
      let op2 = get_parameter_value(program, *counter, op_instance, 2);
      assert!(op_instance.parameter3_mode != ParameterMode::ImmediateMode);
      let dst = program[*counter + 3];
      program[dst as usize] = op1 + op2;
      *counter + size
    },
  },
  02u32 => Operation {
    name: "MULTIPLY",
    size: 4,
    execute: |program: &mut Vec<i32>, counter: &mut usize, op_instance: &OperationInstance| -> usize {
      let size = validate_operation(program, *counter, op_instance);
      let op1 = get_parameter_value(program, *counter, op_instance, 1);
      let op2 = get_parameter_value(program, *counter, op_instance, 2);
      assert!(op_instance.parameter3_mode != ParameterMode::ImmediateMode);
      let dst = program[*counter + 3];
      program[dst as usize] = op1 * op2;
      *counter + size
    },
  },
  03u32 => Operation {
    name: "INPUT",
    size: 2,
    execute: |program: &mut Vec<i32>, counter: &mut usize, op_instance: &OperationInstance| -> usize {
      let size = validate_operation(program, *counter, op_instance);
      let dst = program[*counter + 1];
      let mut input = String::new();
      io::stdin().read_line(&mut input).unwrap();
      input = input.trim().to_string();
      program[dst as usize] = input.parse::<i32>().unwrap();
      *counter + size
    },
  },
  04u32 => Operation {
    name: "PRINT",
    size: 2,
    execute: |program: &mut Vec<i32>, counter: &mut usize, op_instance: &OperationInstance| -> usize {
      let size = validate_operation(program, *counter, op_instance);
      let dst = program[*counter + 1];
      println!("{}", program[dst as usize]);
      *counter + size
    },
  },
  05u32 => Operation {
    name: "JUMP-IF-TRUE",
    size: 3,
    execute: |program: &mut Vec<i32>, counter: &mut usize, op_instance: &OperationInstance| -> usize {
      let size = validate_operation(program, *counter, op_instance);
      let test = get_parameter_value(program, *counter, op_instance, 1);
      let dst_raw = get_parameter_value(program, *counter, op_instance, 2);
      if test != 0 {
        let dst: usize = validate_jump_destination(dst_raw, &program);
        return dst
      }
      *counter + size
    },
  },
  06u32 => Operation {
    name: "JUMP-IF-FALSE",
    size: 3,
    execute: |program: &mut Vec<i32>, counter: &mut usize, op_instance: &OperationInstance| -> usize {
      let size = validate_operation(program, *counter, op_instance);
      let test = get_parameter_value(program, *counter, op_instance, 1);
      let dst_raw = get_parameter_value(program, *counter, op_instance, 2);
      if test == 0 {
        let dst: usize = validate_jump_destination(dst_raw, &program);
        return dst
      }
      *counter + size
    },
  },
  07u32 => Operation {
    name: "LESS-THAN",
    size: 4,
    execute: |program: &mut Vec<i32>, counter: &mut usize, op_instance: &OperationInstance| -> usize {
      let size = validate_operation(program, *counter, op_instance);
      let op1 = get_parameter_value(program, *counter, op_instance, 1);
      let op2 = get_parameter_value(program, *counter, op_instance, 2);
      assert!(op_instance.parameter3_mode != ParameterMode::ImmediateMode);
      let dst = program[*counter + 3];
      if op1 < op2 {
        program[dst as usize] = 1;
      } else {
        program[dst as usize] = 0;
      }
      *counter + size
    },
  },
  08u32 => Operation {
    name: "EQUALS",
    size: 4,
    execute: |program: &mut Vec<i32>, counter: &mut usize, op_instance: &OperationInstance| -> usize {
      let size = validate_operation(program, *counter, op_instance);
      let op1 = get_parameter_value(program, *counter, op_instance, 1);
      let op2 = get_parameter_value(program, *counter, op_instance, 2);
      assert!(op_instance.parameter3_mode != ParameterMode::ImmediateMode);
      let dst = program[*counter + 3];
      if op1 == op2 {
        program[dst as usize] = 1;
      } else {
        program[dst as usize] = 0;
      }
      *counter + size
    },
  },
  99u32 => Operation {
    name: "HALT",
    size: 1,
    execute: |program: &mut Vec<i32>, counter: &mut usize, op_instance: &OperationInstance| -> usize {
      usize::max_value()
    },
  },
};

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

fn validate_operation(program: &Vec<i32>, counter: usize, op_instance: &OperationInstance) -> usize {
  let operation: &Operation = &OPERATIONS[&(op_instance.opcode as u32)];
  if !(1..=4).contains(&operation.size) {
    panic!("expected operation size to be in range [1..4], but found '{}'", operation.size);
  }
  if operation.size == 1 {
    // this is like halt, nothing to check
    return operation.size
  }
  if counter + operation.size > program.len() {
    panic!(
      "invalid instruction, expected three values after {} opcode, found '{:?}'",
      operation.name,
      program[counter..].iter());
  }
  if
    op_instance.parameter1_mode == ParameterMode::PositionMode &&
    (program[counter + 1] < 0 || program[counter + 1] as usize >= program.len())
  {
    panic!(
      "invalid address, first parameter of {} '{}' is out of range for the program of length '{}'",
      operation.name,
      program[counter + 1],
      program.len());
  }
  if
    operation.size > 2 &&
    op_instance.parameter2_mode == ParameterMode::PositionMode &&
    (program[counter + 2] < 0 || program[counter + 2] as usize >= program.len())
  {
    panic!(
      "invalid address, second parameter of {} '{}' is out of range for the program of length '{}'",
      operation.name,
      program[counter + 2],
      program.len());
  }
  if
    operation.size > 3 &&
    op_instance.parameter3_mode == ParameterMode::PositionMode &&
    (program[counter + 3] < 0 || program[counter + 3] as usize >= program.len())
  {
    panic!(
      "invalid address, third parameter of {} '{}' is out of range for the program of length '{}'",
      operation.name,
      program[counter + 3],
      program.len());
  }
  operation.size
}

fn split_into_five_digits_right_to_left(number: u32) -> [u32; 5] {
  if number > 99_999 {
    panic!("expected number with no more than 5 digits, got '{}'", number);
  }
  [
    number % 10,
    if number < 10 { 0 } else { (number % 100) / 10 },
    if number < 100 { 0 } else { (number % 1_000) / 100 },
    if number < 1_000 { 0 } else { (number % 10_000) / 1_000 },
    if number < 10_000 { 0 } else { (number % 100_000) / 10_000 },
  ]
}

fn load_program_from_comma_delimited_string(input_string: &str) -> Vec<i32> {
  let lines = input_string.split(",");
  let mut intcodes = Vec::new();
  for mut line in lines {
    line = line.trim();
    if line.is_empty() {
      continue;
    }
    intcodes.push(line.parse::<i32>().unwrap());
  }
  intcodes
}

fn load_program_from_comma_delimited_file(input_file: &str) -> Vec<i32> {
  let mut file = File::open(input_file).unwrap();
  let mut content = String::new();
  file.read_to_string(&mut content).unwrap();
  load_program_from_comma_delimited_string(&content)
}

fn execute_instruction_at(program: &mut Vec<i32>, counter: &mut usize) -> bool {
  // operation intcodes must be positive
  let raw_intcode: i32 = program[*counter];
  if raw_intcode < 0 {
    panic!("expected the opcode to be positive, got '{}' instead", raw_intcode);
  }
  let operation_intcode: u32 = u32::try_from(raw_intcode).unwrap();
  let operation_instance = parse_operation_intcode(operation_intcode);
  validate_operation(program, *counter, &operation_instance);
  let operation = &OPERATIONS[&operation_instance.opcode];
  // println!("Executing instruction '{}' at offset '{}': {:?} {:?}", operation.name, counter, &program[*counter..*counter + operation.size], operation_instance);
  *counter = (operation.execute)(program, counter, &operation_instance);
  if *counter == usize::max_value() {
    // halt
    return false
  }
  if *counter > program.len() - 1 {
    panic!("invlaid program, unexpected reached end of program without halt");
  }
  true
}

fn run_program(program: &mut Vec<i32>) {
  let mut counter: usize = 0;
  while program[counter] != 99 {
    if !execute_instruction_at(program, &mut counter) {
      // halt encountered, exit
      return
    }
  }
}
