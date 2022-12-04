use std::vec::Vec;

struct Section {
	min: u32,
	max: u32
}

fn main(){
    let vec = inp::parse_file("day04.txt");
    solve(&vec);
}

fn do_overlap(pair1: &Section, pair2: &Section) -> bool {
	return pair1.min <= pair2.max && pair2.min <= pair1.max 
}

fn contains(pair1: &Section, pair2: &Section) -> bool {
	return pair1.min <= pair2.min && pair1.max >= pair2.max;
}

// Solution
fn solve(input: &Vec<String>) {
	let mut contained_counter = 0;
	let mut overlap_counter = 0;
	for line in input {
		let mut line = line.split(",");

		let mut section1 = line.next()
			.expect("Not enough sections in line.")
			.split("-")
			.map(|x| x.parse::<u32>()
				.expect(&format!("Couldn't parse {} as u32.", x)));
		let section1 = Section{
			min:section1.next()
				.expect("Not enough numbers in section."), 
			max:section1.next()
				.expect("Not enough numbers in section.")};

		let mut section2 = line.next()
			.expect("Not enough sections in line.")
			.split("-")
			.map(|x| x.parse::<u32>()
				.expect(&format!("Couldn't parse {} as u32.", x)));
		let section2 = Section{
			min:section2.next()
				.expect("Not enough numbers in section."), 
			max:section2.next()
				.expect("Not enough numbers in section.")};

		if do_overlap(&section1, &section2) {
			overlap_counter += 1;
			if contains(&section1, &section2) || 
				contains(&section2, &section1) {
				contained_counter += 1;
			}
		}
	}
    println!("Part 1: {}", contained_counter);
    println!("Part 2: {}", overlap_counter);
}
