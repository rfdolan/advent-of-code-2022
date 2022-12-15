use std::vec::Vec;

fn main(){
	let vec = inp::parse_file("day08.txt").iter()
		.map(|x| x.chars()
			.into_iter()
			.map(|c| c.to_digit(10)
				.unwrap())
			.collect::<Vec<u32>>())
		.collect::<Vec<Vec<u32>>>();
	solve(&vec);
}

fn is_visible(tree: u32, trees: &[u32]) -> bool {
	for t in trees {
		if *t >= tree {
			return false;
		}
	}
	true
}

fn _print_forest(input: &Vec<Vec<u32>>) {
	for row in input {
		for col in row {
			print!("{}", *col);
		}
		println!("");
	}
}

fn transpose_forest(input: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
	let mut output:Vec<Vec<u32>> = vec![Vec::new(); input[0].len()];
	for row in 0..input.len() {
		for col in 0..input[row].len() {
			output[col].insert(row, input[row][col]);
		}
	}
	output
}

fn get_visible_trees(tree: u32, tree_line: &Vec<&u32>) -> u32 {
	let mut visible = 0;
	for t in tree_line {
		visible += 1;
		if **t >= tree {
			return visible;
		}
	}
	visible
}

// Solution
fn solve(input: &Vec<Vec<u32>>) {
	let mut num_visible = 0;
	let mut curr_most_visible_trees = 0;
	let transposed_forest = transpose_forest(input);
	for row in 0..input.len() {
		let curr_row = &input[row];
		for col in 0..curr_row.len() {
			let curr_col = &transposed_forest[col];
			let curr_tree = curr_row[col];

			let left_trees = &curr_row[..col];
			let right_trees = &curr_row[col+1..];
			let up_trees = &curr_col[..row];
			let down_trees = &curr_col[row+1..];

			let visible_horizontal = is_visible(curr_tree,left_trees) ||
				is_visible(curr_tree, right_trees);
			let visible_vertical = is_visible(curr_tree, up_trees) ||
				is_visible(curr_tree, down_trees);
			
			if visible_horizontal || visible_vertical{
				num_visible += 1;
			}

			let visible_trees = get_visible_trees(curr_tree, &left_trees.iter().rev().collect::<Vec<&u32>>()) *
				get_visible_trees(curr_tree, &right_trees.iter().collect::<Vec<&u32>>()) *
				get_visible_trees(curr_tree, &up_trees.iter().rev().collect::<Vec<&u32>>()) *
				get_visible_trees(curr_tree, &down_trees.iter().collect::<Vec<&u32>>());
			if visible_trees > curr_most_visible_trees {
				curr_most_visible_trees = visible_trees;
			}

		}
	}
	println!("Part 1: {}", num_visible);
	println!("Part 2: {}", curr_most_visible_trees);
}
