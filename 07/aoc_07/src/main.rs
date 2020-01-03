use permutohedron::heap_recursive;

extern crate intcode_machine;

use intcode_machine::Program;
use intcode_machine::ProgramInstance;
use intcode_machine::StepError;
use intcode_machine::run_intcode_program;

fn main() {
  let original_program = Program::from_file("amplifier_program.csv");

  let mut best_signal = 0;
  for phase_sequence in create_phase_sequences(5) {
    best_signal = best_signal.max(run_amp_sequence(&mut original_program.clone(), &phase_sequence));
  }

  println!("best signal: {}", best_signal);
}

fn create_phase_sequences(number_of_amplifiers: i64) -> Vec<Vec<i32>> {
  let mut initial_vector = Vec::new();
  for i in 0..number_of_amplifiers {
    initial_vector.push((i + 5) as i32);
  }
  let mut permutations = Vec::new();
  heap_recursive(&mut initial_vector, |permutation| {
    let perm = permutation.to_vec();
    for x in &perm {
      if perm.iter().filter(|&y| x == y).count() != 1 {
        return
      }
    }
    permutations.push(perm)
  });

  permutations
}

fn run_amp_sequence(program: &Program, sequence: &Vec<i32>) -> i64 {
  let mut result: i64 = 0;
  for gain in sequence {
    let results = run_intcode_program(&mut (program.clone()), &Some(vec![*gain, result as i32]));
    assert_eq!(results.len(), 1);
    result = results[0] as i64;
  }
  result
}

fn run_amp_sequence_until_halt(program: &Program, sequence: &Vec<i32>) -> i64 {
  let mut result: i64 = 0;
  let mut programs = Vec::new();
  let mut program_instances = Vec::new();
  for _ in sequence {
    programs.push(program.clone());
    program_instances.push(ProgramInstance::new(&mut programs.last_mut().unwrap()));
  }
  loop {
    for (i, gain) in sequence.iter().enumerate() {
      // setup the next two inputs
      program_instances[i].context.inputs = Some(vec![*gain, result as i32]);
      // run until halt or input is requested again
      let mut next_result = 0;
      let mut halted = false;
      loop {
        match program_instances[i].step() {
          Ok(x) => match x {
            Some(x) => {
              if next_result != 0 {
                panic!("unexpectedly got second result");
              }
            },
            None => continue,
          },
          Err(x) => match x {
            StepError::NeedInput => break,
            StepError::EndOfProgram => {
              halted = true;
              break;
            },
            StepError::Error(msg) => panic!("unexpected error: {}", msg),
          },
        }
      }
      if next_result == 0 {
        panic!("program ended before giving result");
      }
      result = next_result;
      if halted {
        program_instances.remove(i);
      }
      // let results = run_intcode_program(&mut (programs[i]), &Some(vec![*gain, result as i32]));
      // assert_eq!(results.len(), 1);
      // result = results[0] as i64;
    }
  }
  result
}

#[test]
fn example1() {
  let original_program = Program::from_string("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");

  assert_eq!(run_amp_sequence(&original_program, &vec![4,3,2,1,0]), 43210);
}

#[test]
fn example2() {
  let original_program = Program::from_string("
3,23,3,24,1002,24,10,24,1002,23,-1,23,
101,5,23,23,1,24,23,23,4,23,99,0,0");

  assert_eq!(run_amp_sequence(&original_program, &vec![0,1,2,3,4]), 54321);
}

#[test]
fn example3() {
  let original_program = Program::from_string("
3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0");

  assert_eq!(run_amp_sequence(&original_program, &vec![1,0,4,3,2]), 65210);
}
