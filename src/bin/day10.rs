use std::vec::Vec;

const FIRST_RELEVANT_CYCLE: i32 = 20;
const CYCLES_UNTIL_NEXT_RELEVANT: i32 = 40;

fn main(){
    let vec = inp::parse_file("day10.txt");
    solve(&vec);
}

fn is_in_range(target: i32, sprite_center_pos: i32) -> bool {
	sprite_center_pos - 1 == target ||
		sprite_center_pos == target ||
		sprite_center_pos + 1 == target
}

// Solution
fn solve(input: &Vec<String>) {
	let mut reg_x = 1;
	let mut cycle = 0;
	let mut cycle_to_add = 0;
	let mut next_to_add = 0;
	let mut strength = 0;
	let mut input_iter = input.iter();

	loop {
		cycle += 1;
		if cycle_to_add <= cycle {
			// Do op
			reg_x += next_to_add;
			// Get next op
			match input_iter.next() {
				Some(x) => {
					let split = x.split(" ").collect::<Vec<&str>>();
					let op = split[0];
					match op {
						"noop" => {
							next_to_add = 0;
							cycle_to_add = cycle + 1;
						},
						"addx" => {
							next_to_add = split[1].parse::<i32>().expect(&format!("Couldn't parse {} as int.", split[1]));
							cycle_to_add = cycle + 2;
						},
						_ => println!("Unrecognized command {}", split[0])
					}
				},
				None => break
			}
			//println!("Cycle {} reg_x = {}", cycle, reg_x);
		}
		if is_in_range((cycle-1) % CYCLES_UNTIL_NEXT_RELEVANT, reg_x) {
			print!("#");
		} else {
			print!(".");
		} 
		if (cycle - FIRST_RELEVANT_CYCLE)%CYCLES_UNTIL_NEXT_RELEVANT == 0 {
			strength += cycle * reg_x;
		}
		if cycle % CYCLES_UNTIL_NEXT_RELEVANT == 0 {
			// Print line
			println!("");
		}
		//println!("{} * {} = {}",cycle,reg_x, (cycle *reg_x));
	}
    println!("Part 1: {}", strength);
}
