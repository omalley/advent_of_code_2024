use std::cmp::Ordering;
use itertools::Itertools;

type Position = i64;

fn parse_int(s: &str) -> Result<Position, String> {
  s.parse().map_err(|_| format!("Can't parse integer - '{s}'"))
}

#[derive(Clone,Debug)]
pub struct Coordinate {
  y: Position,
  x: Position,
}

impl Coordinate {
  fn from_str(s: &str) -> Result<Self, String> {
    let (_, values) = s.split_once('=')
        .ok_or(format!("Can't find '=' in string '{s}'"))?;
    let (x_str,y_str) = values.split_once(",")
        .ok_or(format!("Can't find , in {values}"))?;
    Ok(Coordinate{x: parse_int(x_str)?, y: parse_int(y_str)?})
  }
}

#[derive(Clone,Debug)]
pub struct Robot {
  location: Coordinate,
  velocity: Coordinate,
}

impl Robot {
  const BOARD_WIDTH: Position = 101;
  const BOARD_HEIGHT: Position = 103;

  fn from_str(s: &str) -> Result<Self, String> {
    let (loc_str, vel_str) = s.split_once(" ")
        .ok_or("Can't split line {s}")?;
    let location = Coordinate::from_str(loc_str)?;
    let velocity = Coordinate::from_str(vel_str)?;
    Ok(Robot{location: location, velocity: velocity})
  }

  fn move_forward(&mut self, steps: i64, width: Position, height: Position) {
    self.location.x = (self.location.x + self.velocity.x * steps).rem_euclid(width);
    self.location.y = (self.location.y + self.velocity.y * steps).rem_euclid(height);
  }

  fn quadrant(&self, width: Position, height: Position) -> Option<usize> {
    let base = match self.location.y.cmp(&(height / 2)) {
      Ordering::Less => 0,
      Ordering::Equal => return None,
      Ordering::Greater => 2,
    };
    match self.location.x.cmp(&(width / 2)) {
      Ordering::Less => Some(base),
      Ordering::Equal => None,
      Ordering::Greater => Some(base + 1),
    }
  }
}

pub fn generator(input: &str) -> Vec<Robot> {
  input.lines().map(Robot::from_str).try_collect().expect("Can't parse input")
}

fn score(robots: &[Robot], width: Position, height: Position) -> usize {
  let mut counts = [0usize; 4];
  robots.iter().filter_map(|r| r.quadrant(width, height))
      .for_each(|quadrant| counts[quadrant] += 1);
  counts.iter().product()
}

pub fn part1(input: &[Robot]) -> usize {
  let mut working = input.to_vec();
  working.iter_mut().for_each(|r|
      r.move_forward(100, Robot::BOARD_WIDTH, Robot::BOARD_HEIGHT));
  score(&working, Robot::BOARD_WIDTH, Robot::BOARD_HEIGHT)
}

fn tree_filter(robot: &Robot, width: Position, height: Position) -> bool {
  robot.location.y * width >= height * (2 * robot.location.x - width).abs()
}

pub fn part2(input: &[Robot]) -> usize {
  let mut working = input.to_vec();
  let goal = input.len() * 75 / 100;
  let mut steps = 0;
  while working.iter()
      .filter(|r| tree_filter(r, Robot::BOARD_WIDTH, Robot::BOARD_HEIGHT))
      .count() < goal {
    steps += 1;
    working.iter_mut().for_each(|r|
        r.move_forward(1, Robot::BOARD_WIDTH, Robot::BOARD_HEIGHT));
  }
  /*
  let mut display = [[' '; Robot::BOARD_WIDTH as usize]; Robot::BOARD_HEIGHT as usize];
  working.iter_mut().for_each(|r|
      display[r.location.y as usize][r.location.x as usize] = '#');
  for row in display {
    for c in row {
      print!("{}", c);
    }
    println!();
  } */
  steps
}

#[cfg(test)]
mod tests {
  use super::{generator, score};

  const INPUT: &str =
"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

  #[test]
  fn test_part1() {
    let mut robots = generator(INPUT).clone();
    robots.iter_mut().for_each(|r|
        r.move_forward(100, 11, 7));
    assert_eq!(12, score(&robots, 11, 7))
  }
}
