use super::parameter_mode::ParameterMode;
use super::instruction_type::InstructionType;

#[derive(Debug, PartialEq)]
pub
struct OperationInstance {
  pub opcode: InstructionType,
  pub parameter1_mode: ParameterMode,
  pub parameter2_mode: ParameterMode,
  pub parameter3_mode: ParameterMode,
}

#[allow(dead_code)]
fn split_into_five_digits_right_to_left(number: InstructionType) -> [InstructionType; 5] {
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

#[cfg(test)]
mod split_into_five_digits_right_to_left_tests {
  use super::split_into_five_digits_right_to_left;

  #[test]
  fn test_split_into_five_digits_right_to_left() {
    assert_eq!(split_into_five_digits_right_to_left(12345), [5, 4, 3, 2, 1]);
    assert_eq!(split_into_five_digits_right_to_left(1), [1, 0, 0, 0, 0]);
  }

  #[test]
  #[should_panic]
  fn test_bad_split_into_five_digits_right_to_left() {
    assert_eq!(split_into_five_digits_right_to_left(199_9999), [1, 2, 3, 4, 5]);
  }
}

#[allow(dead_code)]
pub
fn parse_operation_intcode(operation_intcode: InstructionType) -> OperationInstance {
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

#[cfg(test)]
mod parse_operation_intcode_tests {
  use super::OperationInstance;
  use super::ParameterMode;
  use super::parse_operation_intcode;

  #[test]
  fn test_parse_operation_intcode() {
    assert_eq!(parse_operation_intcode(01001), OperationInstance {
      opcode: 1,
      parameter1_mode: ParameterMode::PositionMode,
      parameter2_mode: ParameterMode::ImmediateMode,
      parameter3_mode: ParameterMode::PositionMode,
    });
    assert_eq!(parse_operation_intcode(1), OperationInstance {
      opcode: 1,
      parameter1_mode: ParameterMode::PositionMode,
      parameter2_mode: ParameterMode::PositionMode,
      parameter3_mode: ParameterMode::PositionMode,
    });
  }

  #[test]
  #[should_panic(expected = "expected parameter mode for parameter 1 to be 0 or 1")]
  fn test_invalid_parameter_mode_for_1() {
    assert_eq!(parse_operation_intcode(11201), OperationInstance {
      opcode: 1,
      parameter1_mode: ParameterMode::PositionMode,
      parameter2_mode: ParameterMode::PositionMode,
      parameter3_mode: ParameterMode::PositionMode,
    });
  }

  #[test]
  #[should_panic(expected = "expected parameter mode for parameter 2 to be 0 or 1")]
  fn test_invalid_parameter_mode_for_2() {
    assert_eq!(parse_operation_intcode(12101), OperationInstance {
      opcode: 1,
      parameter1_mode: ParameterMode::PositionMode,
      parameter2_mode: ParameterMode::PositionMode,
      parameter3_mode: ParameterMode::PositionMode,
    });
  }

  #[test]
  #[should_panic(expected = "expected parameter mode for parameter 3 to be 0 or 1")]
  fn test_invalid_parameter_mode_for_3() {
    assert_eq!(parse_operation_intcode(21101), OperationInstance {
      opcode: 1,
      parameter1_mode: ParameterMode::PositionMode,
      parameter2_mode: ParameterMode::PositionMode,
      parameter3_mode: ParameterMode::PositionMode,
    });
  }
}
