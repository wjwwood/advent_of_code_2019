use std::convert::TryFrom;

use super::instruction_type::InstructionType;
use super::operation_instance::parse_operation_intcode;
use super::operations::OPERATIONS;
use super::program::Program;
use super::program::ProgramContext;

pub
fn execute_instruction_at(
  program: &mut Program,
  mut program_context: &mut ProgramContext,
  trace: bool,
) -> bool
{
  // operation intcodes must be positive
  let raw_intcode: InstructionType = program.instructions[program_context.counter];
  if raw_intcode < 0 {
    panic!("expected the opcode to be positive, got '{}' instead", raw_intcode);
  }
  let operation_intcode: InstructionType = InstructionType::try_from(raw_intcode).unwrap();
  let operation_instance = parse_operation_intcode(operation_intcode);
  if trace {
    println!(
      "Executing instruction at offset '{}': {:?} {:?}",
      program_context.counter,
      &program.instructions[program_context.counter..program_context.counter + 4],
      operation_instance);
  }
  if !OPERATIONS.contains_key(&operation_instance.opcode) {
    let mut keys = Vec::new();
    for key in OPERATIONS.keys() {
      keys.push(key);
    }
    keys.sort();
    panic!(
      "Unknown operation with opcode '{}', expected one of these: {:?}",
      &operation_instance.opcode,
      keys);
  }
  let operation = &OPERATIONS[&operation_instance.opcode];
  program_context.counter = (operation.execute)(&mut (program.instructions), &mut program_context, &operation_instance);
  if program_context.counter == usize::max_value() {
    // halt
    return false
  }
  if program_context.counter > program.instructions.len() - 1 {
    panic!("invlaid program, unexpected reached end of program without halt");
  }
  true
}
