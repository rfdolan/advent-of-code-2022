use std::vec::Vec;
use std::ops;
use std::collections::{HashSet, VecDeque};

#[derive (Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
  x: i32,
  y: i32,
  z: i32
}

impl ops::Add<Point> for Point {
  type Output = Point;
  fn add(self, other: Point) -> Point {
    Point{x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
  }
}

fn main(){
  let vec = inp::parse_file("day18.txt");
  solve(&vec);
}

fn bfs(graph: &HashSet<Point>) -> i32 {
  let mut surface_area = 0;
  let mut visited: HashSet<Point> = HashSet::new();
  let mut to_visit: VecDeque<Point> = VecDeque::new();
  // Try starting at each point in case some are unreachable from others.
  for start in graph.iter() {
    to_visit.push_front(*start);

    loop {
      //println!("loop");
      match to_visit.pop_back() {
        Some(curr_block) => {
          // Blank side, add to surface area.
          if !graph.contains(&curr_block) {
            surface_area += 1;
          } 
          // There's a block here that we haven't visited, add its neighbors
          else if !visited.contains(&curr_block){
            visited.insert(curr_block);
            to_visit.push_back(curr_block + Point{x:1, y:0, z:0});
            to_visit.push_back(curr_block + Point{x:-1, y:0, z:0});
            to_visit.push_back(curr_block + Point{x:0, y:1, z:0});
            to_visit.push_back(curr_block + Point{x:0, y:-1, z:0});
            to_visit.push_back(curr_block + Point{x:0, y:0, z:1});
            to_visit.push_back(curr_block + Point{x:0, y:0, z:-1});
          }
        },
        None => break
      }
    }
  }
  return surface_area;
}

// Solution
fn solve(input: &Vec<String>) {
  let mut blocks:HashSet<Point> = HashSet::new();
  for b in input {
    let b = b.split(",").map(|x| x.parse::<i32>().expect(&format!("{} not an int", x))).collect::<Vec<i32>>();
    let block = Point{x:b[0], y:b[1], z:b[2]};
    blocks.insert(block);
  }
  println!("Part 1: {}", bfs(&blocks));
  println!("Part 2: {}", 0);
}
