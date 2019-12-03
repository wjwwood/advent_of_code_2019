use std::fs::File;
use std::io::prelude::*;

fn main() {
  let (wire1_path, wire2_path) = load_wire_paths_from_file("wire_paths.csv");
  println!("{:?}", find_distance_to_closest_intersection(&wire1_path, &wire2_path));
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Coordinate {
  x: i32,
  y: i32,
  wire_length: u32,
}

#[derive(Debug)]
struct SparseOccupancyGrid {
  occupied_coordinates: Vec<Coordinate>,
}

fn create_move_operation_from_direction(direction: char) -> Box<Fn(&mut Coordinate)> {
  match direction {
    'U' => {
      Box::new(move |c: &mut Coordinate| c.y += 1)
    }
    'D' => {
      Box::new(move |c: &mut Coordinate| c.y -= 1)
    }
    'R' => {
      Box::new(move |c: &mut Coordinate| c.x += 1)
    }
    'L' => {
      Box::new(move |c: &mut Coordinate| c.x -= 1)
    }
    _ => {
      panic!("unexpected direction '{:?}'", direction);
    }
  }
}

fn generate_occupied_coordinates(wire_path: &Vec<PathSegment>) -> SparseOccupancyGrid {
  // do not count 0, 0 as occupied
  let mut current_coordinate = Coordinate {x: 0, y: 0, wire_length: 0};
  let mut occupied_coordinates = SparseOccupancyGrid {occupied_coordinates: Vec::new()};
  for path_seg in wire_path {
    let operation_closure = create_move_operation_from_direction(path_seg.direction);
    for _ in 0..path_seg.distance {
      operation_closure(&mut current_coordinate);
      current_coordinate.wire_length += 1;
      occupied_coordinates.occupied_coordinates.push(current_coordinate.clone());
    }
  }
  let mut unique_occupied_coordinates = SparseOccupancyGrid {occupied_coordinates: Vec::new()};
  for oc in &occupied_coordinates.occupied_coordinates {
    let mut already_in = false;
    for uoc in &mut unique_occupied_coordinates.occupied_coordinates {
      if uoc.x == oc.x && uoc.y == oc.y {
        already_in = true;
        if oc.wire_length < uoc.wire_length {
          uoc.wire_length = oc.wire_length;
        }
      }
    }
    if !already_in {
      unique_occupied_coordinates.occupied_coordinates.push(oc.clone());
    }
  }
  unique_occupied_coordinates
}

fn find_intersections(
  wire1_path: &Vec<PathSegment>,
  wire2_path: &Vec<PathSegment>
) -> Vec<Coordinate>
{
  let mut intersections = Vec::new();

  let wire1_occupied_coordinates = generate_occupied_coordinates(wire1_path);
  let wire2_occupied_coordinates = generate_occupied_coordinates(wire2_path);

  for wire1_coord in &wire1_occupied_coordinates.occupied_coordinates {
    for wire2_coord in &wire2_occupied_coordinates.occupied_coordinates {
      if wire1_coord.x == wire2_coord.x && wire1_coord.y == wire2_coord.y {
        // intersections.push(wire1_coord.clone());
        intersections.push(Coordinate {
          x: wire1_coord.x,
          y: wire1_coord.y,
          wire_length: wire1_coord.wire_length + wire2_coord.wire_length
        });
      }
    }
  }

  intersections
}

// fn calculate_manhattan_distance_for_coordinate(coordinate: &Coordinate) -> u32 {
//   (coordinate.x.abs() + coordinate.y.abs()) as u32
// }

fn find_distance_to_closest_intersection(
  wire1_path: &Vec<PathSegment>,
  wire2_path: &Vec<PathSegment>
) -> u32
{
  let intersections = find_intersections(&wire1_path, &wire2_path);
  let mut distances: Vec<(Coordinate, u32)> = Vec::new();
  for intersection in &intersections {
    // distances.push((intersection.clone(), calculate_manhattan_distance_for_coordinate(&intersection)));
    distances.push((intersection.clone(), intersection.wire_length));
  }
  let mut smallest_distance = 0;
  for distance in &distances {
    if smallest_distance == 0 {
      smallest_distance = distance.1;
      continue;
    }
    if distance.1 < smallest_distance {
      smallest_distance = distance.1;
    }
  }
  let mut shortest_distances = Vec::new();
  for distance in &distances {
    if distance.1 == smallest_distance {
      shortest_distances.push(distance.clone());
    }
  }
  if shortest_distances.len() == 0 {
    panic!("unexpected no shortest_distances, '{:?}' with smallest_distance = '{:?}'", distances, smallest_distance);
  }
  shortest_distances[0].1
}

#[derive(Debug)]
struct PathSegment {
  direction: char,
  distance: u32,
}

fn load_wire_path_from_comma_delimited_string(input_string: &str) -> Vec<PathSegment> {
  let items = input_string.split(",");
  let mut path_segments = Vec::new();
  for mut item in items {
    item = item.trim();
    if item.is_empty() {
      continue;
    }
    let first_char: char = item.as_bytes()[0] as char;
    if !first_char.is_alphabetic() {
      panic!("invalid path segment, expected an alphabetic character first, got '{}'", item);
    }
    let distance: u32 = item.get(1..).unwrap().parse::<u32>().unwrap();
    path_segments.push(PathSegment {direction: first_char, distance: distance});
  }
  path_segments
}

fn load_wire_paths_from_string(input_string: &str) -> (Vec<PathSegment>, Vec<PathSegment>) {
  let raw_lines = input_string.split("\n");
  let mut lines = Vec::new();
  for mut line in raw_lines {
    line = line.trim();
    if line.is_empty() {
      continue;
    }
    lines.push(line);
  }
  if lines.len() != 2 {
    panic!("invalid wire paths, expected two lines, one for each wire, got '{}'", lines.len());
  }
  let wire1_path = load_wire_path_from_comma_delimited_string(lines[0]);
  let wire2_path = load_wire_path_from_comma_delimited_string(lines[1]);
  (wire1_path, wire2_path)
}

fn load_wire_paths_from_file(input_file: &str) -> (Vec<PathSegment>, Vec<PathSegment>) {
  let mut file = File::open(input_file).unwrap();
  let mut content = String::new();
  file.read_to_string(&mut content).unwrap();
  load_wire_paths_from_string(&content)
}

#[cfg(test)]
mod tests {
  use super::*;

  // #[test]
  // fn examples_from_first_question() {
  //   let (wire1_path, wire2_path) = load_wire_paths_from_string(
  //     "R8,U5,L5,D3\nU7,R6,D4,L4");
  //   assert_eq!(6, find_distance_to_closest_intersection(&wire1_path, &wire2_path));

  //   let (wire1_path, wire2_path) = load_wire_paths_from_string(
  //     "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83");
  //   assert_eq!(159, find_distance_to_closest_intersection(&wire1_path, &wire2_path));

  //   let (wire1_path, wire2_path) = load_wire_paths_from_string(
  //     "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
  //   assert_eq!(135, find_distance_to_closest_intersection(&wire1_path, &wire2_path));
  // }

  #[test]
  fn examples_from_second_question() {
    let (wire1_path, wire2_path) = load_wire_paths_from_string(
      "R8,U5,L5,D3\nU7,R6,D4,L4");
    assert_eq!(30, find_distance_to_closest_intersection(&wire1_path, &wire2_path));

    let (wire1_path, wire2_path) = load_wire_paths_from_string(
      "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83");
    assert_eq!(610, find_distance_to_closest_intersection(&wire1_path, &wire2_path));

    let (wire1_path, wire2_path) = load_wire_paths_from_string(
      "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
    assert_eq!(410, find_distance_to_closest_intersection(&wire1_path, &wire2_path));
  }
}
