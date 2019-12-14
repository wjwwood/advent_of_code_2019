use std::collections::HashMap;

fn main() {
    let tmp = "
..#..###....#####....###........#
.##.##...#.#.......#......##....#
#..#..##.#..###...##....#......##
..####...#..##...####.#.......#.#
...#.#.....##...#.####.#.###.#..#
#..#..##.#.#.####.#.###.#.##.....
#.##...##.....##.#......#.....##.
.#..##.##.#..#....#...#...#...##.
.#..#.....###.#..##.###.##.......
.##...#..#####.#.#......####.....
..##.#.#.#.###..#...#.#..##.#....
.....#....#....##.####....#......
.#..##.#.........#..#......###..#
#.##....#.#..#.#....#.###...#....
.##...##..#.#.#...###..#.#.#..###
.#..##..##...##...#.#.#...#..#.#.
.#..#..##.##...###.##.#......#...
...#.....###.....#....#..#....#..
.#...###..#......#.##.#...#.####.
....#.##...##.#...#........#.#...
..#.##....#..#.......##.##.....#.
.#.#....###.#.#.#.#.#............
#....####.##....#..###.##.#.#..#.
......##....#.#.#...#...#..#.....
...#.#..####.##.#.........###..##
.......#....#.##.......#.#.###...
...#..#.#.........#...###......#.
.#.##.#.#.#.#........#.#.##..#...
.......#.##.#...........#..#.#...
.####....##..#..##.#.##.##..##...
.#.#..###.#..#...#....#.###.#..#.
............#...#...#.......#.#..
.........###.#.....#..##..#.##...
";
//   let tmp = "
// .#....#####...#..
// ##...##.#####..##
// ##...#...#.#####.
// ..#.....#...###..
// ..#.#.....#....##
// ";
//   let tmp = "
// .#..##.###...#######
// ##.############..##.
// .#.######.########.#
// .###.#######.####.#.
// #####.##.#.##.###.##
// ..#####..#.#########
// ####################
// #.####....###.#.#.##
// ##.#################
// #####.##.###..####..
// ..######..##.#######
// ####.##.####...##..#
// .#####..#.######.###
// ##...#.##########...
// #.##########.#######
// .####.#.###.###.#.##
// ....##.##.###..#####
// .#.#.###########.###
// #.#.#.#####.####.###
// ###.##.####.##.#..##
// ";
  let mut map = parse_asteroid_map_from_string(tmp);
  // panic!("{:?}", map);
  count_observable_asteroids_from_each_asteroid(&mut map);

  {
    let mut map_str: String = "".to_string();
    for i in 0..map.rows {
      for j in 0..map.cols {
        match map.data[j + i * map.cols] {
          Cell::Empty => map_str += "   .",
          Cell::Asteroid(observable_asteroids) => map_str += &format!("{:>4}", observable_asteroids),
        }
      }
      map_str += "\n";
    }
    println!("{}", map_str);
  }

  let mut max: usize = 0;
  let mut best_location = (0, 0);
  for (i, cell) in map.data.iter().enumerate() {
    let x = i % map.cols;
    let y = i / map.cols;
    match cell {
      Cell::Empty => continue,
      Cell::Asteroid(n) => max = if *n > max {best_location = (x, y); *n} else {max},
    }
  }
  println!("Maximum number of visible asteroids from best location ({}, {}): {:?}", best_location.0, best_location.1, max);
  let destruction_order = calculate_destruction_order(&map, best_location, 0);
  if destruction_order.len() >= 200 {
    println!("{:?}", destruction_order[199].0 * 100 + destruction_order[199].1);
  }

  let mut map_str: String = "".to_string();
  for i in 0..map.rows {
    for j in 0..map.cols {
      match map.data[j + i * map.cols] {
        Cell::Empty => map_str += "   .",
        Cell::Asteroid(_) => map_str += "   .",
      }
    }
    map_str += "\n";
  }
  let coord_to_index = |coord: (usize, usize)| -> usize {
    (coord.1 * map.cols * 4) + (coord.0 * 4) + coord.1
  };
  map_str.replace_range(coord_to_index(best_location)..(coord_to_index(best_location) + 4), "   X");
  for i in 0..destruction_order.len() {
    let coord = destruction_order[i];
    // map_str.replace_range(coord_to_index(coord)..(coord_to_index(coord) + 4), &format!("{:>4}", (i + 1) % 9));
    map_str.replace_range(coord_to_index(coord)..(coord_to_index(coord) + 4), &format!("{:>4}", i + 1));
  }
  println!("{}", map_str);

  // let destruction_order = calculate_destruction_order(&map, (8, 3), 0);
  // println!("{:?}", destruction_order[9].0 * 100 + destruction_order[9].1);
  // let mut tmp2 = "".to_string();
  // for line in tmp.split("\n") {
  //   let foo = line.trim();
  //   if foo.is_empty() {
  //     continue;
  //   }
  //   tmp2 += &(foo.to_string() + "\n");
  // }
  // tmp2.replace_range(coord_to_index((8, 3))..(coord_to_index((8, 3)) + 1), "X");
  // for i in 0..9 {
  //   let x = destruction_order[i];
  //   println!("{:?}", (x, coord_to_index(x), map.cols));
  //   tmp2.replace_range(coord_to_index(x)..(coord_to_index(x) + 1), &(i + 1).to_string());
  // }
  // println!("{}\n", tmp);
  // println!("{}", tmp2);
}

fn compute_angle(origin: (usize, usize), point: (usize, usize)) -> f64 {
  let origin_rotated = (-1.0 * origin.1 as f64, origin.0 as f64);
  let point_rotated = (-1.0 * point.1 as f64, point.0 as f64);
  let relative_x = point_rotated.0 - origin_rotated.0;
  let relative_y = point_rotated.1 - origin_rotated.1;
  relative_x.atan2(relative_y).to_degrees()
}

fn distance(a: (usize, usize), b: (usize, usize)) -> i64 {
  let (x1, x2, y1, y2) = (a.0 as f64, a.1 as f64, b.0 as f64, b.1 as f64);
  (((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt() * 1_000_000_000.0) as i64
}

fn calculate_angles(map: &Map, source: (usize, usize)) -> HashMap<i64, HashMap<i64, (usize, usize)>> {
  let mut angles = HashMap::new();

  for (i, cell) in map.data.iter().enumerate() {
    match cell {
      Cell::Empty => continue,
      Cell::Asteroid(_) => {},
    }

    let x = i % map.cols;
    let y = i / map.cols;

    if source == (x, y) {
      continue;
    }

    // compute the counter-clockwise angle
    let mut angle = compute_angle(source, (x, y));
    // rotate 90 degrees
    angle -= 91.0;
    if angle < 0.0 {
      angle += 360.0;
    }
    // println!("{:?}", (source, (x, y), angle));
    // convert to scaled integer
    let scaled_angle: i64 = (angle * 1_000_000_000.0) as i64;

    // println!("insert({}, {:?})", distance(source, (x, y)), (x, y));
    angles.entry(scaled_angle).or_insert(HashMap::new()).insert(distance(source, (x, y)), (x, y));
  }

  angles
}

fn calculate_destruction_order(
  original_map: &Map,
  laser_location: (usize, usize),
  limit: usize
) -> Vec<(usize, usize)>
{
  let mut destroyed_asteroid_coordinates = Vec::new();

  // calculate the angle to each asteroid from the laser
  let mut angles = calculate_angles(&original_map, laser_location);
  let mut ordered_angles = Vec::new();
  {
    for (key, _) in angles.iter() {
      ordered_angles.push(key.clone());
    }
  }
  ordered_angles.sort_unstable();
  ordered_angles.reverse();

  // rotate the laser through each pass until all asteroids have been destroyed
  let mut index = 0;
  loop {
    // iterate through the angles, destroying one asteroid each time (the closest)
    for scaled_angle in &ordered_angles {
      if !angles.contains_key(scaled_angle) {
        continue;
      }
      let asteroids_at_angle_by_distance = angles.get_mut(scaled_angle).unwrap();
      let closest = *asteroids_at_angle_by_distance.keys().min().unwrap();
      let closest_point = asteroids_at_angle_by_distance.remove(&closest).unwrap();
      if asteroids_at_angle_by_distance.is_empty() {
        angles.remove(scaled_angle);
      }
      destroyed_asteroid_coordinates.push(closest_point);
      index += 1;

      if index == limit {
        return destroyed_asteroid_coordinates
      }
    }

    if angles.is_empty() {
      break;
    }
  }

  destroyed_asteroid_coordinates
}

#[derive(Debug)]
enum Cell {
  Asteroid(/*number_of_observable_asteroids:*/ usize),
  Empty,
}

#[derive(Debug)]
struct Map {
  data: Vec<Cell>,
  rows: usize,
  cols: usize,
}

impl Map {
  fn new() -> Map {
    Map {data: Vec::new(), rows: 0, cols: 0}
  }
}

fn count_observable_asteroids_from_each_asteroid(map: &mut Map) {
  let mut temp: Vec<(usize, usize)> = Vec::new();
  for (i, cell) in map.data.iter().enumerate() {
    temp.push((i, match cell {
      Cell::Empty => continue,
      Cell::Asteroid(_) => {
        let x = i % map.cols;
        let y = i / map.cols;
        {
          count_observable_asteroids_from_coordinate(&map, (x, y))
        }
      }
    }));
  }
  for (i, observable_asteroids) in temp {
    assert!(i < map.data.len());
    match map.data[i] {
      Cell::Empty => panic!("cell unexpectedly empty"),
      Cell::Asteroid(ref mut x) => *x = observable_asteroids,
    }
  }
}

fn count_observable_asteroids_from_coordinate(map: &Map, coordinate: (usize, usize)) -> usize {
  let mut observable_asteroids = 0;

  let mut debug_map = String::new();
  for (i, cell) in map.data.iter().enumerate() {
    if i % map.rows == 0 {
      debug_map += "\n";
    }
    match cell {
      Cell::Empty => {
        debug_map += " .";
        continue;
      },
      _ => {},
    };
    let x = i % map.cols;
    let y = i / map.cols;
    if (x, y) == coordinate {
      debug_map += " *";
      continue;
    }

    let colinear = |a: (f32, f32), b: (f32, f32), c: (f32, f32)| -> bool {
      let area =
        a.0 * (b.1 - c.1) +
        b.0 * (c.1 - a.1) +
        c.0 * (a.1 - b.1);
      area == 0.0
    };
    assert!(colinear((1.0, 0.0), (2.0, 2.0), (3.0, 4.0)));
    let between = |target: (f32, f32), a: (f32, f32), b: (f32, f32)| -> bool {
      assert!(!(a.0 == b.0 && a.1 == b.1));
      let x_inside = (target.0 >= a.0 && target.0 <= b.0) || (target.0 <= a.0 && target.0 >= b.0);
      let y_inside = (target.1 >= a.1 && target.1 <= b.1) || (target.1 <= a.1 && target.1 >= b.1);
      // println!("{:?}, {:?}, {:?}: {:?} {:?}", target, a, b, x_inside, y_inside);
      x_inside && y_inside
    };
    assert!(between((1.0, 1.0), (0.0, 0.0), (2.0, 2.0)));
    assert!(between((1.0, 1.0), (2.0, 2.0), (0.0, 0.0)));
    assert!(between((2.0, 2.0), (2.0, 2.0), (0.0, 0.0)));
    assert!(between((2.0, 1.0), (2.0, 2.0), (0.0, 0.0)));
    assert!(between((1.0, 2.0), (2.0, 2.0), (0.0, 0.0)));
    assert!(!between((3.0, 1.0), (2.0, 2.0), (0.0, 0.0)));
    let mut obstructed = false;

    for (j, cell) in map.data.iter().enumerate() {
      match cell {
        Cell::Empty => continue,
        _ => {},
      }
      let xx = j % map.cols;
      let yy = j / map.cols;
      let point = (xx, yy as f32);
      if point == (x, y as f32) || point == (coordinate.0, coordinate.1 as f32) {
        continue;
      }
      if colinear((point.0 as f32, point.1), (x as f32, y as f32), (coordinate.0 as f32, coordinate.1 as f32)) {
        if between((point.0 as f32, point.1), (x as f32, y as f32), (coordinate.0 as f32, coordinate.1 as f32)) {
          debug_map += " x";
          obstructed = true;
          break;
        }
      }
    }
    if !obstructed {
      debug_map += " o";
      observable_asteroids += 1;
    }
  }
  // println!("{}\n", debug_map);

  observable_asteroids
}

fn parse_asteroid_map_from_string(input_string: &str) -> Map {
  let mut map = Map::new();

  let mut trimmed_lines = Vec::new();
  for line in input_string.split("\n") {
    let trimmed_line = line.trim();
    if !trimmed_line.is_empty() {
      trimmed_lines.push(trimmed_line);
    }
  }

  assert!(!trimmed_lines.is_empty());

  map.cols = trimmed_lines[0].len();
  for line in &trimmed_lines {
    assert_eq!(map.cols, line.len());
    map.rows += 1;
    for character in line.chars() {
      map.data.push(match character {
        '#' => Cell::Asteroid(0),
        '.' => Cell::Empty,
        _ => panic!("unexpected character '{}' in map, expected one of '#' or '.'", character),
      });
    }
  }

  map
}
