use itertools::Itertools;
use smallvec::SmallVec;

pub type Number = i64;

fn parse_int(s: &str) -> Result<Number, String> {
  s.parse().map_err(|_| format!("Can't parse integer - '{s}'"))
}

#[derive(Debug)]
pub struct Row {
  target: Number,
  inputs: SmallVec<[Number;16]>,
}

fn parse_line(line: &str) -> Result<Row, String> {
  let (target_str, inputs_str) = line.split_once(':')
      .ok_or(format!("Can't find separator: '{line}'"))?;
  let target = parse_int(target_str)?;
  let inputs = inputs_str.split_whitespace()
      .map(parse_int).try_collect()?;
  Ok(Row { target, inputs })
}

pub fn generator(input: &str) -> Vec<Row> {
  input.lines().map(parse_line).try_collect().expect("Can't parse input")
}

fn subtract(right: Number, result: Number) -> Option<Number> {
  Some(result - right)
}

fn divide(right: Number, result: Number) -> Option<Number> {
  if right != 0 && result % right == 0 {
    Some(result / right)
  } else {
    None
  }
}

fn split(right: Number, result: Number) -> Option<Number> {
  let pow10 = (10 as Number).saturating_pow(right.ilog10() + 1);
  if result % pow10 == right {
    Some(result / pow10)
  } else {
    None
  }
}

fn has_solution<const HAS_CONCAT:bool>(inputs: &[Number], result: Number) -> bool {
  let len = inputs.len();
  match len {
    0 => false,
    1 => result == inputs[0],
    2 => {
      let eqls = |r| r == inputs[0];
      (HAS_CONCAT && split(inputs[1], result).is_some_and(eqls)) ||
          divide(inputs[1], result).is_some_and(eqls) ||
          subtract(inputs[1], result).is_some_and(eqls)
    },
    _ => {
      let recurse =
          |r| has_solution::<HAS_CONCAT>(&inputs[..len - 1], r);
      (HAS_CONCAT && split(inputs[len - 1], result).is_some_and(recurse)) ||
          divide(inputs[len - 1], result).is_some_and(recurse) ||
          subtract(inputs[len - 1], result).is_some_and(recurse)
    }

  }
}

fn solvable<const HAS_CONCAT:bool>(row: &Row) -> bool {
  if row.inputs.is_empty() {
    false
  } else if *row.inputs.iter().min().unwrap() < 1 {
    panic!("not handling negative numbers")
  } else {
    let result = has_solution::<HAS_CONCAT>(&row.inputs, row.target);
    result
  }
}

pub fn part1(input: &[Row]) -> Number {
  input.iter().filter(|&r| solvable::<false>(r)).map(|row| row.target).sum()
}

pub fn part2(input: &[Row]) -> Number {
  input.iter().filter(|&r| solvable::<true>(r)).map(|row| row.target).sum()
}

#[cfg(test)]
mod tests {
  use super::{generator, part1, part2};

  const INPUT: &str =
"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

  #[test]
  fn test_part1() {
    let data = generator(INPUT);
    assert_eq!(3749, part1(&data));
  }

  #[test]
  fn test_part2() {
    let data = generator(INPUT);
    assert_eq!(11387, part2(&data));
  }
}
