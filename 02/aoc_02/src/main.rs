use std::fs::File;
use std::io::prelude::*;

fn main() {
  // let mut program: Vec<u32> = load_program_from_comma_delimited_file("program1.csv");
  // program[1] = 12;
  // program[2] = 2;
  // run_program(&mut program);
  // println!("{:?}", program[0]);
  let original_program: Vec<u32> = load_program_from_comma_delimited_file("program1.csv");
  // search for the noun/verb combination ([0..99] for each) that results in 19690720
  for noun in 0..99 {
    for verb in 0..99 {
      let mut program: Vec<u32> = original_program.clone();
      program[1] = noun;
      program[2] = verb;
      run_program(&mut program);
      if program[0] == 19690720 {
        println!("Found noun, verb combo that results in 19690720: ({:?}, {:?})", noun, verb);
        println!("Answer to '100 * noun + verb': {:?}", 100 * noun + verb);
      }
    }
  }
}

fn load_program_from_comma_delimited_string(input_string: &str) -> Vec<u32> {
  let lines = input_string.split(",");
  let mut intcodes = Vec::new();
  for mut line in lines {
    line = line.trim();
    if line.is_empty() {
      continue;
    }
    intcodes.push(line.parse::<u32>().unwrap());
  }
  intcodes
}

fn load_program_from_comma_delimited_file(input_file: &str) -> Vec<u32> {
  let mut file = File::open(input_file).unwrap();
  let mut content = String::new();
  file.read_to_string(&mut content).unwrap();
  load_program_from_comma_delimited_string(&content)
}

fn check_valid_two_operand_instruction(
  program: &mut Vec<u32>,
  counter: usize,
  instruction_name: &str)
{
  if counter + 3 > program.len() - 1 {
    let slice = program[counter..].iter();
    panic!(
      "invalid instruction, expected three values after {} opcode, found '{:?}'",
      instruction_name,
      slice);
  }
  let op1 = program[counter + 1];
  let op2 = program[counter + 2];
  let dst = program[counter + 3];
  if op1 as usize >= program.len() {
    panic!(
      "invalid address, first operand of {} '{}' is out of range for the program of length '{}'",
      instruction_name,
      op1,
      program.len());
  }
  if op2 as usize >= program.len() {
    panic!(
      "invalid address, second operand of {} '{}' is out of range for the program of length '{}'",
      instruction_name,
      op2,
      program.len());
  }
  if dst as usize >= program.len() {
    panic!(
      "invalid address, destination of {} '{}' is out of range for the program of length '{}'",
      instruction_name,
      dst,
      program.len());
  }
}

fn execute_opcode(program: &mut Vec<u32>, counter: usize) {
  match program[counter] {
    1 => {
      // add
      check_valid_two_operand_instruction(program, counter, "ADD");
      let op1 = program[counter + 1];
      let op2 = program[counter + 2];
      let dst = program[counter + 3];
      program[dst as usize] = program[op1 as usize] + program[op2 as usize];
    }
    2 => {
      // multiply
      check_valid_two_operand_instruction(program, counter, "MULTIPLY");
      let op1 = program[counter + 1];
      let op2 = program[counter + 2];
      let dst = program[counter + 3];
      program[dst as usize] = program[op1 as usize] * program[op2 as usize];
    }
    99 => {
      // halt
    }
    _ => {
      panic!("invalid opcode '{}', expected one of [1, 2, 99]");
    }
  }
}

fn run_program(program: &mut Vec<u32>) {
  let mut counter: usize = 0;
  while program[counter] != 99 {
    execute_opcode(program, counter);
    counter += 4;
    if counter > program.len() - 1 {
      panic!("invlaid program, expected sets of 4 intcodes until 99 reached");
    }
  }
}
