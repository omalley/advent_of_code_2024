use std::iter::zip;
use itertools::Itertools;
use smallvec::SmallVec;

fn parse_int(s: &str) -> Result<i32, String> {
  s.parse().map_err(|_| format!("Can't parse integer - '{s}'"))
}

fn parse_line(s: &str) -> Result<(i32,i32), String> {
  let words: SmallVec<[i32; 2]> = s.split_whitespace().map(parse_int).try_collect()?;
  if words.len() != 2 {return Err(format!("Line is wrong length - {s}"))}
  Ok((words[0], words[1]))
}

pub fn generator(input: &str) -> Vec<(i32,i32)> {
  input.lines().map(parse_line).try_collect().expect("Can't parse input")
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
  let counts: Vec<(i32, usize)> = right.into_iter().dedup_with_count()
      .map(|(c,e)| (e, c)).collect();
  // Go through the left and match it with the count on the right.
  left.iter().map(|l|
      match counts.binary_search_by(|probe| probe.0.cmp(l)) {
        Ok(i) => *l * (counts[i].1 as i32),
        _ => 0,
      }).sum()
}

#[cfg(test)]
mod tests {
  use super::{generator, part1, part2};

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
