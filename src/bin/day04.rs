use std::vec::Vec;

struct Section {
	min: u32,
	max: u32
}

fn main(){
    let vec = inp::parse_file("day04.txt");
    solve(&vec);
}

fn do_overlap(pair1: Section, pair2: Section) -> bool {
	return pair1.min <= pair2.max && pair2.min <= pair1.max 
}

// Solution
fn solve(input: &Vec<String>) {
	let mut counter = 0;
	for line in input {
		let mut line = line.split(",");
		let mut section1 = line.next().unwrap().split("-");
		let section1 = Section{min:section1.next().unwrap().parse().unwrap(), max:section1.next().unwrap().parse().unwrap()};
		let mut section2 = line.next().unwrap().split("-");
		let section2 = Section{min:section2.next().unwrap().parse().unwrap(), max:section2.next().unwrap().parse().unwrap()};
		if (section1.min <= section2.min && section1.max >= section2.max) ||
		(section2.min <= section1.min && section2.max >= section1.max) {
			counter += 1;
		}
	}
    println!("Part 1: {}", counter);

	let mut counter = 0;
	for line in input {
		let mut line = line.split(",");
		let mut section1 = line.next().unwrap().split("-");
		let section1 = Section{min:section1.next().unwrap().parse().unwrap(), max:section1.next().unwrap().parse().unwrap()};
		let mut section2 = line.next().unwrap().split("-");
		let section2 = Section{min:section2.next().unwrap().parse().unwrap(), max:section2.next().unwrap().parse().unwrap()};
		if do_overlap(section1, section2) {
			counter += 1;
		}
	}
    println!("Part 2: {}", counter);
}
