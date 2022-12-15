use std::{vec::Vec, collections::HashSet};

#[derive (Copy, Clone, Eq, Hash, PartialEq)]
struct Point {
  x: i32,
  y: i32
}

fn main(){
  let vec = inp::parse_file("day09.txt");
  solve(&vec);
}

fn move_segment(target: Point, start: Point) -> Point {
  let x_diff = target.x - start.x;
  let y_diff = target.y - start.y;
  let mut new_segment_x = start.x;
  let mut new_segment_y = start.y;
  if x_diff.abs() > 1 || y_diff.abs() > 1 {
    new_segment_x = start.x + x_diff.signum();
    new_segment_y = start.y + y_diff.signum();
  }
  Point{x: new_segment_x, y: new_segment_y}

}

// Solution
fn solve(input: &Vec<String>) {
  let mut rope_segments: Vec<Point> = vec![Point{x:0,y:0}; 10];
  let mut visited: HashSet<Point> = HashSet::new();
  let mut first_segment_visited: HashSet<Point> = HashSet::new();
  visited.insert(Point{x: 0, y: 0});
  first_segment_visited.insert(Point{x: 0, y: 0});
  for command in input {
    println!("{}", command);
    let split = command.split(" ").collect::<Vec<&str>>();
    let command = split[0];
    let dist = split[1].parse::<i32>().expect(&format!("Couldn't parse {} as int.", split[1]));
    let mut xdir = 0;
    let mut ydir = 0;
    match command {
      "U" => ydir = 1,
      "R" => xdir = 1,
      "D" => ydir = -1,
      "L" => xdir = -1,
      _ => println!("Invalid command {}", command)
    }
    for _ in 0..dist {
      for segment_num in 0..rope_segments.len() {
        let segment = rope_segments[segment_num];
        if segment_num == 0 {
          rope_segments[segment_num] = Point{x: segment.x + xdir, y: segment.y + ydir};
        } else {
          let target = rope_segments[segment_num-1];
          rope_segments[segment_num] = move_segment(target, segment);
          if segment_num == 1 {
            first_segment_visited.insert(rope_segments[segment_num]);
          }
          if segment_num == 9 {
            //println!("End visited: ({}, {})", rope_segments[segment_num].x, rope_segments[segment_num].y);
            visited.insert(rope_segments[segment_num]);
          }

        }
      }

    }
    println!("STATE");
    for s in rope_segments.iter() {
      println!("({},{})", s.x, s.y);
    }
  }
  println!("DONE");
  for s in visited.iter() {
    println!("({},{})", s.x, s.y);
  }
  println!("Part 1: {}", first_segment_visited.len());
  println!("Part 2: {}", visited.len());
}
