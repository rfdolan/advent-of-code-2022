use std::vec::Vec;
use std::collections::VecDeque;

fn main(){
    let vec = inp::parse_file("day06.txt");
    solve(&vec);
}

fn all_different(mut input: VecDeque<char>) -> bool {
	loop {
		match input.pop_front() {
			Some(x) => {
				if input.contains(&x) {
					return false;
				}
			},
			None => {
				return true;
			}
		}
	}
}

fn find_signal_position(in_string: &String, msg_len: usize) -> u32 {
	let mut last_four_seen : VecDeque<char> = VecDeque::new();
	let mut nums_processed = 0;

	for char in in_string.chars() {
		last_four_seen.push_back(char);
		nums_processed += 1;
		if last_four_seen.len() >= msg_len {
			if all_different(last_four_seen.clone()) {
				return nums_processed;
			} else {
				last_four_seen.pop_front();
			}
		}
	}
	0
}

// Solution
fn solve(input: &Vec<String>) {
    println!("Part 1: {}", find_signal_position(&input[0], 4));
    println!("Part 2: {}", find_signal_position(&input[0], 14));
}
