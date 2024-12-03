pub fn generator(input: &str) -> String {
  input.to_string()
}

fn parse_int(s: &str) -> Result<i32, String> {
  s.parse().map_err(|_| format!("Can't parse integer - '{s}'"))
}

pub fn part1(input: &str) -> i32 {
  let mul_pattern = regex_static::static_regex!(r"mul\((\d{1,3}),(\d{1,3})\)");
  mul_pattern.captures_iter(input).map(|cap| {
    let (_, [left, right]) = cap.extract();
    parse_int(left).unwrap() * parse_int(right).unwrap()
  }).sum()
}

pub fn part2(input: &str) -> i32 {
  let cmd_pattern = regex_static::static_regex!(
    r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)");
  let mut result = 0;
  let mut enabled = true;
  for cap in cmd_pattern.captures_iter(input) {
    match &cap.get(0).unwrap().as_str()[0..3] {
      "mul" => {
        if enabled {
          result += parse_int(cap.get(1).unwrap().as_str()).unwrap() *
              parse_int(cap.get(2).unwrap().as_str()).unwrap();
        }
      }
      "do(" => { enabled = true; }
      "don" => { enabled = false }
      _ => {}
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
