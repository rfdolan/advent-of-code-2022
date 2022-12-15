use std::vec::Vec;

const BLOCK_SIZE: usize = 4;

fn main(){
  let vec = inp::parse_file("day05.txt");
  solve(&vec);
}

fn _print_stacks(input: &Vec<Vec<char>>) {
  println!("========");
  let mut num = 0;
  for stack in input.iter() {
    if stack.len() > 0 {
      print!("{}: ", num + 1);
      for c in stack {
        print!("{},",c);
      }
      println!("");
    }
    num += 1;
  }
  println!("========");
}

fn parse_initial_stacks(input: &Vec<String>) -> (Vec<Vec<char>>, usize) {
  let mut line_num = 0;
  let mut stacks: Vec<Vec<char>> = vec![Vec::new(); 30]; // idk about 30 should work

  // Parse initial state into stacks.
  for string in input {
    line_num += 1;
    if string == "" {
      break;
    }
    let mut i = 0;
    while i * BLOCK_SIZE < string.len(){
      let char_at_pos = string.chars().collect::<Vec<char>>()[(i*BLOCK_SIZE) + 1];
      if char_at_pos.is_alphabetic() {
        let mut new_vec = stacks[i].clone();
        new_vec.push(char_at_pos);
        stacks[i] = new_vec;
      }
      i += 1;
    }
  }
  (stacks, line_num)
}

fn get_top_of_each_stack(input: Vec<Vec<char>>) -> String {
  let mut end_string = Vec::new();
  for mut stack in input {
    match stack.pop() {
      Some(x) => end_string.push(x),
      None => ()
    }
  }
  end_string.iter().collect::<String>()

}

// Solution
fn solve(input: &Vec<String>) {
  let (stacks, line_num) = parse_initial_stacks(input);
  
  // Reverse them so that things that should be on top are.
  let mut new_stacks = Vec::new();
  for stack in stacks {
    let new_stack = stack.clone().iter().rev().map(|x| *x).collect::<Vec<char>>();
    new_stacks.push(new_stack);
  }

  // Copy into two different states for each part.
  let mut stacks1 = new_stacks.clone();
  let mut stacks2 = new_stacks.clone();
  //print_stacks(&stacks);

  // Go through moves
  for string in &input[line_num..] {
    let nums = string.split(" ").collect::<Vec<_>>();
    let num_to_move = nums[1].parse::<u32>().unwrap();
    let src: usize = nums[3].parse::<usize>().unwrap() - 1;
    let dest: usize = nums[5].parse::<usize>().unwrap() - 1;
    let mut src_array_1 = stacks1[src].clone();
    let mut dest_array_1 = stacks1[dest].clone();
    let mut src_array_2 = stacks2[src].clone();
    let mut dest_array_2 = stacks2[dest].clone();
    let mut staging_array : Vec<char>= Vec::new();

    // Push directly for part 1. Push to stagin array for part 2.
    for _ in 0..num_to_move {
      dest_array_1.push(src_array_1.pop().unwrap());
      staging_array.push(src_array_2.pop().unwrap());
    }
    stacks1[src] = src_array_1;
    stacks1[dest] = dest_array_1;

    // Pop from staging array for part 2.
    while staging_array.len() > 0 {
      dest_array_2.push(staging_array.pop().unwrap());
    }
    stacks2[src] = src_array_2;
    stacks2[dest] = dest_array_2;
  }

  println!("Part 1: {}", get_top_of_each_stack(stacks1));
  println!("Part 2: {}", get_top_of_each_stack(stacks2));
}
