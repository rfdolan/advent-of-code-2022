use std::vec::Vec;
use std::collections::HashMap;

const LINES_PER_MONKEY: usize = 7;

#[derive (Clone)]
struct Monkey {
	id: u32,
	pub items: Vec<u128>,
	operation: String,
	operation_num: u128,
	test_divisor: u128,
	true_target: u128,
	false_target: u128
}

fn _print_monkies(input: &Vec<Monkey>) {
	println!("");
	for m in input {
		println!("Monkey {}:", m.id);
		print!("\t");
		for i in m.items.iter() {
			print!("{}, ", i);
		}
		println!("");
		println!("\t{} {}", m.operation, m.operation_num);
		println!("\t/ {}", m.test_divisor);
		println!("\tTrue: {}", m.true_target);
		println!("\tFalse: {}", m.false_target);
		println!("");
	}
}

fn get_monkey(input: &[String], id: u32) -> Monkey {
	// Don't look please.
	let start_items = input[1].split(",")
		.collect::<Vec<&str>>()
		.iter()
		.map(|&x| x.chars()
			.into_iter()
			.filter(|x| x.is_digit(10))
			.collect::<String>())
			.map(|x| x.parse::<u128>()
				.expect(&format!("Could not parse {} as int.", x)))
		.collect::<Vec<u128>>();
	let split = &input[2].trim().split(" ").collect::<Vec<&str>>()[4..];
	let op = split[0];
	let op_num = match split[1].parse::<u128>() {
		Ok(x) => x,
		Err(_) => 0
	};
	let test = input[3].chars()
		.into_iter()
		.filter(|x| x.is_digit(10))
		.collect::<String>()
		.parse::<u128>()
		.unwrap();
	let true_target = input[4].chars()
		.into_iter()
		.filter(|x| x.is_digit(10))
		.collect::<String>()
		.parse::<u128>()
		.unwrap();
	let false_target = input[5].chars()
		.into_iter()
		.filter(|x| x.is_digit(10))
		.collect::<String>()
		.parse::<u128>()
		.unwrap();
	Monkey{id:id, items:start_items, operation:op.to_string(), operation_num: op_num, test_divisor:test,
		true_target:true_target,false_target:false_target}
}

fn main(){
	let vec = inp::parse_file("day11.txt");
	solve(&vec);
}

fn do_op(op: &str, op_num: u128, num: u128) -> u128 {
	let op_num = match op_num {
		0 => num,
		x => x
	};
	match op {
		"+" => op_num + num,
		"*" => op_num * num,
		_ => 0
	}
}

fn run_sim(input: &Vec<Monkey>, num_rounds: usize, divide_by_3: bool) -> u128 {
	let magic_num = input.iter().fold(1, |acc, x| x.test_divisor * acc );
	let mut monkey_list = input.clone();
	let mut times_each_monkey_inspected: HashMap<u32, u128> = HashMap::new();
	for x in 0..input.len() {
		times_each_monkey_inspected.insert(x as u32, 0);
	}
	for _ in 0..num_rounds {
		for i in 0..monkey_list.len() {
			let mut list_clone = monkey_list.clone();
			let monkey = &mut list_clone[i];
			while let Some(item) = monkey.items.pop() {
				// Inspect
				times_each_monkey_inspected.insert(monkey.id, times_each_monkey_inspected[&monkey.id] + 1);
				// Do operation
				let new = do_op(&monkey.operation, monkey.operation_num, item);
				let new = if divide_by_3 {new / 3} else {new};
				// Test, throw to monkey based on that
				if new % monkey.test_divisor == 0 {
					monkey_list[monkey.true_target as usize].items.push(if divide_by_3{new}else{new %magic_num});
				} else {
					monkey_list[monkey.false_target as usize].items.push(if divide_by_3{new}else{new %magic_num});
				}
			}
			monkey_list[i].items.clear();
		}
	}
	let mut max = 0;
	let mut second_to_max = 0;
	for x in times_each_monkey_inspected.iter() {
		//println!("Monkey {} inspected {}", x.0, x.1);
		let num = *x.1;
		if num > max {
			second_to_max = max;
			max = num;
		} else if num > second_to_max {
			second_to_max = num;
		}
	}
	max * second_to_max

}

// Solution
fn solve(input: &Vec<String>) {
	let mut monkey_list: Vec<Monkey> = Vec::new();
	for x in 0..((input.len()/LINES_PER_MONKEY)+1) {
		monkey_list.push(get_monkey(&input[x*LINES_PER_MONKEY..((x+1)*LINES_PER_MONKEY)-1], x as u32));
	}
	println!("Part 1: {}", run_sim(&monkey_list, 20, true));
	println!("Part 2: {}", run_sim(&monkey_list, 10000, false));
}
