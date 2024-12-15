use std::collections::VecDeque;
use ahash::AHashSet;
use array2d::Array2D;
use itertools::Itertools;

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
pub enum Side {
  Left, Right, Both,
}

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
pub enum FloorKind {
  Empty, Box(Side), Wall,
}

impl FloorKind {
  /// Should this count a box for scoring?
  fn is_box(self) -> bool {
    match self {
      FloorKind::Box(Side::Both) | FloorKind::Box(Side::Left) => true,
      _ => false,
    }
  }
}

type Position = u16;

#[derive(Clone,Debug,Eq,Hash,PartialEq)]
pub struct Coordinate {
  y: Position,
  x: Position,
}

impl Coordinate {
  fn step(&self, direction: Direction) -> Coordinate {
    match direction {
      Direction::North => Coordinate{y: self.y - 1, x: self.x},
      Direction::West => Coordinate{y: self.y, x: self.x - 1},
      Direction::South => Coordinate{y: self.y + 1, x: self.x},
      Direction::East => Coordinate{y: self.y, x: self.x + 1},
    }
  }
}

fn read_grid(input: &str) -> Result<(Array2D<FloorKind>, Coordinate), String> {
  let mut guard = None;
  let floor_vec: Vec<Vec<FloorKind>> = input.lines().enumerate()
      .map(|(y, line)|
        line.chars().enumerate()
            .map(|(x, ch)| match ch {
              '#' => Ok(FloorKind::Wall),
              '.' => Ok(FloorKind::Empty),
              'O' => Ok(FloorKind::Box(Side::Both)),
              '@' => {
                guard = Some(Coordinate{y: y as Position, x: x as Position});
                Ok(FloorKind::Empty)
              }
              _ => Err(format!("Invalid character '{}'", ch))})
            .try_collect())
      .try_collect()?;
  let floor = Array2D::from_rows(&floor_vec).map_err(|e| format!("Can't build floor - {e}"))?;
  Ok((floor, guard.ok_or("Can't find_guard")?))
}

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
pub enum Direction {
  North, West, South, East,
}

impl Direction {
  fn from_char(ch: char) -> Result<Direction, String> {
    match ch {
      '<' => Ok(Direction::West),
      '>' => Ok(Direction::East),
      '^' => Ok(Direction::North),
      'v' => Ok(Direction::South),
      _ => Err(format!("Invalid direction '{ch}'")),
    }
  }
}

#[derive(Clone,Debug)]
pub struct Grid {
  floor: Array2D<FloorKind>,
  guard: Coordinate,
}

#[derive(Clone,Debug)]
pub struct Problem {
  grid: Grid,
  instructions: Vec<Direction>,
}

impl Grid {
  /// Find the list of blocks to move. They should be moved in reverse order.
  fn plan_move(&self, location: &Coordinate, direction: Direction) -> Option<Vec<Coordinate>> {
    let mut result = Vec::with_capacity(20);
    let mut pending = VecDeque::with_capacity(20);
    let mut done = AHashSet::new();
    pending.push_front(location.step(direction));
    while let Some(location) = pending.pop_back() {
      if done.insert(location.clone()) {
        match self.floor.get(location.y as usize, location.x as usize) {
          Some(FloorKind::Empty) => {}
          Some(FloorKind::Box(side)) => {
            match (direction, side) {
              (Direction::East | Direction::West, _) | (_, Side::Both) => {},
              (_, Side::Left) => {
                let other = location.step(Direction::East);
                if !done.contains(&other) {
                  pending.push_back(other);
                }
              },
              (_, Side::Right) => {
                let other = location.step(Direction::West);
                if !done.contains(&other) {
                  pending.push_back(other);
                }
              },
            }
            pending.push_front(location.step(direction));
            result.push(location);
          }
          _ => { return None }
        }
      }
    }
    Some(result)
  }

  fn perform_commands(&mut self, instructions: &[Direction]) {
    for &instruction in instructions {
      if let Some(mut moving) = self.plan_move(&self.guard, instruction) {
        while let Some(from) = moving.pop() {
          let old_floor = self.floor.get(from.y as usize, from.x as usize).unwrap();
          let target = from.step(instruction);
          *self.floor.get_mut(target.y as usize, target.x as usize).unwrap() = old_floor.clone();
          *self.floor.get_mut(from.y as usize, from.x as usize).unwrap() = FloorKind::Empty;
        }
        self.guard = self.guard.step(instruction);
      }
    }
  }

  fn compute_gps(&self) -> usize {
    self.floor.rows_iter().enumerate()
        .map(|(y, row_itr)| row_itr.enumerate()
            .filter(|(_, val)| val.is_box())
            .map(|(x, _)| y * 100 + x)
            .sum::<usize>())
        .sum()
  }

  #[allow(dead_code)]
  fn display(&self) {
    for (y, row_itr) in self.floor.rows_iter().enumerate() {
      for (x, val) in row_itr.enumerate() {
        let ch = match val {
          _ if self.guard.x == x as Position && self.guard.y == y as Position => { '@' },
          FloorKind::Wall => { '#' },
          FloorKind::Empty => { '.' },
          FloorKind::Box(side) => match side {
            Side::Both => 'O',
            Side::Left => '[',
            Side::Right => ']',
          }
        };
        print!("{ch}");
      }
      println!();
    }
  }

  fn double_width(&self) -> Self {
    let mut floor = Array2D::filled_with(FloorKind::Empty, self.floor.num_rows(),
    self.floor.num_columns() * 2);
    for (y, row_iter) in self.floor.rows_iter().enumerate() {
      for (x, spot) in row_iter.enumerate() {
        match spot {
          FloorKind::Wall => {
            floor[(y, 2 * x)] = FloorKind::Wall;
            floor[(y, 2 * x + 1)] = FloorKind::Wall;
          },
          FloorKind::Box(_) => {
            floor[(y, 2 * x)] = FloorKind::Box(Side::Left);
            floor[(y, 2 * x + 1)] = FloorKind::Box(Side::Right);
          }
          _ => {}
        }
      }
    }
    let guard = Coordinate{y: self.guard.y, x: self.guard.x * 2};
    Grid{floor, guard}
  }
}

pub fn generator(input: &str) -> Problem {
  let (grid_str, instructions) = input.split_once("\n\n").unwrap();
  let (floor, guard) = read_grid(grid_str).expect("Can't parse floor");
  let instructions = instructions.chars().filter(|ch| !ch.is_whitespace())
      .map(Direction::from_char).try_collect().expect("Can't parse instructions");
  Problem{ grid: Grid{floor, guard}, instructions}
}

pub fn part1(input: &Problem) -> usize {
  let mut state = input.grid.clone();
  state.perform_commands(&input.instructions);
  state.compute_gps()
}

pub fn part2(input: &Problem) -> usize {
  let mut state = input.grid.double_width();
  state.perform_commands(&input.instructions);
  state.compute_gps()
}

#[cfg(test)]
mod tests {
  use super::{generator, part1, part2};

  const SMALL: &str =
"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";

  #[test]
  fn test_small_part1() {
    assert_eq!(2028, part1(&generator(SMALL)));
  }

  const INPUT: &str =
"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

  #[test]
  fn test_part1() {
    let data = generator(INPUT);
    assert_eq!(10092, part1(&data));
  }

  #[test]
  fn test_part2() {
    let data = generator(INPUT);
    assert_eq!(9021, part2(&data));
  }
}
