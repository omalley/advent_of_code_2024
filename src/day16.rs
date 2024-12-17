use std::cmp::Reverse;
use std::collections::BinaryHeap;
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

#[derive(Clone,Copy,Debug,Default,Eq,Ord,PartialEq,PartialOrd)]
pub enum Direction {
  #[default] North, West, South, East,
}

impl Direction {
  fn opposite(self) -> Direction {
    match self {
      Direction::North => Direction::South,
      Direction::West => Direction::East,
      Direction::South => Direction::North,
      Direction::East => Direction::West,
    }
  }
}

#[derive(Clone,Copy,Debug,Eq,Ord,PartialEq,PartialOrd)]
pub struct Coordinate {
  y: Position,
  x: Position,
}

impl Coordinate {
  fn new(y: usize, x: usize) -> Coordinate {
    Coordinate{y: y as Position, x: x as Position}
  }

  fn step(&self, direction: Direction) -> Coordinate {
    match direction {
      Direction::North => Coordinate {y: self.y - 1, x: self.x},
      Direction::West => Coordinate {y: self.y, x: self.x - 1},
      Direction::South => Coordinate {y: self.y + 1, x: self.x},
      Direction::East => Coordinate {y: self.y, x: self.x + 1},
    }
  }
}

#[derive(Clone,Copy,Debug,Eq,Ord,PartialEq,PartialOrd)]
struct PositionedDirection {
  direction: Direction,
  place: Coordinate,
}

type NeighborList = SmallVec<[PositionedDirection; 4]>;

type Cost = u64;

#[derive(Clone,Debug,Eq,Ord,PartialEq,PartialOrd)]
pub struct CostComponents {
  turns: u64,
  steps: u64,
}

impl CostComponents {

  const WALK_COST: Cost = 1;
  const TURN_COST: Cost = 1000;

  fn cost(&self) -> Cost {
    Self::WALK_COST * self.steps + Self::TURN_COST * self.turns
  }
}


#[derive(Clone,Debug)]
pub struct Grid {
  floor: Array2D<FloorKind>,
  start: Coordinate,
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
                    start = Some(Coordinate{x: x as Position, y: y as Position});
                    Ok(FloorKind::Start)},
                  'E' => {
                    end = Some(Coordinate{x: x as Position, y: y as Position});
                    Ok(FloorKind::End)},
                  _ => Err(format!("Invalid character '{}'", ch))})
                .try_collect())
        .try_collect()?;
    let floor = Array2D::from_rows(&floor_vec)
        .map_err(|e| format!("Can't build floor - {e}"))?;
    Ok(Grid{floor, start: start.ok_or("Can't find start")?,
      end: end.ok_or("Can't find end")?})
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

  #[inline]
  fn get(&self, position: Coordinate) -> FloorKind {
    self.floor[(position.y as usize, position.x as usize)]
  }

  fn find_neighbors(&self, place: Coordinate) -> NeighborList {
    [Direction::North, Direction::South, Direction::East, Direction::West].iter()
        .map(|direction| PositionedDirection{direction: *direction,
          place: place.step(*direction)})
        .filter(|n| self.get(n.place).is_open())
        .collect()
  }

  /// Create an array of the intersection id for each location.
  /// id 0 is the start and id 1 is the exit.
  fn find_intersections(&self) -> (Array2D<Option<usize>>, usize) {
    let mut result = Array2D::filled_with(None, self.floor.row_len(),
                                          self.floor.column_len());
    let mut next_id: usize = 2;
    for (y, row) in self.floor.rows_iter().enumerate() {
      for (x, spot) in row.enumerate() {
        let coord = Coordinate::new(y, x);
        match spot {
          FloorKind::Empty if self.find_neighbors(coord).len() > 2 => {
            result[(y, x)] = Some(next_id);
            next_id += 1;
          }
          FloorKind::Start => {
            result[(y, x)] = Some(Graph::START);
          }
          FloorKind::End => {
            result[(y, x)] = Some(Graph::END);
          }
          _ => {},
        }
      }
    }
    (result, next_id)
  }

  /// Calculate the result and cost of taking the given path.
  fn walk(&self, start: PositionedDirection) -> Option<(PositionedDirection, CostComponents)> {
    let mut current = start;
    let mut cost = CostComponents{turns: 0, steps: 1};
    loop {
      // exit if we reach the start or end
      if current.place == self.start || current.place == self.end { break }
      let mut neighbors = self.find_neighbors(current.place);
      // don't turn around
      neighbors.retain(|n| n.direction != current.direction.opposite());
      match neighbors.len() {
        0 => { return None },
        1 => {
          let next = neighbors.pop().unwrap();
          if next.direction != current.direction {
            cost.turns += 1;
          }
          cost.steps += 1;
          current = next;
        }
        _ => { break },
      }
    }
    Some((current, cost))
  }
}

pub fn generator(input: &str) -> Graph {
  Graph::from_grid(&Grid::from_str(input).expect("Can't parse input"))
}

#[derive(Debug)]
pub struct Edge {
  start_direction: Direction,
  destination: usize,
  destination_direction: Direction,
  cost: CostComponents,
}

type EdgeList = SmallVec<[Edge; 4]>;

#[derive(Debug)]
pub struct Graph {
  nodes: Vec<EdgeList>,
}

impl Graph {
  const START: usize = 0;
  const END: usize = 1;

  fn from_grid(grid: &Grid) -> Graph {
    let (intersections, node_count) = grid.find_intersections();
    let mut nodes: Vec<EdgeList> = (0..node_count).map(|_| SmallVec::new()).collect();
    let mut pending = vec![grid.start];
    let mut visited = vec![false; node_count];
    while let Some(current) = pending.pop() {
      let node_id = intersections[(current.y as usize, current.x as usize)].unwrap();
      if !visited[node_id] {
        visited[node_id] = true;
        for neighbor in grid.find_neighbors(current) {
          if let Some((dest, cost)) = grid.walk(neighbor) {
            let dest_node = intersections[(dest.place.y as usize,
                                           dest.place.x as usize)].unwrap();
            if !visited[dest_node] {
              pending.push(dest.place);
              nodes[node_id].push(Edge{start_direction: neighbor.direction,
                destination: dest_node, destination_direction: dest.direction, cost: cost.clone()});
              nodes[dest_node].push(Edge{start_direction: dest.direction.opposite(),
                destination: node_id, destination_direction: neighbor.direction.opposite(), cost});
            }
          }
        }
      }
    }
    Graph{nodes}
  }

  #[allow(dead_code)]
  fn display(&self) {
    for (id, node) in self.nodes.iter().enumerate() {
      println!("Node {id}:");
      for neighbor in node {
        println!("   {neighbor:?}");
      }
    }
  }

  fn minimum_cost(&self) -> Array2D<Cost> {
    let mut cost = Array2D::filled_with(Cost::MAX, self.nodes.len(), 4);
    let mut heap = BinaryHeap::new();
    cost[(Self::START, Direction::East as usize)] = 0;
    heap.push(Reverse(WorkState{cost: 0, node: Self::START, direction: Direction::East}));
    while let Some(Reverse(current)) = heap.pop() {
      if current.cost > cost[(current.node, current.direction as usize)] {
        continue;
      }

      for edge in &self.nodes[current.node] {
        let mut next_cost = current.cost + edge.cost.cost();
        if edge.start_direction != current.direction {
          next_cost += CostComponents::TURN_COST;
        }
        let next = WorkState { cost: next_cost, node: edge.destination,
          direction: edge.destination_direction };

        if next_cost < cost[(edge.destination, edge.destination_direction as usize)] {
          heap.push(Reverse(next));
          cost[(edge.destination, edge.destination_direction as usize)] = next_cost;
        }
      }
    }
    cost
  }
}

#[derive(Debug,Eq,Ord,PartialEq,PartialOrd)]
struct WorkState {
  cost: Cost,
  node: usize,
  direction: Direction,
}

#[allow(dead_code)]
fn display_intersections(grid: &Grid) {
  let (intersections, _) = grid.find_intersections();
  for (y, row) in intersections.rows_iter().enumerate() {
    for (x, int) in row.enumerate() {
      if let Some(i) = int {
        print!("{i:2}");
      } else {
        match grid.get(Coordinate::new(y, x)) {
          FloorKind::Wall => print!("##"),
          _ => print!(".."),
        }
      }
    }
    println!();
  }
}

fn min_cost(cost: &Array2D<Cost>, node: usize) -> Cost {
  *cost.row_iter(node).unwrap().min().unwrap()
}

pub fn part1(graph: &Graph) -> u64 {
  min_cost(&graph.minimum_cost(), Graph::END)
}

pub fn part2(graph: &Graph) -> u64 {
  let cost = graph.minimum_cost();
  let final_cost = min_cost(&cost, Graph::END);
  let mut pending = Vec::with_capacity(10);
  let mut node_visited = vec![false; graph.nodes.len()];
  let mut edge_visited = Array2D::filled_with(false, graph.nodes.len(), 4);
  // set up initial state
  let mut spaces = 1;
  node_visited[Graph::END] = true;
  for edge in &graph.nodes[Graph::END] {
    if cost[(Graph::END, edge.start_direction.opposite() as usize)] == final_cost {
      pending.push(WorkState{cost: final_cost - edge.cost.cost(),
        node: edge.destination, direction: edge.destination_direction});
      edge_visited[(edge.destination, edge.destination_direction as usize)] = true;
      spaces += edge.cost.steps - 1;
    }
  };
  // main loop
  while let Some(current) = pending.pop() {
    if !node_visited[current.node] {
      spaces += 1;
      node_visited[current.node] = true;
    }

    for edge in &graph.nodes[current.node] {
      if !edge_visited[(edge.destination, edge.destination_direction as usize)] {
        let mut goal_cost = current.cost;
        if edge.start_direction != current.direction {
          goal_cost -= CostComponents::TURN_COST;
        }
        if goal_cost == cost[(current.node, edge.start_direction.opposite() as usize)] &&
            goal_cost >= edge.cost.cost() {
          edge_visited[(edge.destination, edge.destination_direction as usize)] = true;
          spaces += edge.cost.steps - 1;
          pending.push(WorkState{cost: goal_cost - edge.cost.cost(), node: edge.destination,
            direction: edge.destination_direction});
        }
      }
    }
  }
  spaces
}

#[cfg(test)]
mod tests {
  use super::{generator, part1, part2};

  const INPUT: &str =
"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

  #[test]
  fn test_part1() {
    let data = generator(INPUT);
    assert_eq!(7036, part1(&data));
  }

  const BIGGER: &str =
"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

  #[test]
  fn test_bigger_part1() {
    let data = generator(BIGGER);
    assert_eq!(11048, part1(&data));
  }

  #[test]
  fn test_part2() {
    let data = generator(INPUT);
    assert_eq!(45, part2(&data));
  }

  #[test]
  fn test_bigger_part2() {
    let data = generator(BIGGER);
    assert_eq!(64, part2(&data));
  }
}
