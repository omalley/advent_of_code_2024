use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::ops::Range;
use array2d::Array2D;
use itertools::Itertools;
use smallvec::SmallVec;
use union_find::{QuickUnionUf, UnionByRank, UnionFind};

type Position = i16;

fn parse_int(s: &str) -> Result<Position, String> {
  s.parse().map_err(|_| format!("Can't parse integer - '{s}'"))
}

#[derive(Clone,Debug,Eq,Ord,PartialEq,PartialOrd)]
pub struct Coordinate {
  x: Position,
  y: Position,
}

fn parse_line(s: &str) -> Result<Coordinate, String> {
  let (left, right) = s.split_once(',').ok_or(format!("Can't split '{s}'"))?;
  Ok(Coordinate{x: parse_int(left)?, y: parse_int(right)?})
}

pub fn generator(input: &str) -> Vec<Coordinate> {
  input.lines().map(parse_line).try_collect().expect("Can't parse input")
}

fn make_grid(blocks: &[Coordinate], bounds: Range<Position>) -> Array2D<bool> {
  let mut grid = Array2D::filled_with(false, bounds.len(), bounds.len());
  for blk in blocks.iter() {
    grid[(blk.y as usize, blk.x as usize)] = true;
  }
  grid
}

type NeighborList = SmallVec<[Coordinate; 4]>;

fn neighbors(grid: &Array2D<bool>, coord: Coordinate) -> NeighborList {
  [(-1, 0), (1, 0), (0, -1), (0, 1)].iter()
      .map(|&(dx, dy)| Coordinate{x: coord.x + dx, y: coord.y + dy})
      // Is the neighbor in bounds
      .filter(|c| c.x >= 0 && c.x < grid.num_columns() as Position
          && c.y >= 0 && c.y < grid.num_rows() as Position)
      // Is the way not blocked
      .filter(|c| !grid[(c.y as usize, c.x as usize)])
      .collect()
}

#[allow(dead_code)]
fn display_grid(grid: &Array2D<bool>) {
  for row in grid.rows_iter() {
    for blk in row {
      print!("{}", if *blk { '#' } else { '.' });
    }
    println!();
  }
}

#[derive(Clone,Debug,Eq,Ord,PartialEq,PartialOrd)]
struct WorkItem {
  distance: usize,
  coord: Coordinate,
}

pub fn run_part1(input: &[Coordinate], bounds: Range<Position>) -> usize {
  let grid = make_grid(input, bounds.clone());
  let mut distance = Array2D::filled_with(usize::MAX, bounds.len(), bounds.len());
  distance[(0, 0)] = 0;
  let mut pending = BinaryHeap::new();
  pending.push(Reverse(WorkItem{distance: 0, coord: Coordinate{x: 0, y: 0}}));
  while let Some(Reverse(current)) = pending.pop() {
    for neighbor in neighbors(&grid, current.coord) {
      if current.distance + 1 < distance[(neighbor.y as usize, neighbor.x as usize)] {
        distance[(neighbor.y as usize, neighbor.x as usize)] = current.distance + 1;
        pending.push(Reverse(WorkItem{distance: current.distance + 1,
          coord: neighbor}));
      }
    }
  }
  distance[(bounds.len() - 1, bounds.len() - 1)]
}

#[allow(dead_code)]
fn print_distances(distances: &Array2D<usize>) {
  for row in distances.rows_iter() {
    for dist in row {
      if *dist == usize::MAX {
        print!("{:4}", "");
      } else {
        print!("{:4}", *dist);
      }
    }
    println!();
  }
}

const FULL_SIZE: Position = 71;

pub fn part1(input: &[Coordinate]) -> usize {
  run_part1(&input[..1024], 0..FULL_SIZE)
}

pub fn run_part2(input: &[Coordinate], bounds: Range<Position>) -> String {
  let mut block_time = Array2D::filled_with(None, bounds.len(), bounds.len());
  let mut unionfind: QuickUnionUf<UnionByRank> = QuickUnionUf::new(2 + input.len());
  for (round, current) in input.iter().enumerate() {
    block_time[(current.y as usize, current.x as usize)] = Some(round + 2);
    // Is it connected to the left/bottom?
    if current.x == 0 || current.y == bounds.end - 1 {
      unionfind.union(0, round + 2);
    // Is it connected to the right/top?
    } else if current.y == 0 || current.x == bounds.end - 1 {
      unionfind.union(1, round + 2);
    }
    // connect us to each of the neighbors
    for (dx, dy) in [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)] {
      let (x, y) = (current.x + dx, current.y + dy);
      if bounds.contains(&x) && bounds.contains(&y) {
        if let Some(other) = block_time[(y as usize, x as usize)] {
          unionfind.union(round + 2, other);
        }
      }
    }
    if unionfind.find(0) == unionfind.find(1) {
      return format!("{},{}", current.x, current.y);
    }
  }
  "None".to_string()
}

pub fn part2(input: &[Coordinate]) -> String {
  run_part2(input, 0..FULL_SIZE)
}

#[cfg(test)]
mod tests {
  use super::{generator, run_part1, run_part2};

  const INPUT: &str =
"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

  #[test]
  fn test_part1() {
    let data = generator(INPUT);
    assert_eq!(22, run_part1(&data[..12], 0..7));
  }

  #[test]
  fn test_part2() {
    let data = generator(INPUT);
    assert_eq!("6,1", run_part2(&data, 0..7));
  }
}
