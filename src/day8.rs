use itertools::Itertools;
use smallvec::SmallVec;
use std::collections::HashSet;

type Position = i32;

#[derive(Clone,Copy,Debug,Eq,Hash,Ord,PartialEq,PartialOrd)]
pub struct Coordinate {
  x: Position,
  y: Position,
}

#[derive(Clone,Debug,Eq,Ord,PartialEq,PartialOrd)]
struct AntennaLocation {
  frequency: char,
  location: Coordinate,
}

#[derive(Clone,Debug)]
pub struct Antenna {
  #[allow(dead_code)]
  frequency: char,
  locations: Vec<Coordinate>,
}

#[derive(Clone,Debug)]
pub struct Grid {
  antenna: Vec<Antenna>,
  rows: Position,
  columns: Position,
}

type CoordinateList = SmallVec<[Coordinate; 32]>;

impl Grid {
  fn in_bounds(&self, coordinate: Coordinate) -> bool {
    (0..self.columns).contains(&coordinate.x) && (0..self.rows).contains(&coordinate.y)
  }

  fn find_antinodes(&self, left: Coordinate, right: Coordinate) -> CoordinateList {
    let mut result = CoordinateList::new();
    let x_delta = left.x - right.x;
    let y_delta = left.y - right.y;
    result.push(Coordinate {x: left.x + x_delta, y: left.y + y_delta });
    result.push(Coordinate {x: right.x - x_delta, y: right.y - y_delta});
    result.retain(|coord| self.in_bounds(*coord));
    result
  }

  fn find_all_antinodes(&self, left: Coordinate, right: Coordinate) -> CoordinateList {
    let mut result = CoordinateList::new();
    let x_delta = left.x - right.x;
    let y_delta = left.y - right.y;
    let mut antinode = Coordinate{x: left.x, y: left.y};
    while self.in_bounds(antinode) {
      result.push(antinode);
      antinode.x += x_delta;
      antinode.y += y_delta;
    }
    let mut antinode = Coordinate{x: right.x, y: right.y};
    while self.in_bounds(antinode) {
      result.push(antinode);
      antinode.x -= x_delta;
      antinode.y -= y_delta;
    }
    result
  }

}

pub fn generator(input: &str) -> Grid {
  let mut raw = Vec::new();
  let mut rows = 0;
  let mut columns = 0;
  for (y, line) in input.lines().enumerate() {
    rows += 1;
    if rows == 1 {
      columns = line.chars().count() as Position;
    }
    for (x, c) in line.chars().enumerate() {
      if c != '.' {
        raw.push(AntennaLocation{frequency: c,
          location: Coordinate{x: x as Position, y:y as Position }});
      }
    }
  }
  raw.sort_unstable();
  let mut antenna = Vec::new();
  for (frequency, chunk) in &raw.into_iter().chunk_by(|elt| elt.frequency) {
    antenna.push(Antenna{frequency, locations: chunk.map(|a| a.location).collect()})
  }
  Grid{antenna, rows, columns}
}

pub fn part1(input: &Grid) -> usize {
  let mut antinodes: HashSet<Coordinate> = HashSet::new();
  for antenna in &input.antenna {
    for (left, right) in antenna.locations.iter().tuple_combinations() {
      antinodes.extend(input.find_antinodes(*left, *right));
    }
  }
  antinodes.len()
}

pub fn part2(input: &Grid) -> usize {
  let mut antinodes: HashSet<Coordinate> = HashSet::new();
  for antenna in &input.antenna {
    for (left, right) in antenna.locations.iter().tuple_combinations() {
      antinodes.extend(input.find_all_antinodes(*left, *right));
    }
  }
  antinodes.len()
}

#[cfg(test)]
mod tests {
  use super::{generator, part1, part2};

  const INPUT: &str =
"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

  const SIMPLE: &str =
"..........
..........
..........
....a.....
........a.
.....a....
..........
..........
..........
..........";

  #[test]
  fn test_part1() {
    let data = generator(INPUT);
    assert_eq!(14, part1(&data));
  }

  #[test]
  fn test_part2() {
    let data = generator(INPUT);
    assert_eq!(34, part2(&data));
  }
}
