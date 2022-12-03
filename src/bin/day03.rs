use std::vec::Vec;

const THE_ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
fn main(){
    let vec = inp::parse_file("day03.txt");
    solve(&vec);
}

fn get_char_value(input: char) -> u32 {
	match THE_ALPHABET.find(input) {
		Some(x) => x as u32 + 1,
		None => 0
	}
}

// Solution for part 1
fn solve(input: &Vec<String>) {
	let mut score1 = 0;
	let mut score2 = 0;
	let mut possible_letters = THE_ALPHABET.to_string();
	let mut bag_number = 0;

	for bag in input {
		let mut intersection = bag[..(bag.len()/2)]
		    .chars()
			.into_iter()
			.filter(|x| bag[(bag.len()/2)..]
			.contains(*x))
			.collect::<Vec<char>>();
		intersection.sort();  // Have to sort to do dedup.
		intersection.dedup();
		score1 += intersection
			.iter()
			.fold(0, |acc, x| acc + get_char_value(*x));

		bag_number += 1;
		possible_letters = possible_letters
			.chars()
			.into_iter()
			.filter(|x| bag
			.contains(*x))
			.collect();

		if bag_number == 3 {
			score2 += get_char_value(possible_letters
			.chars()
			.next()
			.expect("No badge shared between all 3 bags."));
			bag_number = 0;
			possible_letters = THE_ALPHABET.to_string();
		}
	}

    println!("Part 1: {}", score1);
    println!("Part 2: {}", score2);
}
