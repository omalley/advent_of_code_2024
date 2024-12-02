use std::ops::RangeInclusive;
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

const VALID: RangeInclusive<i32> = 1..=3;

fn is_good(row: &Row) -> bool {
  if row.len() <= 1 {
    true
  } else if row[1] > row[0] {
    row.iter().tuple_windows().all(|(a, b)| VALID.contains(&(*b - *a)))
  } else {
    row.iter().tuple_windows().all(|(a, b)| VALID.contains(&(*a - *b)))
  }
}

/// Is the row ok given that we drop the element at the given position?
fn is_good_with_drop(row: &Row, drop: usize) -> bool {
  // All rows with size 2 or less are valid if we drop one of them.
  if row.len() <= 2 {
    true
  } else {
    // Figure out the first two elements that we are keeping.
    let p0 = if drop == 0 { 1 } else { 0 };
    let p1 = if drop <= 1 { 2 } else { 1 };
    // Figure out the correct compare function for either growing or shrinking.
    let check = if row[p1] > row[p0] {
      |(a, b) : (&i32, &i32) | VALID.contains(&(*b - *a))
    } else {
      |(a, b) : (&i32, &i32) | VALID.contains(&(*a - *b))
    };
    // Ignoring the element to drop, check each pair of adjacent values.
    row.iter().enumerate()
        .filter_map(|(i, v)| if i == drop { None } else { Some(v) } )
        .tuple_windows().all(check)
  }
}

/// Is this row ok given that we drop one element?
fn is_ok(row: &Row) -> bool {
  // try each position to drop and if we find one, accept the Row.
  (0..row.len()).any(|drop| is_good_with_drop(row, drop))
}

pub fn part1(input: &[Row]) -> usize {
  input.iter().filter(|v| is_good(v)).count()
}

pub fn part2(input: &[Row]) -> usize {
  input.iter().filter(|v| is_ok(v)).count()
}

#[cfg(test)]
mod tests {
  use super::{generator, part1, part2};

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
