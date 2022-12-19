use std::hash::Hash;
use std::vec::Vec;
use std::collections::HashSet;
use std::ops;

const CHAMBER_WIDTH: i32 = 7;
const TOTAL_NUM_ROCKS: i64 = 1000000000000;

#[derive (Copy, Clone)]
enum SHAPES {
  HorizontalLine = 0,
  Cross = 1,
  BackwardsL = 2,
  VerticalLine = 3,
  Square = 4
}

#[derive (Copy, Clone)]
struct Shape {
  position: Point,
  shape: SHAPES
}

#[derive (Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
  x: i32,
  y: i32
}

impl ops::Add<Point> for Point {
  type Output = Point;
  fn add(self, other: Point) -> Point {
    Point{x: self.x + other.x, y: self.y + other.y}
  }
}

impl SHAPES {
  fn from_i32(value: i32) -> SHAPES {
    match value {
      0 => SHAPES::HorizontalLine,
      1 => SHAPES::Cross,
      2 => SHAPES::BackwardsL,
      3 => SHAPES::VerticalLine,
      4 => SHAPES::Square,
      _ => {
        println!("Unknown val {}", value);
        SHAPES::HorizontalLine
      }
    }
  }
}

fn main(){
  let vec = inp::parse_file("day17.txt");
  solve(&vec);
}

fn _print_chamber(map: &HashSet<Point>, highest: i32) {
  for y in (0..highest+4).rev() {
    for x in 0..CHAMBER_WIDTH {
      if map.contains(&Point{x:x, y:y}) {
        print!("#");
      } else {
        print!(".");
      }
    }
    println!("");
  }
}

fn get_spaces_shape_occupies(in_shape: Shape) -> Vec<Point> {
  match in_shape.shape {
    SHAPES::HorizontalLine => {
      let mut out: Vec<Point> = Vec::new();
      for x_pos in in_shape.position.x..in_shape.position.x + 4 {
        out.push(Point{x: x_pos,y:in_shape.position.y});
      }
      return out;
    },
    SHAPES::Cross => {
      return vec![Point{x: in_shape.position.x, y: in_shape.position.y+ 1}, Point{x: in_shape.position.x + 1, y: in_shape.position.y},
        Point{x: in_shape.position.x + 1, y: in_shape.position.y + 1}, Point{x: in_shape.position.x + 1, y: in_shape.position.y + 2},
        Point{x: in_shape.position.x + 2, y: in_shape.position.y + 1}];
    },
    SHAPES::BackwardsL => {
      return vec![Point{x: in_shape.position.x, y: in_shape.position.y}, Point{x: in_shape.position.x + 1, y: in_shape.position.y},
        Point{x: in_shape.position.x + 2, y: in_shape.position.y}, Point{x: in_shape.position.x + 2, y: in_shape.position.y + 1},
        Point{x: in_shape.position.x + 2, y: in_shape.position.y + 2}];
    },
    SHAPES::VerticalLine => {
      let mut out: Vec<Point> = Vec::new();
      for y_pos in in_shape.position.y..in_shape.position.y + 4 {
        out.push(Point{x: in_shape.position.x,y:y_pos});
      }
      return out;
    },
    SHAPES::Square => {
      return vec![Point{x: in_shape.position.x, y: in_shape.position.y}, Point{x: in_shape.position.x + 1, y: in_shape.position.y},
        Point{x: in_shape.position.x, y: in_shape.position.y + 1}, Point{x: in_shape.position.x + 1, y: in_shape.position.y + 1}];
    }
  }
}

fn get_highest_pos(spaces: &HashSet<Point>) -> i32 {
  spaces.iter().fold(0, |acc, x| std::cmp::max(acc, x.y))
}

fn can_move(curr_shape: Shape, in_move: Point, occupied_spaces: &HashSet<Point>) -> bool {
  let new_pos = curr_shape.position + in_move;
  for point in get_spaces_shape_occupies(Shape{position:new_pos, shape:curr_shape.shape}) {
    if occupied_spaces.contains(&point) || point.x < 0 || point.x >= CHAMBER_WIDTH {
      return false;
    }
  }
  true
}

// Solution
fn solve(input: &Vec<String>) {
  let input = input[0].chars().collect::<Vec<char>>();
  let mut input_index: usize = 0;
  let mut occupied_spaces: HashSet<Point> = HashSet::new();
  // Add floor
  for x in -1..CHAMBER_WIDTH+1 {
    occupied_spaces.insert(Point{x:x, y:0});
  }
  let mut curr_highest_spot = 0;
  let mut curr_shape = Shape{position: Point{x:2, y:curr_highest_spot+4}, shape: SHAPES::HorizontalLine};
  // Drop 2022 rocks
  for rock_num in 0..TOTAL_NUM_ROCKS {
    if rock_num == 2022 {
      println!("Part 1: {}", curr_highest_spot);
    }
    if rock_num % 1000000 == 0 {
      println!("{}", rock_num);
    }

    //_print_chamber(&occupied_spaces, curr_highest_spot);
    //inp::pause();
    loop {
      let dir: char = match input.get(input_index) {
        Some(x) => *x,
        None => {
          input_index = 0;
          input[input_index]
        }
      };
      let x_dir = match dir {
        '<' => -1,
        '>' => 1,
        _ => {
          println!("Invalid char {}", dir);
          0
        }
      };
      let horizontal_move = Point{x: x_dir, y: 0};
      if can_move(curr_shape, horizontal_move, &occupied_spaces) {
        //println!("Moved {}", dir);
        curr_shape.position = curr_shape.position + horizontal_move; 
      } 
      let vertical_move = Point{x: 0, y: -1};
      if !can_move(curr_shape, vertical_move, &occupied_spaces) {
        // Can't move down, come to rest
        //println!("Came to rest");
        for point in get_spaces_shape_occupies(curr_shape) {
          occupied_spaces.insert(point);
        }
        curr_highest_spot = get_highest_pos(&occupied_spaces);
        curr_shape = Shape{position: Point{x:2, y:curr_highest_spot +4}, shape: SHAPES::from_i32((curr_shape.shape as i32 +1) %5) };
        input_index += 1;
        break;
      }
      //println!("Falling");
      curr_shape.position = curr_shape.position + vertical_move; 
      input_index += 1;
    }
  }
  println!("Part 1: {}", curr_highest_spot);
  println!("Part 2: {}", 0);
}
