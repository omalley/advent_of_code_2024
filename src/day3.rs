use std::iter::Peekable;
use std::str::Chars;

#[derive(Clone,Copy,Debug,PartialEq)]
pub enum Command {
  Mul(i32, i32),
  Do,
  Dont,
}

fn parse_int(stream: &mut Peekable<Chars>) -> Option<i32> {
  let mut result = 0;
  for i in 0..3 {
    if let Some(peek) = stream.peek() {
      if peek.is_numeric() {
        result = result * 10 + stream.next().unwrap().to_digit(10).unwrap() as i32;
      }
    } else if i == 0 {
      return None;
    } else {
      break;
    }
  }
  Some(result)
}

fn consume_literal(stream: &mut Peekable<Chars>, lit: &str) -> bool {
  for ch in lit.chars() {
    if let Some(next) = stream.next() {
      if next != ch {
        return false;
      }
    }
  }
  true
}

fn next_command(stream: &mut Peekable<Chars>) -> Option<Command> {
  while let Some(ch) = stream.next() {
    match ch {
      // match mul(999,999)
      'm' => {
        if !consume_literal(stream, "ul(") { continue }
        if let Some(left) = parse_int(stream) {
          if !consume_literal(stream, ",") { continue }
          if let Some(right) = parse_int(stream) {
            if !consume_literal(stream, ")") { continue }
            return Some(Command::Mul(left, right));
          }
        }
      }
      // match do() and don't()
      'd' => {
        if !consume_literal(stream, "o") { continue }
        match stream.peek() {
          Some('(') => {
            if !consume_literal(stream, "()") { continue }
            return Some(Command::Do);
          }
          Some('n') => {
            if !consume_literal(stream, "n't()") { continue }
            return Some(Command::Dont);
          }
          _ => {}
        }
      }
      _ => {}
    }
  }
  None
}

pub fn generator(input: &str) -> Vec<Command> {
  let mut stream = input.chars().peekable();
  let mut result = Vec::new();
  while let Some(command) = next_command(&mut stream) {
    result.push(command);
  }
  result
}

pub fn part1(input: &[Command]) -> i32 {
  input.iter().map(|c| match c {
    Command::Mul(x, y) => x * y,
    _ => 0,
  }).sum()
}

pub fn part2(input: &[Command]) -> i32 {
  let mut result = 0;
  let mut enabled = true;
  for cmd in input {
    match cmd {
      Command::Mul(left, right) => {
        if enabled {
          result += left * right;
        }}
      Command::Do => { enabled = true; }
      Command::Dont => { enabled = false; }
    }
  }
  result
}

#[cfg(test)]
mod tests {
  use super::{generator, part1, part2};

  const INPUT: &str =
"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

  #[test]
  fn test_part1() {
    let data = generator(INPUT);
    assert_eq!(161, part1(&data));
  }

  const INPUT2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

  #[test]
  fn test_part2() {
    let data = generator(INPUT2);
    assert_eq!(48, part2(&data));
  }
}
