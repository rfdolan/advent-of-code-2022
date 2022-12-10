use std::vec::Vec;
use std::collections::HashMap;

const SIZE_THRESHOLD: u32 = 100000;
const TOTAL_SIZE: u32 = 70000000;
const REQUIRED_SPACE: u32 = 30000000;

#[derive (Clone)]
struct File {
	size: u32,
	_name: String
}

#[derive (Clone)]
struct Directory {
	files: Vec<File>,
	subdirectories: Vec<String>
}

fn _print_filesystem(filesystem: HashMap<String, Directory>) {
	println!("===PRINTING DIRECTORIES===");
	for x in filesystem.iter() {
		println!("DIR: {}", x.0);
		for file in &x.1.files {
			println!("{}/{}: size: {}", x.0, file._name, file.size );
		}
		for sub in &x.1.subdirectories {
			println!("sibdir:{}", sub);
		}
	}
}

fn main(){
    let vec = inp::parse_file("day07.txt");
    solve(&vec);
}

fn concat_dir_names(curr_dir: String, target_dir: String) -> String {
	if curr_dir != "/" {
		return format!("{}/{}", curr_dir,target_dir);
	} else {
		return format!("{}{}", curr_dir, target_dir);
	}
}

fn construct_filesystem_recursive(input: &[String], current_filesystem: HashMap<String, Directory>, 
		curr_dir: String) -> (HashMap<String, Directory>, usize) {
	let mut new_filesystem = current_filesystem;
	let mut lines_read = 0;
	while lines_read < input.len() {
		let line = &input[lines_read];
		lines_read += 1;
		let split: Vec<&str> = line.split(" ").collect::<Vec<&str>>();
		if split[0] == "$" {
			// Command
			if split[1] == "cd" {
				let target_dir = split[2].to_string();
				if target_dir == ".." {
					// return current
					return (new_filesystem, lines_read);
				} else {
					let (new_part, were_read) = construct_filesystem_recursive(&input[lines_read..], 
						new_filesystem.clone(), 
						concat_dir_names(curr_dir.clone(), target_dir.clone()));
						new_filesystem.extend(new_part);
						lines_read += were_read;
				}
			}
		} else if split[0] == "dir" {
			let new_dir_name = concat_dir_names(curr_dir.clone(), split[1].to_string());

			// Create dir, throw error if it already exists.
			if new_filesystem.keys().collect::<Vec<&String>>().contains(&&new_dir_name) {
				println!(">Error: directory {} already exists", new_dir_name);
			} else {
				new_filesystem.insert(new_dir_name.clone(), Directory{files: Vec::new(), subdirectories: Vec::new()});
				match new_filesystem.get(&curr_dir) {
					Some(x) =>  {
						let mut temp = x.clone();
						temp.subdirectories.push(new_dir_name.clone());
						new_filesystem.insert(curr_dir.clone(), temp);
					},
					None => println!(">Error: attempt to add to directory {} before creating it.", new_dir_name)
				}
			}
		} else {
			// File, create it
			let new_file = File{size: split[0].parse::<u32>().expect(&format!(">Error: file size {} was not a number", split[0])), 
				_name: split[1].to_string()};
			match new_filesystem.get(&curr_dir) {
				Some(x) =>  {
					let mut temp = x.clone();
					temp.files.push(new_file);
					new_filesystem.insert(curr_dir.clone(), temp);
					
				},
				None => println!(">Error: attempt to add to directory {} before creating it.", curr_dir)
			}
		}
		//_print_filesystem(new_filesystem.clone());
	}
	(new_filesystem, lines_read)
	
}

fn construct_filesystem(input: &Vec<String>) -> HashMap<String, Directory> {
	let mut filesystem: HashMap<String, Directory> = HashMap::new();
	filesystem.insert("/".to_string(), Directory {files: Vec::new(), subdirectories: Vec::new() });

	return construct_filesystem_recursive(&input[1..], filesystem, "/".to_string()).0;
}

fn calculate_score_less_than_threshold(dir: Directory, filesystem: HashMap<String, Directory>) -> (u32, u32) {
	let mut dir_size = 0;
	let mut score = 0;
	for file in dir.files {
		dir_size += file.size;
	}
	for subdir in dir.subdirectories {
		let res = calculate_score_less_than_threshold(filesystem.clone().get(&subdir.clone()).unwrap().clone(), filesystem.clone());
		dir_size += res.0;
		score += res.1;
	}
	if dir_size < SIZE_THRESHOLD {
		return (dir_size, score + dir_size);
	}
	(dir_size, score)
}

fn get_smallest_above_size(dir: Directory, filesystem: HashMap<String, Directory>, 
	desired_space: u32, curr_min: u32) -> (u32, u32) {
	let mut dir_size = 0;
	let mut curr_min_above_threshold = curr_min;
	for file in dir.files {
		dir_size += file.size;
	}
	for subdir in dir.subdirectories {
		let res = get_smallest_above_size(filesystem.clone().get(&subdir.clone()).unwrap().clone(), filesystem.clone(), desired_space,curr_min_above_threshold);
		dir_size += res.0;
		if res.1 < curr_min_above_threshold && res.1 > desired_space {
			curr_min_above_threshold = res.1;
		}
	}
	if dir_size > desired_space {
		if dir_size < curr_min_above_threshold {
			return (dir_size, dir_size);
		}
		return (dir_size, curr_min_above_threshold);
	}
	(dir_size, curr_min_above_threshold)
}

// Solution
fn solve(input: &Vec<String>) {
	let filesystem = construct_filesystem(input);
	let root = match filesystem.get("/") {
		Some(x) => x,
		None => {
			println!("No root!"); 
			return; 
		}
	};
	//_print_filesystem(filesystem.clone());
	let (filesystem_size, score) = calculate_score_less_than_threshold(root.clone(), filesystem.clone());

    println!("Part 1: {}", score);
	println!("Filesystem is currently {}/{}, so we need {} space", filesystem_size, TOTAL_SIZE, filesystem_size - (TOTAL_SIZE - REQUIRED_SPACE));
    println!("Part 2: {}", get_smallest_above_size(root.clone(), filesystem.clone(), filesystem_size - (TOTAL_SIZE - REQUIRED_SPACE), TOTAL_SIZE).1);
}
