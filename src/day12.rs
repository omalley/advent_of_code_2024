use std::cmp::Ordering;
use std::ops::Range;
use smallvec::SmallVec;
use union_find::{QuickUnionUf, UnionBySize, UnionFind};

fn parse_line(s: &str) -> Vec<u8> {
  s.chars().map(|c| c as u8).collect()
}

type Position = i32;

#[derive(Clone,Debug,Eq,Ord,PartialEq,PartialOrd)]
struct Coordinate {
  y: Position,
  x: Position,
}

#[derive(Debug)]
pub struct Grid {
  plots: Vec<Vec<u8>>,
  x_bound: Range<Position>,
  y_bound: Range<Position>,
}

impl Grid {
  /// Find the neighbors who are growing the same crop.
  /// SE controls whether the south and east neighbors are included.
  /// The output is sorted.
  fn neighbors<const SE:bool>(&self, pos: &Coordinate, crop: u8) -> SmallVec<[Coordinate; 4]> {
    [(-1, 0, true), (0, -1, true), (0, 1, false), (1, 0, false)].iter()
        .filter(|(_,_,is_nw)| SE || *is_nw)
        .map(|(dy,dx, _)| Coordinate{x: pos.x + dx, y: pos.y + dy})
        .filter(|coord| self.x_bound.contains(&coord.x)
            && self.y_bound.contains(&coord.y) && self.get(coord) == crop)
        .collect()
  }

  /// Count the neighbors with the relative offsets that are growing the crop.
  fn count_neighbors(&self, pos: &Coordinate, crop: u8, dirs: &[(Position, Position)]) -> usize {
    dirs.iter()
        .map(|(dy,dx)| Coordinate{x: pos.x + dx, y: pos.y + dy})
        .filter(|coord| self.x_bound.contains(&coord.x)
            && self.y_bound.contains(&coord.y) && self.get(coord) == crop)
        .count()
  }

  #[inline]
  fn get(&self, pos: &Coordinate) -> u8 {
    self.plots[pos.y as usize][pos.x as usize]
  }

  /// Count the number of half corners of the shape that this position includes.
  fn count_corners(&self, pos: &Coordinate) -> usize {
    let crop = self.get(pos);
    let neighbors = self.neighbors::<true>(pos, crop);
    match neighbors.len() {
      0 => 8,
      1 => {
        4 + self.count_neighbors(pos, crop,
                                 match neighbors[0].y.cmp(&pos.y) {
                                   Ordering::Less => &[(-1, -1), (-1, 1)],
                                   Ordering::Equal => {
                                     if neighbors[0].x < pos.x {
                                       &[(-1, -1), (1, -1)]
                                     } else {
                                       &[(-1, 1), (1, 1)]
                                     }
                                   }
                                   Ordering::Greater => &[(1, -1), (1, 1)],
                                 })
      },
      2 => {
        // Are they in a straight line?
        if neighbors[0].x == neighbors[1].x ||
            neighbors[0].y == neighbors[1].y {
          self.count_neighbors(pos, crop, &[(-1, -1), (-1, 1), (1, -1), (1, 1)])
        } else {
          2 + self.count_neighbors(pos, crop,
                                   if neighbors[1].x < pos.x || neighbors[0].x > pos.x
                                   { &[(-1, 1), (1, -1)] } else { &[(-1, -1), (1, 1)]})
        }
      },
      3 => {
        self.count_neighbors(pos, crop,
                             if neighbors[0].x == neighbors[2].x {
                               if neighbors[1].x < pos.x {
                                 &[(-1, 1), (1, 1)]
                               } else {
                                 &[(-1, -1), (1, -1)]
                               }
                             } else if neighbors[0].y < pos.y {
                               &[(1, -1), (1, 1)]
                             } else {
                               &[(-1, -1), (-1, 1)]
                             })
      }
      _ => 0,
    }
  }
}

#[derive(Debug)]
pub struct Input {
  grid: Grid,
  sizes: Vec<Vec<usize>>,
}

pub fn generator(input: &str) -> Input {
  let plots: Vec<Vec<u8>> = input.lines().map(parse_line).collect();
  let y_bound = 0..(plots.len() as Position);
  let x_bound = 0..(plots[0].len() as Position);
  let grid = Grid{ plots, x_bound, y_bound};
  let sizes = find_sizes(&grid);
  Input { grid, sizes }
}

/// For each location, find the size of the field it is part of
fn find_sizes(grid: &Grid) -> Vec<Vec<usize>> {
  let width = grid.x_bound.len() as Position;
  // Each location starts as its own set
  let mut unionfind: QuickUnionUf<UnionBySize> =
      QuickUnionUf::new(grid.x_bound.len() * grid.y_bound.len());
  // Merge the matching sets together that are adjacent to each other.
  for y in grid.y_bound.clone() {
    for x in grid.x_bound.clone() {
      let cur = Coordinate{x,y};
      for neighbor in grid.neighbors::<false>(&cur, grid.get(&cur)).iter() {
        unionfind.union((y * width + x) as usize,
                        (neighbor.y * width + neighbor.x) as usize);
      }
    }
  }
  // For each location, find the size of the associated set.
  grid.plots.iter().enumerate()
      .map(|(y, row) | row.iter().enumerate()
          .map(|(x, _) | unionfind.get(y * width as usize + x).size())
          .collect())
      .collect()
}

pub fn part1(input: &Input) -> usize {
  input.sizes.iter().enumerate()
      .map(|(y, row)| row.iter().enumerate()
          .map(|(x, size)| {
            let coord = Coordinate{x: x as i32, y: y as i32};
            size * (4 - input.grid.neighbors::<true>(&coord, input.grid.get(&coord)).len())})
          .sum::<usize>())
      .sum()
}

pub fn part2(input: &Input) -> usize {
  input.sizes.iter().enumerate()
      .map(|(y, row)| row.iter().enumerate()
          .map(|(x, size)| {
            let coord = Coordinate{x: x as i32, y: y as i32};
            size * input.grid.count_corners(&coord)})
          .sum::<usize>())
      .sum::<usize>() / 2
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
    assert_eq!(1206, part2(&generator(INPUT)));
    assert_eq!(80, part2(&generator(INPUT2)));
    assert_eq!(436, part2(&generator(INPUT3)));
    assert_eq!(236, part2(&generator(INPUT4)));
    assert_eq!(368, part2(&generator(INPUT5)));
  }

  const INPUT2: &str =
"AAAA
BBCD
BBCC
EEEC";

  const INPUT3: &str =
"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

  const INPUT4: &str =
"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

  const INPUT5: &str =
"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
}
