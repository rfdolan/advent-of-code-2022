use std::vec::Vec;
use std::collections::{HashMap, HashSet, VecDeque};
use std::cmp;

const THE_ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

#[derive (Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
	x: i32,
	y: i32
}

fn main(){
    let vec = inp::parse_file("day12.txt");
    solve(&vec);
}

fn bfs(start: Point, end: Point, graph: &HashMap<Point, i32>) -> u32 {
	let mut visited: HashSet<Point> = HashSet::new();
	let mut to_visit: VecDeque<(Point, u32, i32)> = VecDeque::new();
	to_visit.push_front((start, 1, 1));
	let mut smallest_path = graph.len() as u32;
	loop {
		//println!("loop");
		match to_visit.pop_back() {
			Some(x) => {
				let (new_position, steps_to_here, last_height) = x;
				// If we haven't visited and this posiiton does exist
				if !visited.contains(&new_position) && graph.contains_key(&new_position) {
					// If we can actually go here 
					let next_height = graph[&new_position];
					if last_height >= next_height || next_height - last_height <=1 {
						visited.insert(new_position);
						if new_position == end {
							if steps_to_here < smallest_path {
								smallest_path = steps_to_here;
							}
						} else {
							// Add adjacent to_visit queue.
							to_visit.push_front((Point{x:new_position.x + 1, y: new_position.y}, steps_to_here+1, next_height));
							to_visit.push_front((Point{x:new_position.x - 1, y: new_position.y}, steps_to_here+1, next_height));
							to_visit.push_front((Point{x:new_position.x, y: new_position.y +1}, steps_to_here+1, next_height));
							to_visit.push_front((Point{x:new_position.x, y: new_position.y -1}, steps_to_here+1, next_height));
						}
					}
				}
			},
			None => return smallest_path
		}
	}

}

// Solution
fn solve(input: &Vec<String>) {
	let mut height_map: HashMap<Point, i32> = HashMap::new();
	let mut start = Point{x:-1, y:-1};
	let mut end = Point{x:-1, y:-1};
	for y in 0..input.len() {
		let curr_line = input[y].chars().collect::<Vec<char>>();
		for x in 0..curr_line.len() {
			let curr_char = curr_line[x];
			match curr_char {
				'S' => {
					start = Point{x:x as i32, y:y as i32};
					height_map.insert(start, 0);
				},
				'E' => {
					end = Point{x:x as i32, y:y as i32};
					height_map.insert(end, 26);
				},
				my_char => {
					let height = THE_ALPHABET.find(my_char).expect(&format!("{} not in the alphabet",my_char));
					height_map.insert(Point{x:x as i32, y:y as i32}, height as i32);
				}
			}
		}
	}

	let mut min_path_from_any = height_map.len() as u32;
	for x in height_map.iter() {
		if *x.1 == 0 {
			min_path_from_any = cmp::min(bfs(*x.0, end,&height_map), min_path_from_any);

		}
	}

	println!("Part 1: {}", bfs(start, end, &height_map) -1);
	println!("Part 2: {}", min_path_from_any - 1);
}
