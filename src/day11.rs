use ahash::AHashMap;

fn parse_int(s: &str) -> Result<u64, String> {
  s.parse().map_err(|_| format!("Can't parse integer - '{s}'"))
}

pub fn generator(input: &str) -> AHashMap<u64, usize> {
  let mut result: AHashMap<u64, usize> = AHashMap::default();
  for number in input.split_whitespace().map(parse_int) {
    let number = number.expect("Could not parse number");
    *result.entry(number).or_insert(0) += 1;
  }
  result
}

fn split_number(num: u64) -> Option<(u64,u64)> {
  let digits = num.ilog10() + 1;
  if digits % 2 == 0 {
    let pow10 = 10u64.pow(digits / 2);
    Some((num / pow10, num % pow10))
  } else {
    None
  }
}

fn blink(values: &mut AHashMap<u64, usize>) {
  let mut result = Vec::new();
  for (num, count) in values.iter() {
    if *num == 0 {
      result.push((1, *count));
    } else if let Some((left, right)) = split_number(*num) {
      result.push((left, *count));
      result.push((right, *count));
    } else {
      result.push((*num * 2024, *count));
    }
  }
  values.clear();
  for (num, count) in result {
    *values.entry(num).or_insert(0) += count;
  }
}

fn do_blinks(input: &AHashMap<u64, usize>, blinks: usize) -> usize {
  let mut work = input.clone();
  for _ in 0..blinks {
    blink(&mut work);
  }
  work.values().sum()
}

pub fn part1(input: &AHashMap<u64, usize>) -> usize {
  do_blinks(input, 25)
}

pub fn part2(input: &AHashMap<u64, usize>) -> usize {
  do_blinks(input, 75)
}

#[cfg(test)]
mod tests {
  use super::{generator, part1, part2, split_number};

  const INPUT: &str = "125 17";

  #[test]
  fn test_part1() {
    let data = generator(INPUT);
    assert_eq!(55312, part1(&data));
  }

  #[test]
  fn test_split() {
    assert_eq!(Some((12, 34)), split_number(1234));
    assert_eq!(None, split_number(12345));
  }

  #[test]
  fn test_part2() {
    let data = generator(INPUT);
    assert_eq!(65601038650482, part2(&data));
  }
}
