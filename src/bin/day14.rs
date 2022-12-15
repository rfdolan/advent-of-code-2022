use std::i32::MIN;
use std::vec::Vec;
use std::collections::HashSet;

#[derive (Eq, PartialEq, Hash, Clone, Copy)]
struct Point {
	x: i32,
	y: i32
}
fn main(){
    let vec = inp::parse_file("day14.txt");
    solve(&vec);
}

fn get_rocks(input: &Vec<String>) -> HashSet<Point> {
	let mut map:HashSet<Point> = HashSet::new();
	for rock in input {
		let rock = rock.split(" -> ")
      .map(|x| x.split(",")
        .map(|y| y.parse::<i32>()
          .unwrap())
        .collect::<Vec<i32>>())
      .collect::<Vec<Vec<i32>>>();
		let mut rock_it = rock.iter();
		let last = rock_it.next().expect("Not enough rocks!");
		let mut last_point = Point{x: last[0], y:last[1]};
		map.insert(last_point);
		while let Some(next_point) = rock_it.next() {
      let next_point = Point{x:next_point[0], y:next_point[1]};
			while next_point != last_point {
        let new_x = (next_point.x - last_point.x).signum() + last_point.x;
        let new_y = (next_point.y - last_point.y).signum() + last_point.y;
        let new_point = Point{x:new_x, y:new_y};
        map.insert(new_point);
        last_point = new_point;
			}
		}
	}
  map
}

fn drop_grain_1(map: &HashSet<Point>, lowest_point: i32) -> (bool, Point) {
  let mut grain_position = Point{x:500, y:0};
  while grain_position.y <= lowest_point {
    let mut next_position = Point{x:grain_position.x, y:grain_position.y + 1};
    match map.get(&next_position) {
      Some(_) => {
        next_position.x -= 1;
        match map.get(&next_position) {
          Some(_) => {
            next_position.x += 2;
            match map.get(&next_position) {
              Some(_) => {
                return (true, grain_position);
            },
              None => grain_position = next_position
            }
          },
          None => grain_position = next_position
        }
      },
      None => grain_position = next_position
    }
  }
  return (false, Point{x:-1,y:-1});
}

fn part1(map: &HashSet<Point>) -> i32 {
  let mut map = map.clone();
  let mut num_grains = 0;
  let lowest_point = map.iter().fold(MIN, |acc, x| std::cmp::max(x.y, acc));
  loop {
    let (came_to_rest, new_grain_pos) = drop_grain_1(&map, lowest_point);
    if !came_to_rest {
      // Grain fell into the void.
      return num_grains;
    }
    // Grain came to rest.
    map.insert(new_grain_pos);
    num_grains += 1;
  }
}

fn drop_grain_2(map: &HashSet<Point>, lowest_point: i32) -> Point {
  let mut grain_position = Point{x:500, y:0};
  while grain_position.y < lowest_point {
    let mut next_position = Point{x:grain_position.x, y:grain_position.y + 1};
    match map.get(&next_position) {
      Some(_) => {
        next_position.x -= 1;
        match map.get(&next_position) {
          Some(_) => {
            next_position.x += 2;
            match map.get(&next_position) {
              Some(_) => {
                return grain_position;
            },
              None => grain_position = next_position
            }
          },
          None => grain_position = next_position
        }
      },
      None => grain_position = next_position
    }
  }
  return grain_position;
}

fn part2(map: &HashSet<Point>) -> i32 {
  let mut map = map.clone();
  let mut num_grains = 0;
  let lowest_point = map.iter().fold(MIN, |acc, x| std::cmp::max(x.y, acc)) + 1;
  loop {
    let new_grain_pos = drop_grain_2(&map, lowest_point);
    num_grains += 1;
    if new_grain_pos == (Point{x: 500, y:0}){
      return num_grains;
    }
    map.insert(new_grain_pos);
  }
}

// Solution
fn solve(input: &Vec<String>) {
  let map = get_rocks(input);
  println!("Part 1: {}", part1(&map));
	println!("Part 2: {}", part2(&map));
}
