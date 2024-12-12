use std::ops::Range;
use smallvec::SmallVec;
use union_find::{QuickUnionUf, UnionBySize, UnionFind};

fn parse_line(s: &str) -> Vec<u8> {
  s.chars().map(|c| c as u8).collect()
}

type Position = i32;

#[derive(Clone,Debug)]
struct Coordinate {
  x: Position,
  y: Position,
}

#[derive(Debug)]
pub struct Grid {
  grid: Vec<Vec<u8>>,
  x_bound: Range<Position>,
  y_bound: Range<Position>,
}

impl Grid {
  fn neighbors<const SE:bool>(&self, pos: &Coordinate) -> SmallVec<[Coordinate; 4]> {
    [(-1, 0, true), (1, 0, false), (0, -1, true), (0, 1, false)].iter()
        .filter(|(_,_,is_nw)| SE || *is_nw)
        .map(|(dx,dy, _)| Coordinate{x: pos.x + dx, y: pos.y + dy})
        .filter(|coord| self.x_bound.contains(&coord.x)
            && self.y_bound.contains(&coord.y))
        .collect()
  }

  #[inline]
  fn get(&self, pos: &Coordinate) -> u8 {
    self.grid[pos.y as usize][pos.x as usize]
  }

  fn count_same_neighbors(&self, pos: &Coordinate) -> usize {
    let crop = self.get(pos);
    self.neighbors::<true>(pos).iter().filter(|&n| self.get(n) == crop).count()
  }
}

pub fn generator(input: &str) -> Grid {
  let grid: Vec<Vec<u8>> = input.lines().map(parse_line).collect();
  let y_bound = 0..(grid.len() as Position);
  let x_bound = 0..(grid[0].len() as Position);
  Grid{grid, x_bound, y_bound}
}

/// For each location, find the size of the field it is part of
fn find_sizes(input: &Grid) -> Vec<Vec<usize>> {
  let width = input.x_bound.len() as Position;
  // Each location starts as its own set
  let mut unionfind: QuickUnionUf<UnionBySize> =
      QuickUnionUf::new(input.x_bound.len() * input.y_bound.len());
  // Merge the matching sets together that are adjacent to each other.
  for y in input.y_bound.clone() {
    for x in input.x_bound.clone() {
      let cur = Coordinate{x,y};
      let crop = input.get(&cur);
      for neighbor in input.neighbors::<false>(&cur).iter()
          .filter(|&c| input.get(c) == crop) {
        unionfind.union((y * width + x) as usize,
                        (neighbor.y * width + neighbor.x) as usize);
      }
    }
  }
  // For each location, find the size of the associated set.
  input.grid.iter().enumerate()
      .map(|(y, row) | row.iter().enumerate()
          .map(|(x, _) | unionfind.get(y * width as usize + x).size())
          .collect())
      .collect()
}

pub fn part1(input: &Grid) -> usize {
  let sizes = find_sizes(input);
  input.grid.iter().enumerate()
      .map(|(y, row)| row.iter().enumerate()
          .map(|(x, _)| sizes[y][x] *
              (4 - input.count_same_neighbors(&Coordinate{x: x as i32, y: y as i32})))
          .sum::<usize>())
      .sum()
}

pub fn part2(_input: &Grid) -> usize {
  0
}

#[cfg(test)]
mod tests {
  use super::{generator, part1, part2};

  const INPUT: &str =
"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

  #[test]
  fn test_part1() {
    let data = generator(INPUT);
    assert_eq!(1930, part1(&data));
  }

  #[test]
  fn test_part2() {
    let _data = generator(INPUT);
    //assert_eq!(31, part2(&data));
  }
}
