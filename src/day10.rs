use smallvec::SmallVec;

type Elevation = u8;
type Position = i32;

#[derive(Clone,Copy,Debug,Eq,Ord,PartialEq,PartialOrd)]
pub struct Coordinate {
  y: Position,
  x: Position,
}

type NeighborList = SmallVec<[Coordinate; 4]>;

pub struct Map {
  grid: Vec<Vec<Elevation>>,
  starts: Vec<Coordinate>,
  ends: Vec<Coordinate>,
}

impl Map {
  fn get(&self, coordinate: Coordinate) -> Option<Elevation> {
    if (0..self.grid.len()).contains(&(coordinate.y as usize)) {
      let row = &self.grid[coordinate.y as usize];
      if (0..row.len()).contains(&(coordinate.x as usize)) {
        return Some(row[coordinate.x as usize]);
      }
    }
    None
  }

  fn neighbors(&self, coordinate: Coordinate) -> NeighborList {
    vec![(-1, 0), (1, 0), (0, -1), (0, 1)].iter()
        .map(|(xd,yd)| Coordinate{x: coordinate.x + xd, y: coordinate.y + yd})
        .collect()
  }

  fn potential_previous(&self, coordinate: Coordinate, elevation: Elevation) -> NeighborList {
    self.neighbors(coordinate).iter().filter(|&c| self.get(*c) == Some(elevation - 1))
        .copied().collect()
  }
}

const START: Elevation = 0;
const END: Elevation = 9;

pub fn generator(input: &str) -> Map {
  let mut starts = Vec::new();
  let mut ends = Vec::new();
  let grid = input.lines().enumerate()
      .map(|(y,line)| line.chars().enumerate().
          map(|(x, c)| {
            let ele = c.to_digit(10).unwrap() as Elevation;
            match ele {
              START => starts.push(Coordinate{x: x as Position, y: y as Position}),
              END => ends.push(Coordinate{x: x as Position, y: y as Position}),
              _ => {},
            }
            ele
          }).collect())
      .collect();
  Map{grid, starts, ends}
}

pub fn part1(input: &Map) -> u64 {
  let mut counts = vec![0; input.starts.len()];
  for dest in &input.ends {
    let mut current = vec![dest.clone()];
    for elevation in (START..END).rev() {
      let mut next: Vec<Coordinate> = current.iter()
          .flat_map(|c| input.potential_previous(*c, elevation+1)).collect();
      next.sort_unstable();
      next.dedup();
      current = next;
    }
    for start in &current {
      counts[input.starts.binary_search(start).unwrap()] += 1;
    }
  }
  counts.into_iter().sum()
}

pub fn part2(input: &Map) -> u64 {
  let mut result = 0;
  for dest in &input.ends {
    let mut current = vec![dest.clone()];
    for elevation in (START..END).rev() {
      let next: Vec<Coordinate> = current.iter()
          .flat_map(|c| input.potential_previous(*c, elevation+1)).collect();
      current = next;
    }
    result += current.len() as u64;
  }
  result
}

#[cfg(test)]
mod tests {
  use super::{generator, part1, part2};

  const INPUT: &str =
"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

  #[test]
  fn test_part1() {
    let data = generator(INPUT);
    assert_eq!(36, part1(&data));
  }

  #[test]
  fn test_part2() {
    let data = generator(INPUT);
    assert_eq!(81, part2(&data));
  }
}
