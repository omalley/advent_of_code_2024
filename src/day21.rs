use std::cmp::Ordering;
use std::iter;
use itertools::Itertools;
use smallvec::SmallVec;

type Position = i8;

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
struct Coordinate {
  x: Position,
  y: Position,
}

trait KeyPad: Sized {
  /// Find the button at the given coordinate.
  fn from_position(coordinate: Coordinate) -> Option<Self>;

  /// Find the position of the given button.
  fn position(&self) -> Coordinate;
}

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
pub enum TenKey {
  Seven, Eight, Nine,
  Four, Five, Six,
  One, Two, Three,
  Zero, Activate,
}

impl TenKey {
  fn from_char(ch: char) -> Result<TenKey, String> {
    match ch {
      '7' => Ok(TenKey::Seven),
      '8' => Ok(TenKey::Eight),
      '9' => Ok(TenKey::Nine),
      '4' => Ok(TenKey::Four),
      '5' => Ok(TenKey::Five),
      '6' => Ok(TenKey::Six),
      '1' => Ok(TenKey::One),
      '2' => Ok(TenKey::Two),
      '3' => Ok(TenKey::Three),
      '0' => Ok(TenKey::Zero),
      'A' => Ok(TenKey::Activate),
      _ => Err(format!("Invalid TenKey char '{}'", ch)),
    }
  }

  fn digit(&self) -> Option<usize> {
    match self {
      TenKey::Seven => Some(7),
      TenKey::Eight => Some(8),
      TenKey::Nine => Some(9),
      TenKey::Four => Some(4),
      TenKey::Five => Some(5),
      TenKey::Six => Some(6),
      TenKey::One => Some(1),
      TenKey::Two => Some(2),
      TenKey::Three => Some(3),
      TenKey::Zero => Some(0),
      _ => None,
    }
  }

  fn to_char(&self) -> char {
    match self {
      TenKey::Seven => '7',
      TenKey::Eight => '8',
      TenKey::Nine => '9',
      TenKey::Four => '4',
      TenKey::Five => '5',
      TenKey::Six => '6',
      TenKey::One => '1',
      TenKey::Two => '2',
      TenKey::Three => '3',
      TenKey::Zero => '0',
      TenKey::Activate => 'A',
    }
  }
}

impl KeyPad for TenKey {
  fn from_position(coordinate: Coordinate) -> Option<TenKey> {
    match coordinate.y {
      0 => match coordinate.x {
        0 => Some(TenKey::Activate),
        1 => Some(TenKey::Zero),
        _ => None,
      },
      1 => match coordinate.x {
        0 => Some(TenKey::Three),
        1 => Some(TenKey::Two),
        2 => Some(TenKey::One),
        _ => None,
      },
      2 => match coordinate.x {
        0 => Some(TenKey::Six),
        1 => Some(TenKey::Five),
        2 => Some(TenKey::Four),
        _ => None,
      },
      3 => match coordinate.x {
        0 => Some(TenKey::Nine),
        1 => Some(TenKey::Eight),
        2 => Some(TenKey::Seven),
        _ => None,
      },
      _ => None,
    }
  }

  fn position(&self) -> Coordinate {
    match self {
      TenKey::Seven => Coordinate{x: 2, y: 3},
      TenKey::Eight => Coordinate{x: 1, y: 3},
      TenKey::Nine => Coordinate{x: 0, y: 3},
      TenKey::Four => Coordinate{x: 2, y: 2},
      TenKey::Five => Coordinate{x: 1, y: 2},
      TenKey::Six => Coordinate{x: 0, y: 2},
      TenKey::One => Coordinate{x: 2, y: 1},
      TenKey::Two => Coordinate{x: 1, y: 1},
      TenKey::Three => Coordinate{x: 0, y: 1},
      TenKey::Zero => Coordinate{x: 1, y: 0},
      TenKey::Activate => Coordinate{x: 0, y: 0},
    }
  }
}

type Sequence = SmallVec<[TenKey; 10]>;

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
pub enum ArrowKey {
  Up, Activate,
  Left, Down, Right,
}

impl ArrowKey {
  fn to_char(&self) -> char {
    match self {
      ArrowKey::Up => '^',
      ArrowKey::Activate => 'A',
      ArrowKey::Down => 'V',
      ArrowKey::Left => '<',
      ArrowKey::Right => '>',
    }
  }

  /// Which vertical direction do we move to get from current to goal?
  fn vertical_move(current: Position, goal: Position) -> Option<Self> {
    match current.cmp(&goal) {
      Ordering::Less => Some(ArrowKey::Up),
      Ordering::Equal => None,
      Ordering::Greater => Some(ArrowKey::Down),
    }
  }

  /// Which horizontal direction do we move to get from current to goal?
  fn horizontal_move(current: Position, goal: Position) -> Option<Self> {
    match current.cmp(&goal) {
      Ordering::Less => Some(ArrowKey::Left),
      Ordering::Equal => None,
      Ordering::Greater => Some(ArrowKey::Right),
    }
  }
}


impl KeyPad for ArrowKey {
  fn from_position(coordinate: Coordinate) -> Option<Self> {
    match coordinate.y {
      0 => match coordinate.x {
        0 => Some(ArrowKey::Right),
        1 => Some(ArrowKey::Down),
        2 => Some(ArrowKey::Left),
        _ => None,
      },
      1 => match coordinate.x {
        0 => Some(ArrowKey::Activate),
        1 => Some(ArrowKey::Up),
        _ => None,
      },
      _ => None,
    }
  }

  fn position(&self) -> Coordinate {
    match self {
      ArrowKey::Up => Coordinate { x: 1, y: 1 },
      ArrowKey::Activate => Coordinate { x: 0, y: 1 },
      ArrowKey::Left => Coordinate { x: 2, y: 0 },
      ArrowKey::Down => Coordinate { x: 1, y: 0 },
      ArrowKey::Right => Coordinate { x: 0, y: 0 },
    }
  }
}

fn parse_line(s: &str) -> Result<Sequence, String> {
  Ok(s.chars().map(|ch| TenKey::from_char(ch)).try_collect()?)
}

pub fn generator(input: &str) -> Vec<Sequence> {
  input.lines().map(parse_line).try_collect().expect("Can't parse input")
}

/// Return the possibilities for pressing the given key.
/// Only the paths along direct routes are included.
fn plan_paths<T: KeyPad>(curr_key: T, goal_key: T) -> Vec<Vec<ArrowKey>> {
  let current = curr_key.position();
  let goal = goal_key.position();
  let mut result = Vec::new();
  // Go horizontal and then vertical.
  if let Some(dir) = ArrowKey::horizontal_move(current.x, goal.x) {
    // Only include the path if it avoids the missing key.
    if current.y == goal.y ||
        T::from_position(Coordinate{x: goal.x, y: current.y}).is_some() {
      let mut path = vec![dir; current.x.abs_diff(goal.x) as usize];
      if let Some(dir) = ArrowKey::vertical_move(current.y, goal.y) {
        path.extend(iter::once(dir).cycle().take(current.y.abs_diff(goal.y) as usize));
      }
      path.push(ArrowKey::Activate);
      result.push(path);
    }
  }
  // Go vertical first and then horizontal.
  if let Some(dir) = ArrowKey::vertical_move(current.y, goal.y) {
    // Only include the path if it avoids the missing key.
    if current.x == goal.x ||
        T::from_position(Coordinate{x: current.x, y: goal.y}).is_some() {
      let mut path = vec![dir; current.y.abs_diff(goal.y) as usize];
      if let Some(dir) = ArrowKey::horizontal_move(current.x, goal.x) {
        path.extend(iter::once(dir).cycle().take(current.x.abs_diff(goal.x) as usize));
      }
      path.push(ArrowKey::Activate);
      result.push(path);
    }
  }
  result
}

trait PadState {
  fn move_to(&self, goal: TenKey) -> Vec<Vec<ArrowKey>>;
}

struct TenKeyHand {
  current: TenKey,
}

impl TenKeyHand {
  fn new() -> Self {
    Self { current: TenKey::Activate }
  }
}

impl PadState for TenKeyHand {
  fn move_to(&self, goal: TenKey) -> Vec<Vec<ArrowKey>> {
    plan_paths(self.current, goal)
  }
}

struct ArrowHand<T: PadState> {
  upstream: T,
  current: ArrowKey,
}

impl<T: PadState> ArrowHand<T> {
  fn new(upstream: T) -> Self {
    Self { upstream, current: ArrowKey::Activate }
  }
}

impl<T: PadState> PadState for ArrowHand<T> {
  fn move_to(&self, goal: TenKey) -> Vec<Vec<ArrowKey>> {
   // self.upstream.move_to(goal).into_iter()
   //     .flat_map(|vec| vec.into_iter().map(|k| ))
   //     .collect()
    vec![]
  }
}

fn find_numeric(val: &Sequence) -> usize {
  val.iter().filter_map(|x| x.digit())
      .fold(0, |acc, x| acc * 10 + x)
}

pub fn part1(input: &[Sequence]) -> usize {
  0
}

pub fn part2(input: &[Sequence]) -> usize {
  0
}

#[cfg(test)]
mod tests {
  use super::{generator, part1, part2};

  const INPUT: &str =
"029A
980A
179A
456A
379A";

  #[test]
  fn test_part1() {
    let data = generator(INPUT);
    for line in &data {
      println!("line: {line:?}");
    }
    assert_eq!(0, part1(&data));
  }

  #[test]
  fn test_part2() {
    let data = generator(INPUT);
    assert_eq!(0, part2(&data));
  }
}
