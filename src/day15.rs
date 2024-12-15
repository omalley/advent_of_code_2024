use array2d::Array2D;
use itertools::Itertools;

#[derive(Clone,Debug,Eq,PartialEq)]
pub enum FloorKind {
  Empty, Box, Wall,
}

type Position = u16;

#[derive(Clone,Debug,Eq,PartialEq)]
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
              'O' => Ok(FloorKind::Box),
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
  North,
  West,
  South,
  East,
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

  fn opposite(&self) -> Direction {
    match *self {
      Direction::North => Direction::South,
      Direction::West => Direction::East,
      Direction::South => Direction::North,
      Direction::East => Direction::West,
    }
  }
}

#[derive(Clone,Debug)]
pub struct Grid {
  floor: Array2D<FloorKind>,
  guard: Coordinate,
  instructions: Vec<Direction>,
}

impl Grid {
  fn find_space(&self, location: &Coordinate, direction: Direction) -> Option<Coordinate> {
    let mut result = location.clone();
    loop {
      result = result.step(direction);
      if let Some(flr) = self.floor.get(result.y as usize, result.x as usize) {
        match flr {
          FloorKind::Wall => return None,
          FloorKind::Empty => {return Some(result);},
          FloorKind::Box => {}
        }
      } else {
        return None;
      }
    }
  }

  fn perform_commands(&mut self, instructions: &[Direction]) {
    for &instruction in instructions {
      if let Some(dest) = self.find_space(&self.guard, instruction) {
        let mut loc = dest.clone();
        let backwards = instruction.opposite();
        self.guard = self.guard.step(instruction);
        while loc != self.guard {
          self.floor.set(loc.y as usize, loc.x as usize, FloorKind::Box).unwrap();
          loc = loc.step(backwards);
        }
        self.floor.set(loc.y as usize, loc.x as usize, FloorKind::Empty).unwrap();
      }
    }
  }

  fn compute_gps(&self) -> usize {
    self.floor.rows_iter().enumerate()
        .map(|(y, row_itr)| row_itr.enumerate()
            .filter(|(_, val)| **val == FloorKind::Box).map(|(x, _)| y * 100 + x).sum::<usize>())
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
          FloorKind::Box => { 'O' },
        };
        print!("{ch}");
      }
      println!();
    }
  }
}

pub fn generator(input: &str) -> Grid {
  let (grid_str, instructions) = input.split_once("\n\n").unwrap();
  let (floor, guard) = read_grid(grid_str).expect("Can't parse floor");
  let instructions = instructions.chars().filter(|ch| !ch.is_whitespace())
      .map(Direction::from_char).try_collect().expect("Can't parse instructions");
  Grid{floor, guard, instructions}
}

pub fn part1(input: &Grid) -> usize {
  let mut state = input.clone();
  state.perform_commands(&input.instructions);
  state.compute_gps()
}

pub fn part2(_input: &Grid) -> usize {
  0
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
    assert_eq!(999, part2(&data));
  }
}
