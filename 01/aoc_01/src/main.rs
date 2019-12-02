use std::fs::File;
use std::io::prelude::*;

fn main() {
  let masses = load_input_masses_from_newline_delimited_file("input.txt");
  let fuel_needed = calculate_fuel_for_modules_given_masses(&masses);
  println!("{:?}", fuel_needed);
}

fn calculate_fuel(mass: f64, divisor: f64, subtractor: f64, should_floor: bool) -> u64 {
  let mut fuel_float: f64 = mass / divisor;
  if should_floor {
    fuel_float = fuel_float.floor();
  }
  (fuel_float- subtractor) as u64
}

fn calculate_fuel_given_mass(mass: f64) -> u64{
  calculate_fuel(mass, 3.0, 2.0, true)
}

fn sum_fuel_costs(fuel_costs: &[u64]) -> u64 {
  let mut total_fuel_cost: u64 = 0;
  for fuel_cost in fuel_costs {
    total_fuel_cost += fuel_cost;
  }
  total_fuel_cost
}

fn calculate_fuel_for_modules_given_masses(masses: &[f64]) -> u64 {
  let mut fuel_costs = Vec::new();
  for mass in masses {
    fuel_costs.push(calculate_fuel_given_mass(*mass));
  }
  sum_fuel_costs(&fuel_costs)
}

fn load_input_masses_from_newline_delimited_string(input_string: &str) -> Vec<f64> {
  let lines = input_string.split("\n");
  let mut masses = Vec::new();
  for line in lines {
    if line.is_empty() {
      continue;
    }
    masses.push(line.parse::<f64>().unwrap());
  }
  masses
}

fn load_input_masses_from_newline_delimited_file(input_file: &str) -> Vec<f64> {
  let mut file = File::open(input_file).unwrap();
  let mut content = String::new();
  file.read_to_string(&mut content).unwrap();
  load_input_masses_from_newline_delimited_string(&content)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn examples_from_first_question() {
    assert_eq!(calculate_fuel_given_mass(12.0), 2);
    assert_eq!(calculate_fuel_given_mass(14.0), 2);
    assert_eq!(calculate_fuel_given_mass(1969.0), 654);
    assert_eq!(calculate_fuel_given_mass(100756.0), 33583);
  }

  #[test]
  fn basic_sum() {
    assert_eq!(calculate_fuel_for_modules_given_masses(&[12.0, 14.0]), 4);
  }

  #[test]
  fn input_masses_basic() {
    let input_string = "1234\n5678";
    assert_eq!(load_input_masses_from_newline_delimited_string(input_string), [1234.0, 5678.0]);
  }

  #[test]
  fn input_masses_extra_newline() {
    let input_string = "\n1234\n5678\n";
    assert_eq!(load_input_masses_from_newline_delimited_string(input_string), [1234.0, 5678.0]);
  }
}
