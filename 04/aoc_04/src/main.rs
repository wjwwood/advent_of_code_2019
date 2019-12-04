fn main() {
  let valid_passwords = find_valid_passwords_in_range((278384, 824795));
  println!("{:?}", valid_passwords.len());
}

fn find_valid_passwords_in_range(range: (u32, u32)) -> Vec<u32> {
  let mut valid_passwords = Vec::new();
  for x in range.0..range.1 {
    if is_valid_password(x) {
      valid_passwords.push(x);
    }
  }
  valid_passwords
}

fn is_valid_password(number: u32) -> bool {
  let digits = split_digits(number);
  (
    has_at_least_one_set_of_exactly_two_adjacent_digits_the_same(&digits) &&
    digits_never_decrease(&digits)
  )
}

fn is_six_digits(number: u32) -> bool {
  number > 99_999 && number < 1_000_000
}

fn split_digits(number: u32) -> [u32; 6] {
  if !is_six_digits(number) {
    panic!("expected number with six digits, got '{}'", number);
  }
  [
    (number % 1000000) / 100000,
    (number % 100000) / 10000,
    (number % 10000) / 1000,
    (number % 1000) / 100,
    (number % 100) / 10,
    number % 10,
  ]
}

// fn has_at_least_two_adjacent_digits_the_same(digits: &[u32; 6]) -> bool {
//   let mut previous_digit: u32 = 0;
//   for digit in digits {
//     if previous_digit == 0 {
//       previous_digit = *digit;
//       continue;
//     }
//     if *digit == previous_digit {
//       return true
//     }
//     previous_digit = *digit;
//   }
//   return false
// }

fn has_at_least_one_set_of_exactly_two_adjacent_digits_the_same(digits: &[u32; 6]) -> bool {
  let mut len_of_current_chain = 1;
  let mut previous_digit: u32 = 0;
  for digit in digits {
    if previous_digit == 0 {
      previous_digit = *digit;
      continue;
    }
    if *digit != previous_digit {
      if len_of_current_chain == 2 {
        return true
      }
      len_of_current_chain = 1;
    } else {
      len_of_current_chain += 1;
    }
    previous_digit = *digit;
  }
  if len_of_current_chain == 2 {
    return true
  }
  return false
}

fn digits_never_decrease(digits: &[u32; 6]) -> bool {
  let mut previous_digit: u32 = 0;
  for digit in digits {
    if previous_digit == 0 {
      previous_digit = *digit;
      continue;
    }
    if *digit < previous_digit {
      return false
    }
    previous_digit = *digit;
  }
  return true
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_is_valid_password() {
    assert!(is_valid_password(223456));
    assert!(!is_valid_password(111111));
    assert!(!is_valid_password(223450));
    assert!(!is_valid_password(123789));
  }

  #[test]
  fn test_is_six_digits() {
    assert!(is_six_digits(123456));
    assert!(!is_six_digits(1234567));
    assert!(!is_six_digits(12345));
  }

  // #[test]
  // fn test_has_at_least_two_adjacent_digits_the_same() {
  //   assert!(has_at_least_two_adjacent_digits_the_same(&split_digits(123455)));
  //   assert!(!has_at_least_two_adjacent_digits_the_same(&split_digits(123456)));
  // }

  #[test]
  fn test_has_at_least_one_set_of_exactly_two_adjacent_digits_the_same() {
    assert!(has_at_least_one_set_of_exactly_two_adjacent_digits_the_same(&split_digits(112233)));
    assert!(!has_at_least_one_set_of_exactly_two_adjacent_digits_the_same(&split_digits(123444)));
    assert!(has_at_least_one_set_of_exactly_two_adjacent_digits_the_same(&split_digits(111122)));
  }

  #[test]
  fn test_digits_never_decrease() {
    assert!(digits_never_decrease(&split_digits(123456)));
    assert!(!digits_never_decrease(&split_digits(123450)));
  }
}
