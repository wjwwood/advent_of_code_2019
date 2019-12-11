use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
  // let uom = parse_universal_orbit_map_from_string("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L");
  // println!("{:?}", count_orbits_in_universal_orbit_map(&uom));
  let uom = parse_universal_orbit_map_from_file("map1.txt");
  println!("{:?}", count_orbits_in_universal_orbit_map(&uom));
}

fn count_orbits_for_space_object(space_object: &SpaceObject) -> usize {
  let mut number_of_orbits: usize = 0;

  let mut next: Option<SpaceObjectLink> = space_object.orbited_object.clone();
  loop {
    next = match next {
      Some(x) => x.borrow().orbited_object.clone(),
      None => break,
    };
    number_of_orbits += 1;
  }

  number_of_orbits
}

fn count_orbits_in_universal_orbit_map(uom: &UniversalOrbitMap) -> usize {
  let mut number_of_orbits: usize = 0;

  for (_, space_object_link) in &uom.orbits {
    number_of_orbits += count_orbits_for_space_object(&space_object_link.borrow());
  }

  number_of_orbits
}

#[derive(Debug)]
struct SpaceObject {
  name: String,
  orbited_object: Option<SpaceObjectLink>,
  satellites: HashMap<String, SpaceObjectLink>,
  self_link: Option<SpaceObjectLink>,
}

type SpaceObjectLink = Rc<RefCell<SpaceObject>>;

impl SpaceObject {
  fn new_link() -> SpaceObjectLink {
    let new_space_object_link = Rc::new(RefCell::new(SpaceObject {
      name: "COM".to_string(),
      orbited_object: None,
      satellites: HashMap::new(),
      self_link: None
    }));
    new_space_object_link.borrow_mut().self_link = Some(new_space_object_link.clone());
    new_space_object_link
  }

  fn add_satellite(&mut self, name: &str) {
    let name_str: String = name.to_string();
    let new_space_object = Rc::new(RefCell::new(SpaceObject {
      name: name_str.clone(),
      orbited_object: None,
      satellites: HashMap::new(),
      self_link: None,
    }));
    new_space_object.borrow_mut().orbited_object = self.self_link.clone();
    new_space_object.borrow_mut().self_link = Some(new_space_object.clone());
    self.satellites.insert(name_str, new_space_object);
  }
}

#[derive(Debug)]
struct UniversalOrbitMap {
  orbits: HashMap<String, SpaceObjectLink>,
}

fn parse_universal_orbit_map_from_string(input_string: &str) -> UniversalOrbitMap {
  let mut uom = UniversalOrbitMap { orbits: HashMap::new() };
  uom.orbits.insert("COM".to_string(), SpaceObject::new_link());

  let mut unlinked_orbits: Vec<(String, String)> = Vec::new();

  let lines = input_string.split("\n");
  for mut line in lines {
    line = line.trim();
    if line.is_empty() {
      continue;
    }
    let tokens: Vec<&str> = line.split(")").collect();
    assert!(tokens.len() == 2);

    if !uom.orbits.contains_key(tokens[0]) {
      // panic!("space object '{}' is referenced before declared, i.e. seen on left hand side of X)Y before right hand side, for line '{}'", tokens[1], line);
      unlinked_orbits.push((tokens[0].to_string(), tokens[1].to_string()));
      continue;
    }

    if uom.orbits.contains_key(tokens[1]) {
      panic!("space object '{}' has more than one parent, i.e. seen on right hand side of X)Y more than once, for line '{}'", tokens[1], line);
    }
    let space_object_link = uom.orbits.get_mut(tokens[0]).unwrap().clone();
    space_object_link.borrow_mut().add_satellite(tokens[1]);
    uom.orbits.insert(tokens[1].to_string(), space_object_link.borrow().satellites[tokens[1]].clone());
  }

  let mut previous_unlinked_orbits_len = 0;
  while unlinked_orbits.len() != 0 {
    if previous_unlinked_orbits_len == unlinked_orbits.len() {
      panic!("could not resolve '{}' orbits, due to missing references: {:?}", unlinked_orbits.len(), unlinked_orbits);
    }
    previous_unlinked_orbits_len = unlinked_orbits.len();

    unlinked_orbits.retain(|unlinked_orbit| {
      let should_retain = if uom.orbits.contains_key(&unlinked_orbit.0) {
        let space_object_link = uom.orbits.get_mut(&unlinked_orbit.0).unwrap().clone();
        space_object_link.borrow_mut().add_satellite(&unlinked_orbit.1);
        uom.orbits.insert(unlinked_orbit.1.to_string(), space_object_link.borrow().satellites[&unlinked_orbit.1].clone());
        false
      } else {
        true
      };
      should_retain
    })
  };

  uom
}

fn parse_universal_orbit_map_from_file(input_file: &str) -> UniversalOrbitMap {
  let mut file = File::open(input_file).unwrap();
  let mut content = String::new();
  file.read_to_string(&mut content).unwrap();
  parse_universal_orbit_map_from_string(&content)
}
