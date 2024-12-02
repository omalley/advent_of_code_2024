use std::cmp::Ordering;
use itertools::Itertools;
use smallvec::SmallVec;

fn parse_int(s: &str) -> Result<i32, String> {
  s.parse().map_err(|_| format!("Can't parse integer - '{s}'"))
}

type Row = SmallVec<[i32; 20]>;

fn parse_line(s: &str) -> Result<Row, String> {
  s.split_whitespace().map(parse_int).try_collect()
}

pub fn generator(input: &str) -> Vec<Row> {
  input.lines().map(parse_line).try_collect().expect("Can't parse input")
}

fn is_good(row: &Row) -> bool {
  if row.len() <= 1 {
    true
  } else if row[1] > row[0] {
    row.iter().tuple_windows().all(|(a, b)| (1..=3).contains(&(*b - *a)))
  } else {
    row.iter().tuple_windows().all(|(a, b)| (1..=3).contains(&(*a - *b)))
  }
}

#[derive(Clone,Debug,Eq,PartialEq)]
enum DeltaKind {
  Gain,
  Loss,
  Bad,
}

fn analyze_delta(x: i32) -> DeltaKind {
  match x {
    1..=3 => DeltaKind::Gain,
    -3..=-1 => DeltaKind::Loss,
    _ => DeltaKind::Bad,
  }
}

fn analyze_diffs(x: &[i32]) -> Option<DeltaKind> {
  x.iter().fold(None, |acc, v| match acc {
    Some(DeltaKind::Bad) => acc,
    Some(prev) =>
      if prev == analyze_delta(*v) { Some(prev) } else { Some(DeltaKind::Bad) },
    None => Some(analyze_delta(*v)),
  })
}

fn is_ok(row: &Row) -> bool {
  if row.len() <= 1 {
    true
  } else {
    // compute the deltas between
    let diffs: Vec<i32> = row.iter().tuple_windows().map(|(a, b)| *b - *a).collect();
    if analyze_diffs(&diffs[1..]) != Some(DeltaKind::Bad) { return true }
    if analyze_diffs(&diffs[..diffs.len()-1]) != Some(DeltaKind::Bad) { return true }
    for i in 0..diffs.len() {
      let mut copy = Vec::with_capacity(diffs.len() - 1);
      for j in 0..diffs.len()-1 {
        match j.cmp(&i) {
          Ordering::Less => { copy.push(diffs[j]) }
          Ordering::Equal => { copy.push(diffs[j] + diffs[j+1]) }
          Ordering::Greater => { copy.push(diffs[j + 1])}
        }
      }
      if analyze_diffs(&copy) != Some(DeltaKind::Bad) { return true }
    }
    false
  }
}

pub fn part1(input: &[Row]) -> usize {
  input.iter().filter(|v| is_good(v)).count()
}

pub fn part2(input: &[Row]) -> usize {
  input.iter().filter(|v| is_ok(v)).count()
}

#[cfg(test)]
mod tests {
  use crate::day2::{generator, part1, part2};

  const INPUT: &str =
"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

  #[test]
  fn test_part1() {
    let data = generator(INPUT);
    assert_eq!(2, part1(&data));
  }

  #[test]
  fn test_part2() {
    let data = generator(INPUT);
    assert_eq!(4, part2(&data));
  }
}
