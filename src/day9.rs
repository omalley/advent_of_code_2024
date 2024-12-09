use std::ops::Range;

type Position = u32;
type FileId = u32;

#[derive(Clone,Debug)]
pub struct FileRange {
  range: Range<Position>,
  id: FileId,
}

impl FileRange {
  fn checksum(&self) -> u64 {
    self.id as u64 * self.range.clone().sum::<u32>() as u64
  }
}

fn parse_int(ch: char) -> Result<Position, String> {
  ch.to_digit(10).ok_or(format!("{} is not a valid integer", ch))
}

pub fn generator(input: &str) -> Vec<FileRange> {
  let mut next_address = 0;
  let mut result = Vec::new();
  let mut is_file = true;
  for ch in input.trim().chars() {
    let size = parse_int(ch).expect("Can't parse integer");
    if is_file {
      let id = result.len() as Position;
      result.push(FileRange{range: next_address..(next_address + size), id });
    }
    next_address += size;
    is_file = !is_file;
  }
  result
}

fn compacted_size(files: &[FileRange]) -> Position {
  files.iter().map(|f| f.range.len() as Position).sum()
}

/// Divide the files into ranges that are fine and ones the need to be compacted.
fn split_files(files: &[FileRange]) -> (Vec<FileRange>,Vec<FileRange>) {
  let new_size = compacted_size(files);
  let mut good = Vec::new();
  let mut bad = Vec::new();
  for f in files {
    if f.range.end < new_size {
      good.push(f.clone())
    } else if f.range.start < new_size {
      good.push(FileRange{id: f.id, range: f.range.start..new_size});
      bad.push(FileRange{id: f.id, range: new_size..f.range.end});
    } else {
      bad.push(f.clone());
    }
  }
  (good, bad)
}

fn compact(files: &[FileRange]) -> Vec<FileRange> {
  let mut result = Vec::new();
  let (left, mut right) = split_files(files);
  let mut next_address = 0;
  for f in left {
    while next_address < f.range.start && !right.is_empty() {
      let mut moving = right.pop().unwrap();
      let moving_space = moving.range.len() as Position;
      let room = f.range.start - next_address;
      if moving_space <= room {
        result.push(FileRange{range: next_address..(next_address + moving_space),
          id : moving.id});
        next_address += moving_space;
      } else {
        result.push(FileRange{range: next_address..(next_address + room),
          id : moving.id});
        moving.range.end = moving.range.start + moving_space - room;
        right.push(moving);
        next_address += room;
      }
    }
    next_address = f.range.end;
    result.push(f);
  }
  for f in right.iter().rev() {
    let len = f.range.len() as Position;
    result.push(FileRange{id: f.id, range: next_address..next_address + len});
    next_address += len;
  }
  result
}

fn checksum(files: &[FileRange]) -> u64 {
  files.iter().map(|f| f.checksum()).sum()
}

pub fn part1(input: &[FileRange]) -> u64 {
  checksum(&compact(input))
}

pub fn part2(input: &[FileRange]) -> u64 {
  0
}

#[cfg(test)]
mod tests {
  use super::{generator, part1, part2};

  const INPUT: &str = "2333133121414131402";

  #[test]
  fn test_part1() {
    let data = generator(INPUT);
    assert_eq!(1928, part1(&data));
  }

  #[test]
  fn test_part2() {
    let data = generator(INPUT);
    assert_eq!(2858, part2(&data));
  }
}
