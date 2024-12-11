use std::collections::HashSet;
use itertools::Itertools;
use smallvec::SmallVec;

#[derive(Clone,Copy,Debug,Eq,Hash,PartialEq)]
pub enum Direction{
  North,
  East,
  South,
  West,
}

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
pub enum Floor {
  Empty,
  Full,
  Guard(Direction),
}

impl Floor {
  fn is_occupied(&self) -> bool {
    *self == Floor::Full
  }

  fn from_char(ch: char) -> Result<Floor, String> {
    match ch {
      '.' => Ok(Floor::Empty),
      '#' => Ok(Floor::Full),
      '^' => Ok(Floor::Guard(Direction::North)),
      'v' => Ok(Floor::Guard(Direction::South)),
      '>' => Ok(Floor::Guard(Direction::East)),
      '<' => Ok(Floor::Guard(Direction::West)),
      _ => Err(format!("Invalid character '{}'", ch)),
    }
  }
}

#[derive(Clone,Debug,Eq,Hash,PartialEq)]
pub struct Coordinate {
  x: i32,
  y: i32,
}

impl Coordinate {
  fn step(&self, direction: Direction) -> Coordinate {
    match direction {
      Direction::North => Coordinate{x: self.x, y: self.y - 1},
      Direction::East => Coordinate{x: self.x + 1, y: self.y},
      Direction::South => Coordinate{x: self.x, y: self.y + 1},
      Direction::West => Coordinate{x: self.x - 1, y: self.y},
    }
  }
}

#[derive(Clone,Debug,Eq,Hash,PartialEq)]
struct Guard {
  position: Coordinate,
  facing: Direction,
}

impl Guard {
  fn turn_right(&mut self) {
    match self.facing {
      Direction::North => { self.facing = Direction::East; },
      Direction::East => { self.facing = Direction::South; },
      Direction::South => { self.facing = Direction::West; },
      Direction::West => { self.facing = Direction::North; },
    }
  }
}

#[derive(Clone,Debug)]
pub struct Grid {
  floor: Vec<Vec<Floor>>,
  guard: Guard,
  bounds: Coordinate,
}

impl Grid {

  fn find_guard(floor: &[Vec<Floor>]) -> Option<Guard> {
    floor.iter().enumerate()
        .find_map(|(y, line)|
            line.iter().enumerate()
                .find_map(|(x, flr)| match flr {
                  Floor::Guard(facing) =>
                    Some(Guard{position: Coordinate{x: x as i32, y: y as i32},
                    facing: *facing}),
                  _ => None,
                }))
  }

  fn get(&self, position: &Coordinate) -> Option<&Floor> {
    if (0..self.bounds.x).contains(&position.x) &&
        (0..self.bounds.y).contains(&position.y) {
      Some(&self.floor[position.y as usize][position.x as usize])
    } else {
      None
    }
  }

  fn get_mut(&mut self, position: &Coordinate) -> &mut Floor {
    &mut self.floor[position.y as usize][position.x as usize]
  }

  pub fn from_string(input: &str) -> Result<Grid, String> {
    let floor: Vec<Vec<Floor>> = input.lines()
        .map(|line| line.chars().map(Floor::from_char).try_collect()).try_collect()?;
    let bounds = Coordinate { x: floor[0].len() as i32, y: floor.len() as i32};
    let guard = Self::find_guard(&floor).ok_or("No guard found")?;
    Ok(Grid { floor, guard, bounds })
  }

  fn walk(&self, guard: &mut Guard) -> Option<usize> {
    let forward_spot = guard.position.step(guard.facing);
    if self.get(&forward_spot).unwrap_or(&Floor::Empty).is_occupied() {
      guard.turn_right();
      Some(0)
    } else {
      guard.position = forward_spot;
      if (0..self.bounds.x).contains(&guard.position.x) &&
          (0..self.bounds.y).contains(&guard.position.y) {
        Some(1)
      } else {
        None
      }
    }
  }

  fn is_loop(&self) -> bool {
    let mut guard = self.guard.clone();
    let mut covered: HashSet<Guard> = HashSet::new();
    while let Some(steps) = self.walk(&mut guard) {
      if steps > 0 && !covered.insert(guard.clone()) {
        return true
      }
    }
    false
  }
}

pub fn generator(input: &str) -> Grid {
  Grid::from_string(input).expect("Can't parse input")
}

#[derive(Debug,Default)]
struct SquareState {
  stack: SmallVec<[Guard; 4]>,
}

struct WalkState {
  state: Vec<Vec<SquareState>>,
  current: Guard,
  square_count: usize,
}

impl WalkState {

  fn get_mut(&mut self, position: &Coordinate) -> &mut SquareState {
    &mut self.state[position.y as usize][position.x as usize]
  }

  fn from_grid(grid: &Grid) -> Self {
    let mut state = vec![vec![SquareState::default(); grid.bounds.x as usize];
                    grid.bounds.y as usize];
    let current = grid.guard.clone();
    state[current.position.y as usize][current.position.x as usize].stack.push(current.clone());
    WalkState{state, current, square_count: 1}
  }

  fn step(&mut self) {

  }
}

pub fn part1(input: &Grid) -> usize {
  let mut guard = input.guard.clone();
  let mut places: HashSet<Coordinate> = HashSet::new();
  places.insert(guard.position.clone());
  while let Some(steps) = input.walk(&mut guard) {
    if steps > 0 {
      places.insert(guard.position.clone());
    }
  }
  places.len()
}

fn get_guarded_locations(input: &Grid) -> HashSet<Coordinate> {
  let mut guard = input.guard.clone();
  let mut places: HashSet<Coordinate> = HashSet::new();
  while let Some(steps) = input.walk(&mut guard) {
    if steps > 0 {
      places.insert(guard.position.clone());
    }
  }
  places
}

pub fn part2(input: &Grid) -> usize {
  let mut playground = input.clone();
  // Get the places the guard goes other than the start location
  let mut guarded = get_guarded_locations(input);
  guarded.remove(&input.guard.position);
  let mut result = 0;
  for new_obstruction in guarded {
    *playground.get_mut(&new_obstruction) = Floor::Full;
    if playground.is_loop() {
      result += 1;
    }
    *playground.get_mut(&new_obstruction) = Floor::Empty;
  }
  result
}

#[cfg(test)]
mod tests {
  use super::{generator, part1, part2};

  const INPUT: &str =
"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

  #[test]
  fn test_part1() {
    let data = generator(INPUT);
    assert_eq!(41, part1(&data));
  }

  #[test]
  fn test_part2() {
    let data = generator(INPUT);
    assert_eq!(6, part2(&data));
  }
}
