pub struct Board {
  vals: Vec<Vec<u8>>,
  width: usize,
  height: usize,
}

impl Board {
  fn get(&self, x: i32, y: i32) -> u8 {
    self.vals[y as usize][x as usize]
  }
}

pub fn generator(input: &str) -> Board {
  let vals: Vec<Vec<u8>> = input.lines()
      .map(|l: &str| l.chars().map(|ch| ch as u8).collect())
      .collect();
  let height = vals.len();
  let width = vals[0].len();
  Board{vals, width, height}
}

fn count_words(board: &Board,
               pattern: &[u8],
               x: usize, y: usize, delta_x: i32, delta_y: i32) -> usize {
  let mut result = 0;
  let mut x = x as i32;
  let mut y = y as i32;
  let mut current = 0;
  while x < board.width as i32 && y < board.height as i32 && x >= 0 && y >= 0 {
    let next = board.get(x, y);
    if next == pattern[current] {
      current += 1;
      if current == pattern.len() {
        result += 1;
        current = 0;
      }
    } else if next == pattern[0] {
      current = 1;
    } else {
      current = 0;
    }
    x += delta_x;
    y += delta_y;
  }
  result
}

pub fn part1(input: &Board) -> usize {
  let pattern = "XMAS".as_bytes();
  let mut result = 0;
  for x in 0..input.width {
    result += count_words(input, pattern, x, 0, 0, 1);
    result += count_words(input, pattern, x, 0, 1, 1);
    result += count_words(input, pattern, x, 0, -1, 1);
    result += count_words(input, pattern, x, input.height - 1, 0, -1);
    result += count_words(input, pattern, x, input.height - 1, -1, -1);
    result += count_words(input, pattern, x, input.height - 1, 1, -1);
  }
  for y in 0..input.height {
    result += count_words(input, pattern, 0, y, 1, 0);
    result += count_words(input, pattern, input.width - 1, y, -1, 0);
  }
  for y in 1..input.height-1 {
    result += count_words(input, pattern, 0, y, 1, 1);
    result += count_words(input, pattern, 0, y, 1, -1);
    result += count_words(input, pattern, input.width - 1, y, -1, -1);
    result += count_words(input, pattern, input.width - 1, y, -1, 1);
  }
  result
}

fn has_x_mas(board: &Board, x: i32, y: i32, pattern: &[u8]) -> bool {
  let up_left = board.get(x - 1, y - 1);
  let down_left = board.get(x - 1, y + 1);
  let up_right = board.get(x + 1, y - 1);
  let down_right = board.get(x + 1, y + 1);
  if up_left == pattern[0] {
    if down_right != pattern[2] {
      return false;
    }
  } else if up_left == pattern[2] {
    if down_right != pattern[0] {
      return false
    }
  } else {
    return false
  }
  if down_left == pattern[0] {
    up_right == pattern[2]
  } else if down_left == pattern[2] {
    up_right == pattern[0]
  } else {
    false
  }
}

pub fn part2(input: &Board) -> usize {
  let pattern = "MAS".as_bytes();
  let mut result = 0;
  for x in 1..(input.width-1) as i32 {
    for y in 1..(input.height-1) as i32 {
      if input.get(x, y) == pattern[1] && has_x_mas(input, x, y, pattern) {
        result += 1;
      }
    }
  }
  result
}

#[cfg(test)]
mod tests {
  use super::{generator, part1, part2};

  const INPUT: &str =
"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

  #[test]
  fn test_part1() {
    let data = generator(INPUT);
    assert_eq!(18, part1(&data));
  }

  #[test]
  fn test_part2() {
    let data = generator(INPUT);
    assert_eq!(9, part2(&data));
  }
}
