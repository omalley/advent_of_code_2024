use itertools::Itertools;

type Position = i64;

#[derive(Clone,Debug,Eq,PartialEq)]
pub struct Coordinate {
  y: Position,
  x: Position,
}

#[derive(Clone,Debug)]
pub struct Pushes {
  button_a: i64,
  button_b: i64,
}

impl Pushes {
  fn price(&self) -> i64 {
    self.button_a * 3 + self.button_b
  }
}

#[derive(Clone,Debug)]
pub struct Machine {
  button_a: Coordinate,
  button_b: Coordinate,
  goal: Coordinate,
}

impl Machine {
  fn solve(&self) -> Option<Pushes> {
    let top = self.button_a.y * self.goal.x - self.button_a.x * self.goal.y;
    let bottom = self.button_a.y * self.button_b.x - self.button_a.x * self.button_b.y;
    if bottom == 0 || top % bottom != 0 {
      None
    } else {
      let button_b = top / bottom;
      let top = self.goal.x - self.button_b.x * button_b;
      if self.button_a.x == 0 || top % self.button_a.x != 0 {
        None
      } else {
        Some(Pushes{button_a: top / self.button_a.x, button_b})
      }
    }
  }
}

fn parse_int(s: &str) -> Result<Position, String> {
  s.parse().map_err(|_| format!("Can't parse integer - '{s}'"))
}

fn parse_attribute(s: &str, attribute: &str) -> Result<Position, String> {
  let s = s.trim();
  let (att, value) = s.split_once('+')
      .or_else(|| s.split_once('='))
      .ok_or(format!("Can't split attribute '{}'", s))?;
  if att != attribute {
    return Err(format!("Attribute {attribute} doesn't match '{s}'"));
  }
  parse_int(value)
}

fn parse_line(s: &str) -> Result<Coordinate, String> {
  let (_, values) = s.split_once(": ")
      .ok_or(format!("Can't split line - '{s}'"))?;
  let (x_str, y_str) = values.split_once(",")
      .ok_or(format!("Can't split numbers - '{values}'"))?;
  let x = parse_attribute(x_str, "X")?;
  let y = parse_attribute(y_str, "Y")?;
  Ok(Coordinate{x, y})
}

fn parse_machine(s: &str) -> Result<Machine, String> {
  let lines: Vec<&str> = s.lines().collect();
  if lines.len() != 3 {
    return Err(format!("Can't parse machine - {s}"));
  }
  let button_a = parse_line(lines[0])?;
  let button_b = parse_line(lines[1])?;
  let goal = parse_line(lines[2])?;
  Ok(Machine{button_a, button_b, goal})
}

pub fn generator(input: &str) -> Vec<Machine> {
  input.split("\n\n").map(parse_machine).try_collect().expect("Can't parse input")
}

pub fn part1(input: &[Machine]) -> i64 {
  input.iter().filter_map(|m| m.solve()).map(|p| p.price()).sum()
}

fn part2_munge(machine: &Machine) -> Machine {
  let mut result = machine.clone();
  result.goal.x += 10000000000000;
  result.goal.y += 10000000000000;
  result
}

pub fn part2(input: &[Machine]) -> i64 {
  input.iter().filter_map(|m| part2_munge(m).solve()).map(|p| p.price()).sum()
}

#[cfg(test)]
mod tests {
  use super::{generator, part1, part2};

  const INPUT: &str =
"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

  #[test]
  fn test_part1() {
    let data = generator(INPUT);
    assert_eq!(480, part1(&data));
  }

  #[test]
  fn test_part2() {
    let data = generator(INPUT);
    assert_eq!(875318608908, part2(&data));
  }
}
