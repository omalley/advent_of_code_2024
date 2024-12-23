use array2d::Array2D;
use itertools::Itertools;
use smallvec::SmallVec;

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
pub enum FloorKind {
  Empty, Wall, Start, End,
}

impl FloorKind {
  #[inline]
  fn is_open(self) -> bool {
    self != FloorKind::Wall
  }
}

type Position = i16;

#[derive(Clone,Debug,Eq,PartialEq)]
pub struct Coordinate {
  y: Position,
  x: Position,
}

impl Coordinate {
  fn new(y: usize, x: usize) -> Coordinate {
    Coordinate { y: y as Position, x: x as Position }
  }
}

type NeighborList = SmallVec<[Coordinate; 4]>;

#[derive(Clone,Debug)]
pub struct Grid {
  floor: Array2D<FloorKind>,
  start: Coordinate,
  #[allow(dead_code)]
  end: Coordinate,
}

impl Grid {
  fn from_str(input: &str) -> Result<Self, String> {
    let mut start = None;
    let mut end = None;
    let floor_vec: Vec<Vec<FloorKind>> = input.lines().enumerate()
        .map(|(y, line)|
            line.chars().enumerate()
                .map(|(x, ch)| match ch {
                  '#' => Ok(FloorKind::Wall),
                  '.' => Ok(FloorKind::Empty),
                  'S' => {
                    start = Some(Coordinate::new(y,x));
                    Ok(FloorKind::Start)
                  },
                  'E' => {
                    end = Some(Coordinate::new(y,x));
                    Ok(FloorKind::End)
                  },
                  _ => Err(format!("Invalid character '{}'", ch))
                })
                .try_collect())
        .try_collect()?;
    let floor = Array2D::from_rows(&floor_vec)
        .map_err(|e| format!("Can't build floor - {e}"))?;
    Ok(Grid {
      floor,
      start: start.ok_or("Can't find start")?,
      end: end.ok_or("Can't find end")?
    })
  }

  #[allow(dead_code)]
  fn display(&self) {
    for row_itr in self.floor.rows_iter() {
      for val in row_itr {
        let ch = match val {
          FloorKind::Wall => '#',
          FloorKind::Empty => '.',
          FloorKind::Start => 'S',
          FloorKind::End => 'E',
        };
        print!("{ch}");
      }
      println!();
    }
  }

  fn neighbors(&self, source: &Coordinate) -> NeighborList {
    [(-1, 0), (1, 0), (0, -1), (0, 1)].iter()
        .map(|(dy, dx)| Coordinate{y: source.y + dy, x: source.x + dx})
        .filter(|dest| (0..self.floor.row_len() as Position).contains(&dest.y) &&
            (0..self.floor.column_len() as Position).contains(&dest.x) &&
            self.floor[(dest.y as usize, dest.x as usize)].is_open())
        .collect()
  }

  fn find_distances(&self) -> Array2D<usize> {
    let mut result = Array2D::filled_with(usize::MAX, self.floor.row_len(),
                                          self.floor.column_len());
    let mut pending = vec![(0, self.start.clone())];
    while let Some((cost, spot)) = pending.pop() {
      if result[(spot.y as usize, spot.x as usize)] > cost {
        result[(spot.y as usize, spot.x as usize)] = cost;
        for n in self.neighbors(&spot) {
          pending.push((cost + 1, n.clone()));
        }
      }
    }
    result
  }
}

pub fn generator(input: &str) -> Grid {
  Grid::from_str(input).expect("Can't parse input")
}

fn cheat_distance(distances: &Array2D<usize>, p1: Coordinate, p2: Coordinate) -> usize {
  match (distances[(p1.y as usize, p1.x as usize)], distances[(p2.y as usize, p2.x as usize)]) {
    (usize::MAX, _) | (_, usize::MAX) => 0,
    (left, right) => left.abs_diff(right).max(2) - 2,
  }
}

pub fn do_part1(input: &Grid, limit: usize) -> usize {
  let distances = input.find_distances();
  let mut count = 0;
  for (y, row) in input.floor.rows_iter().enumerate() {
    for (x, flr) in row.enumerate() {
      if *flr == FloorKind::Wall && x != 0 && y != 0 && y != input.floor.num_rows() - 1 &&
          x != input.floor.num_columns() - 1 {
        if cheat_distance(&distances, Coordinate::new(y - 1, x),
                          Coordinate::new(y + 1, x)) >= limit {
          count += 1;
        }
        if cheat_distance(&distances, Coordinate::new(y, x - 1),
                          Coordinate::new(y, x + 1)) >= limit {
          count += 1;
        }
      }
    }
  }
  count
}

pub fn part1(input: &Grid) -> usize {
  do_part1(input, 100)
}

pub fn part2(_input: &Grid) -> usize {
  0
}

#[cfg(test)]
mod tests {
  use super::{generator, do_part1, part2};

  const INPUT: &str =
"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

  #[test]
  fn test_part1() {
    let data = generator(INPUT);
    assert_eq!(3, do_part1(&data, 38));
  }

  #[test]
  fn test_part2() {
    let data = generator(INPUT);
    assert_eq!(0, part2(&data));
  }
}
