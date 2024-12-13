use array2d::Array2D;
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
  floor: Array2D<Floor>,
  guard: Guard,
  bounds: Coordinate,
}

impl Grid {

  fn find_guard(floor: &Array2D<Floor>) -> Option<Guard> {
    floor.rows_iter().enumerate()
        .find_map(|(y, line)|
            line.enumerate()
                .find_map(|(x, flr)| match flr {
                  Floor::Guard(facing) =>
                    Some(Guard{position: Coordinate{x: x as i32, y: y as i32},
                    facing: *facing}),
                  _ => None,
                }))
  }

  fn get(&self, position: &Coordinate) -> Option<&Floor> {
    self.floor.get(position.y as usize, position.x as usize)
  }

  fn get_mut(&mut self, position: &Coordinate) -> Option<&mut Floor> {
    self.floor.get_mut(position.y as usize, position.x as usize)
  }

  pub fn from_string(input: &str) -> Result<Grid, String> {
    let array: Vec<Vec<Floor>> = input.lines()
        .map(|line| line.chars().map(Floor::from_char).try_collect()).try_collect()?;
    let floor: Array2D<Floor> = Array2D::from_rows(&array).map_err(|e| format!("{e}"))?;
    let bounds = Coordinate { x: floor.column_len() as i32, y: floor.row_len() as i32};
    let guard = Self::find_guard(&floor).ok_or("No guard found")?;
    Ok(Grid { floor, guard, bounds })
  }
}

pub fn generator(input: &str) -> Grid {
  Grid::from_string(input).expect("Can't parse input")
}

#[derive(Clone,Debug,Default)]
struct SquareState {
  stack: SmallVec<[Guard; 4]>,
}

struct WalkState {
  state: Array2D<SquareState>,
  current: Guard,
  square_count: usize,
}

impl WalkState {

  #[inline]
  fn get_mut(&mut self, position: &Coordinate) -> &mut SquareState {
    self.state.get_mut(position.y as usize,position.x as usize).unwrap()
  }

  fn from_grid(grid: &Grid) -> Self {
    let state = Array2D::filled_with(SquareState::default(),
                                     grid.bounds.y as usize, grid.bounds.x as usize);
    let current = grid.guard.clone();
    WalkState{state, current, square_count: 1}
  }

  /// Walk through the grid until either the path loops or it leaves the edge.
  fn walk_is_loop(&mut self, grid: &Grid) -> bool {
    loop {
      let forward_coordinate = self.current.position.step(self.current.facing);
      if let Some(forward_floor) = grid.get(&forward_coordinate) {
        if forward_floor.is_occupied() {
          self.current.turn_right();
        } else {
          // If we haven't been to this square, bump up the count.
          if self.get_mut(&forward_coordinate).stack.is_empty() && *forward_floor == Floor::Empty {
            self.square_count += 1;
          }
          let old = self.current.clone();
          if self.get_mut(&forward_coordinate).stack.contains(&old) {
            return true
          }
          self.get_mut(&forward_coordinate).stack.push(old);
          self.current.position = forward_coordinate;
        }
      } else {
        return false
      }
    }
  }

  fn pop(&mut self) -> Option<Coordinate> {
    let current = self.current.position.clone();
    if let Some(prev) = self.get_mut(&current).stack.pop() {
      self.current = prev;
      Some(current)
    } else {
      None
    }
  }

  fn place_block(&mut self, grid: &mut Grid, place: &Coordinate) -> bool {
    // It should be empty or the guard, but we can't place it on the guard.
    // If this is not the first time to this place, we can't test it now.
    if *grid.get(place).unwrap_or(&Floor::Full) != Floor::Empty || !self.get_mut(place).stack.is_empty() {
      return false
    }
    *(grid.get_mut(place).unwrap()) = Floor::Full;
    let orig_guard = self.current.clone();
    self.get_mut(&orig_guard.position).stack.push(orig_guard.clone());
    let is_loop = self.walk_is_loop(grid);
    // reset self
    while self.current != orig_guard {
      if self.pop().is_none() {
        panic!("Crap");
      }
    }
    *(grid.get_mut(place).unwrap()) = Floor::Empty;
    is_loop
  }
}

pub fn part1(input: &Grid) -> usize {
  let mut state = WalkState::from_grid(input);
  state.walk_is_loop(input);
  state.square_count
}

pub fn part2(input: &Grid) -> usize {
  let mut playground = input.clone();
  let mut state = WalkState::from_grid(&playground);
  assert!(!state.walk_is_loop(&playground), "shouldn't loop");
  let mut result = 0;
  while let Some(new_block) = state.pop() {
    if state.place_block(&mut playground, &new_block) {
      result += 1;
    }
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
