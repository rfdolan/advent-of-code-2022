use std::vec::Vec;

const POSSIBLE_VALS: [u32;3] = [1,2,3];
fn main(){
    let vec = inp::parse_file("day02.txt");
    solve(&vec);
}

fn string_to_points(hand: &str) -> u32 {
	match hand {
		"A"|"X" => 1,
		"B"|"Y" => 2,
		"C"|"Z" =>3,
		_ => 0
	}
}

fn get_winning_val(loser: u32) -> u32 {
	match loser {
		1 => 2,
		2 => 3,
		3 => 1,
		_ => 0
	}
}

fn play_round(turn: (u32, u32)) -> u32 {
	let played_num = turn.1;
	let mut outcome_points = 0;
	if get_winning_val(turn.0) == turn.1 {
		outcome_points = 6;
	}
	if turn.0 == turn.1 {
		outcome_points = 3;
	}
	played_num + outcome_points
}

fn play_round_2(turn: (u32, u32)) -> u32 {
	match turn.1 {
		1 => *POSSIBLE_VALS.iter().filter(|x| !vec![turn.0, get_winning_val(turn.0)].contains(x)).next().unwrap(),
		2 => turn.0 + 3,
		3 => get_winning_val(turn.0) + 6,
		_ => 0
	}
}

// Solution for part 1
fn solve(input: &Vec<String>) {
	let mut splitted = input.iter().map(|x| x.split(" ")).collect::<Vec<_>>();
	let mut total_score_1 = 0;
	let mut total_score_2 = 0;
	for split in splitted.iter_mut() {
		let tuple = (string_to_points(split.next().unwrap()), string_to_points(split.next().unwrap()));
		total_score_1 += play_round(tuple);
		total_score_2 += play_round_2(tuple);
	}
    println!("Part 1: {}", total_score_1);
    println!("Part 2: {}", total_score_2);
}
