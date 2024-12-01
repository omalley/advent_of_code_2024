use std::collections::HashMap;
use std::iter::zip;
use itertools::Itertools;

fn parse_int(s: &str) -> Result<i32, String> {
  s.parse().map_err(|_| format!("Can't parse integer - '{s}'"))
}
pub fn generator(input: &str) -> Vec<(i32,i32)> {
  input.lines().map(|l| l.split_whitespace().map(parse_int)
      .collect::<Result<Vec<i32>,String>>())
      .map_ok(|v| (v[0], v[1]))
      .collect::<Result<Vec<(i32,i32)>,String>>()
      .expect("Can't parse input")
}

pub fn part1(input: &[(i32,i32)]) -> i32 {
  let (mut left, mut right): (Vec<i32>, Vec<i32>) = input.iter().copied().unzip();
  left.sort_unstable();
  right.sort_unstable();
  zip(left,right).map(|(l,r)| (l-r).abs()).sum()
}

pub fn part2(input: &[(i32,i32)]) -> i32 {
  let (left, mut right): (Vec<i32>, Vec<i32>) = input.iter().copied().unzip();
  right.sort_unstable();
  let counts: HashMap<i32, usize> = right.into_iter().dedup_with_count()
      .map(|(c,e)| (e, c)).collect();
  left.iter().filter_map(|l| counts.get(l).map(|r| l * *r as i32))
      .sum()
}

#[cfg(test)]
mod tests {
  use crate::day1::{generator, part1, part2};

  const INPUT: &str =
"3   4
4   3
2   5
1   3
3   9
3   3";

  #[test]
  fn test_part1() {
    let data = generator(INPUT);
    assert_eq!(11, part1(&data));
  }

  #[test]
  fn test_part2() {
    let data = generator(INPUT);
    assert_eq!(31, part2(&data));
  }
}
