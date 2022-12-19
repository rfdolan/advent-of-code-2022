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

fn can_escape(bottom_left_close: Point, top_right_far: Point, start: Point, graph: &HashSet<Point>) -> bool {
  let mut visited: HashSet<Point> = HashSet::new();
  let mut to_visit: VecDeque<Point> = VecDeque::new();
  let x_range = bottom_left_close.x..top_right_far.x;
  let y_range = bottom_left_close.y..top_right_far.y;
  let z_range = bottom_left_close.z..top_right_far.z;
  // Try starting at each point in case some are unreachable from others.
  to_visit.push_front(start);
  loop {
    //println!("loop");
    match to_visit.pop_back() {
      Some(curr_space) => {
        // We haven't been here and there's nothing in the way.
        if !visited.contains(&curr_space) && !graph.contains(&curr_space) {
          if !x_range.contains(&curr_space.x) || !y_range.contains(&curr_space.y) || !z_range.contains(&curr_space.z) {
            //println!("Escaped!");
            return true;
          }
          visited.insert(curr_space);
          to_visit.push_back(curr_space + Point{x:1, y:0, z:0});
          to_visit.push_back(curr_space + Point{x:-1, y:0, z:0});
          to_visit.push_back(curr_space + Point{x:0, y:1, z:0});
          to_visit.push_back(curr_space + Point{x:0, y:-1, z:0});
          to_visit.push_back(curr_space + Point{x:0, y:0, z:1});
          to_visit.push_back(curr_space + Point{x:0, y:0, z:-1});
        } 
      },
      None => return false
    }
  }
}

// Solution
fn solve(input: &Vec<String>) {
  let mut blocks:HashSet<Point> = HashSet::new();
  let mut x_min = 2;
  let mut x_max = 2;
  let mut y_min = 2;
  let mut y_max = 2;
  let mut z_min = 2;
  let mut z_max = 2;
  for b in input {
    let b = b.split(",").map(|x| x.parse::<i32>().expect(&format!("{} not an int", x))).collect::<Vec<i32>>();
    let (x, y, z) = (b[0], b[1], b[2]);
    x_min = std::cmp::min(x_min, x);
    x_max = std::cmp::max(x_max, x);
    y_min = std::cmp::min(y_min, y);
    y_max = std::cmp::max(y_max, y);
    z_min = std::cmp::min(z_min, z);
    z_max = std::cmp::max(z_max, z);
    let block = Point{x:x, y:y, z:z};
    blocks.insert(block);
  }

  let bottom_left_close = Point{x:x_min, y:y_min, z:z_min};
  let top_right_far = Point{x:x_max, y:y_max, z:z_max};
  let mut surface_area = 0;
  let mut surface_area_including_inside = 0;
  let mut visited: HashSet<Point> = HashSet::new();
  let mut to_visit: VecDeque<Point> = VecDeque::new();
  // Try starting at each point in case some are unreachable from others.
  for start in blocks.iter() {
    //println!("({}, {}, {})", start.x, start.y, start.z);
    to_visit.push_front(*start);

    loop {
      //println!("loop");
      match to_visit.pop_back() {
        Some(curr_block) => {
          // Blank side, add to surface area if air can escape.
          if !blocks.contains(&curr_block) {
            surface_area_including_inside += 1;
            if can_escape(bottom_left_close, top_right_far, curr_block, &blocks) {
              surface_area += 1;
            }
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
  println!("Part 1: {}", surface_area_including_inside);
  println!("Part 2: {}", surface_area);
}
