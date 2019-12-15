use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
  // let (known_chemicals, reactions) = parse_reactions_string_from_file("examples/example1.txt");
  // let (known_chemicals, reactions) = parse_reactions_string_from_file("examples/example2.txt");
  // let (known_chemicals, reactions) = parse_reactions_string_from_file("examples/example3.txt");
  // let (known_chemicals, reactions) = parse_reactions_string_from_file("examples/example4.txt");
  let (known_chemicals, reactions) = parse_reactions_string_from_file("examples/example5.txt");
  // let (known_chemicals, reactions) = parse_reactions_string_from_file("part1_input.txt");

  assert!(known_chemicals.contains(&Chemical {name: "FUEL".to_string()}));
  assert!(known_chemicals.contains(&Chemical {name: "ORE".to_string()}));

  let fuel_reactions = find_reactions_with_chemical_in_outputs("FUEL", &reactions);
  assert_eq!(fuel_reactions.len(), 1);
  let fuel_reaction = fuel_reactions[0].clone();
  println!("Reaction producing FUEL: {:?}", fuel_reaction);

  let mut pool = HashMap::new();
  println!("ORE needed: {}", calculate_ore_needed_for_reaction(&fuel_reaction, &reactions, &mut pool));
  println!("{:?}", pool);

  // let mut pool = HashMap::new();
  // let mut ore_needed = 0;
  // let mut max_fuel_produced = 0;

  // loop {
  //   ore_needed += calculate_ore_needed_for_reaction(&fuel_reaction, &reactions, &mut pool);
  //   if ore_needed < 1_000_000_000_000 {
  //     max_fuel_produced += 1;
  //     println!("FUEL produced: {}, ORE required: {}", max_fuel_produced, ore_needed);
  //   } else {
  //     break;
  //   }
  // }
  // println!("FUEL obtained with '{}' ORE: {}", ore_needed, max_fuel_produced);
}

fn calculate_ore_needed_for_n_reactions(
  reaction: &Reaction,
  number_of_reactions: usize,
  reactions: &Vec<Reaction>,
  pool: &mut ChemicalsPool
) -> usize
{
  let mut ore_needed = 0;

  // println!("Processing reaction: {}", reaction.to_string());
  println!("Creating {} {}, {} times", reaction.outputs[0].amount, reaction.outputs[0].kind.name, number_of_reactions);
  for input in &reaction.inputs {
    // println!("  Processing reaction input: {:?}", input);
    loop {
      // if input is ore, add it to the total and continue
      if input.kind.name == "ORE" {
        ore_needed += input.amount * number_of_reactions;
        break;
      } else {
        let required_amount = input.amount * number_of_reactions;
        // ensure the pool contains the chemical kind
        pool.entry(input.kind.clone()).or_insert(Chemicals {kind: input.kind.clone(), amount: 0});
        if pool[&input.kind].amount >= required_amount {
          pool.get_mut(&input.kind).unwrap().amount -= input.amount * number_of_reactions;
          break;
        }
        let matched_reactions = find_reactions_with_chemical_in_outputs(&input.kind.name, reactions);
        assert_eq!(matched_reactions.len(), 1);
        assert_eq!(matched_reactions[0].outputs.len(), 1);
        let mut needed_cycles = required_amount / matched_reactions[0].outputs[0].amount;
        if needed_cycles == 0 {
          needed_cycles = 1;
        }
        // if required_amount % matched_reactions[0].outputs[0].amount != 0 {
        //   needed_cycles += 1;
        // }
        ore_needed += calculate_ore_needed_for_n_reactions(
          &matched_reactions[0],
          needed_cycles,
          // 1,
          reactions,
          pool);
        for output in &matched_reactions[0].outputs {
          pool.get_mut(&output.kind).unwrap().amount += output.amount * needed_cycles;
          // pool.get_mut(&output.kind).unwrap().amount += output.amount;
        }
      }
    }
  }

  ore_needed
}

fn calculate_ore_needed_for_reaction(reaction: &Reaction, reactions: &Vec<Reaction>, pool: &mut ChemicalsPool) -> usize {
  calculate_ore_needed_for_n_reactions(reaction, 1, reactions, pool)
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Chemical {
  name: String,
  // producing_reactions: Vec<Reaction>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Chemicals {
  kind: Chemical,
  amount: usize,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Reaction {
  inputs: Vec<Chemicals>,
  outputs: Vec<Chemicals>,
}

impl Reaction {
  fn to_string(&self) -> String {
    let mut in_chem_strings = Vec::new();
    for i in &self.inputs {
      in_chem_strings.push(format!("{} {}", i.amount, i.kind.name));
    }
    let mut out_chem_strings = Vec::new();
    for o in &self.outputs {
      out_chem_strings.push(format!("{} {}", o.amount, o.kind.name));
    }
    format!("{} => {}", in_chem_strings.join(", "), out_chem_strings.join(", "))
  }
}

type ChemicalsPool = HashMap<Chemical, Chemicals>;

fn find_reactions_with_chemical_in_outputs(name: &str, reactions: &Vec<Reaction>) -> Vec<Reaction> {
  let mut found_reactions = Vec::new();

  for reaction in reactions {
    if reaction.outputs.iter().any(|x| x.kind.name == name) {
      found_reactions.push(reaction.clone());
    }
  }

  found_reactions
}

fn parse_chemicals_strings(chemicals_strings: Vec<&str>) -> Vec<Chemicals> {
  let mut chemicals = Vec::new();

  for chemicals_string in chemicals_strings {
    let chemical_tokens: Vec<_> = chemicals_string.trim().split(" ").collect();
    assert_eq!(chemical_tokens.len(), 2);
    chemicals.push(Chemicals {
      kind: Chemical {name: chemical_tokens[1].to_string()},
      amount: chemical_tokens[0].parse::<usize>().unwrap()
    });
  }

  chemicals
}

fn parse_reactions_string(input_string: &str) -> (Vec<Chemical>, Vec<Reaction>) {
  let mut chemical_kinds = Vec::new();
  let mut reactions = Vec::new();

  for line in input_string.split("\n") {
    let trimmed_line = line.trim();
    if trimmed_line.is_empty() {
      continue;
    }
    let reaction_tokens: Vec<_> = trimmed_line.split("=>").collect();
    assert_eq!(reaction_tokens.len(), 2);
    let input_chemicals_strings: Vec<_> = reaction_tokens[0].split(",").collect();
    let input_chemicals = parse_chemicals_strings(input_chemicals_strings);

    let output_chemicals_strings: Vec<_> = reaction_tokens[1].split(",").collect();
    let output_chemicals = parse_chemicals_strings(output_chemicals_strings);

    let new_reaction = Reaction {
      inputs: input_chemicals,
      outputs: output_chemicals,
    };

    for input in &new_reaction.inputs {
      if !chemical_kinds.contains(&input.kind) {
        chemical_kinds.push(input.kind.clone());
      }
    }
    for output in &new_reaction.outputs {
      if !chemical_kinds.contains(&output.kind) {
        chemical_kinds.push(output.kind.clone());
      }
    }

    reactions.push(new_reaction);
  }

  (chemical_kinds, reactions)
}

fn parse_reactions_string_from_file(input_file: &str) -> (Vec<Chemical>, Vec<Reaction>) {
  let mut file = File::open(input_file).unwrap();
  let mut content = String::new();
  file.read_to_string(&mut content).unwrap();
  parse_reactions_string(&content)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_chemicals_strings() {
    assert_eq!(
      parse_chemicals_strings(vec!["10 A", "2 B"]),
      vec![
        Chemicals {kind: Chemical {name: "A".to_string()}, amount: 10},
        Chemicals {kind: Chemical {name: "B".to_string()}, amount: 2},
      ],
    );
  }

  #[test]
  fn test_parse_reactions_string() {
    assert_eq!(
      parse_reactions_string("10 ORE => 2 A"),
      (
        vec![
          Chemical {name: "ORE".to_string()},
          Chemical {name: "A".to_string()},
        ],
        vec![
          Reaction {
            inputs: vec![
              Chemicals {kind: Chemical {name: "ORE".to_string()}, amount: 10},
            ],
            outputs: vec![
              Chemicals {kind: Chemical {name: "A".to_string()}, amount: 2},
            ],
          }
        ],
      )
    );

    assert_eq!(
      parse_reactions_string("10 B, 12 C => 1 FUEL"),
      (
        vec![
          Chemical {name: "B".to_string()},
          Chemical {name: "C".to_string()},
          Chemical {name: "FUEL".to_string()},
        ],
        vec![
          Reaction {
            inputs: vec![
              Chemicals {kind: Chemical {name: "B".to_string()}, amount: 10},
              Chemicals {kind: Chemical {name: "C".to_string()}, amount: 12},
            ],
            outputs: vec![
              Chemicals {kind: Chemical {name: "FUEL".to_string()}, amount: 1},
            ],
          }
        ],
      )
    );

    assert_eq!(
      parse_reactions_string("10 B, 12 C => 1 D, 2 X"),
      (
        vec![
          Chemical {name: "B".to_string()},
          Chemical {name: "C".to_string()},
          Chemical {name: "D".to_string()},
          Chemical {name: "X".to_string()},
        ],
        vec![
          Reaction {
            inputs: vec![
              Chemicals {kind: Chemical {name: "B".to_string()}, amount: 10},
              Chemicals {kind: Chemical {name: "C".to_string()}, amount: 12},
            ],
            outputs: vec![
              Chemicals {kind: Chemical {name: "D".to_string()}, amount: 1},
              Chemicals {kind: Chemical {name: "X".to_string()}, amount: 2},
            ],
          }
        ],
      )
    );

    assert_eq!(
      parse_reactions_string("20 ORE => 10 B\n10 B, 12 C => 1 D, 2 X\n10 C => 1 FUEL"),
      (
        vec![
          Chemical {name: "ORE".to_string()},
          Chemical {name: "B".to_string()},
          Chemical {name: "C".to_string()},
          Chemical {name: "D".to_string()},
          Chemical {name: "X".to_string()},
          Chemical {name: "FUEL".to_string()},
        ],
        vec![
          Reaction {
            inputs: vec![
              Chemicals {kind: Chemical {name: "ORE".to_string()}, amount: 20},
            ],
            outputs: vec![
              Chemicals {kind: Chemical {name: "B".to_string()}, amount: 10},
            ],
          },
          Reaction {
            inputs: vec![
              Chemicals {kind: Chemical {name: "B".to_string()}, amount: 10},
              Chemicals {kind: Chemical {name: "C".to_string()}, amount: 12},
            ],
            outputs: vec![
              Chemicals {kind: Chemical {name: "D".to_string()}, amount: 1},
              Chemicals {kind: Chemical {name: "X".to_string()}, amount: 2},
            ],
          },
          Reaction {
            inputs: vec![
              Chemicals {kind: Chemical {name: "C".to_string()}, amount: 10},
            ],
            outputs: vec![
              Chemicals {kind: Chemical {name: "FUEL".to_string()}, amount: 1},
            ],
          },
        ],
      )
    );
  }
}
