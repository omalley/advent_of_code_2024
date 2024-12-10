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

pub fn generator(input: &str) -> Vec<FileRange> {
  let mut next_address = 0;
  let mut result = Vec::new();
  let mut is_file = true;
  for ch in input.trim().chars() {
    let size = ch.to_digit(10).unwrap();
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

const SIZE_COUNT: usize = 10;

#[derive(Debug)]
struct FileCompactor<'a> {
  buckets: [Vec<FileRange>; SIZE_COUNT],
  done: Vec<bool>,
  files: &'a [FileRange],
  next_address: Position,
}

impl<'a> FileCompactor<'a> {
  fn from_files(files: &'a [FileRange]) -> Self {
    let mut buckets: [Vec<FileRange>; SIZE_COUNT] = Default::default();
    for f in files {
      buckets[f.range.len()].push(f.clone());
    }
    let done = vec![false; files.len()];
    Self{ files, done, buckets, next_address: 0}
  }

  fn next_file(&mut self) -> Option<FileRange> {
    loop {
      if !self.files.is_empty() {
        if self.files[0].range.start == self.next_address {
          self.next_address = self.files[0].range.end;
          let result = self.files[0].clone();
          self.files = &self.files[1..];
          if !self.done[result.id as usize] {
            self.done[result.id as usize] = true;
            return Some(result)
          }
        } else {
          let space = self.files[0].range.start - self.next_address;
          let mut best = None;
          for s in 1..=space {
            if !self.buckets[s as usize].is_empty() {
              let last = self.buckets[s as usize].len() - 1;
              if let Some((prev, _)) = best {
                if self.buckets[s as usize][last].id > prev {
                  best = Some((self.buckets[s as usize][last].id, s));
                }
              } else {
                best = Some((self.buckets[s as usize][last].id, s));
              }
            }
          }
          if let Some((_, size)) = best {
            let mut result = self.buckets[size as usize].pop().unwrap();
            result.range = self.next_address..self.next_address + size;
            self.next_address += size;
            if !self.done[result.id as usize] {
              self.done[result.id as usize] = true;
              return Some(result)
            }
          } else {
            let result = self.files[0].clone();
            self.files = &self.files[1..];
            self.next_address = result.range.end;
            if !self.done[result.id as usize] {
              self.done[result.id as usize] = true;
              return Some(result)
            }
          }
        }
      } else {
        return None
      }
    }
  }
}

fn file_compact(files: &[FileRange]) -> Vec<FileRange> {
  let mut compactor = FileCompactor::from_files(files);
  let mut result = Vec::new();
  while let Some(next) = compactor.next_file() {
    result.push(next);
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
  checksum(&file_compact(input))
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
