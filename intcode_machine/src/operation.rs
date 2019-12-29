use super::operation_instance::OperationInstance;
use super::operations::OPERATIONS;
use super::parameter_mode::ParameterMode;
use super::program::ProgramContext;

pub
struct Operation {
  pub name: &'static str,
  pub size: usize,
  pub execute: fn(
    program: &mut Vec<i32>,
    program_context: &mut ProgramContext,
    op_instance: &OperationInstance) -> usize,
}

pub
fn validate_operation(program: &Vec<i32>, counter: usize, op_instance: &OperationInstance) -> usize {
  if !OPERATIONS.contains_key(&op_instance.opcode) {
    let mut keys = Vec::new();
    for key in OPERATIONS.keys() {
      keys.push(*key);
    }
    keys.sort();
    panic!("invalid opcode key '{}', expected on of: {:?}", op_instance.opcode, keys);
  }
  let operation: &Operation = &OPERATIONS[&op_instance.opcode];
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
