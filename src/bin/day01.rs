use std::vec::Vec;

fn main(){
    let vec = inp::parse_file("day01.txt");
    solve(&vec);
}

// Solution for part 2
fn solve(input: &Vec<String>) {
    let mut curr_elf_amt = 0;
    let mut total_cal_vec: Vec<u32> = Vec::new();
    for line in input {
        if !line.is_empty() {
            let amt = line.parse::<u32>().expect("Could not parse");
            curr_elf_amt += amt;
        } else {
            total_cal_vec.push(curr_elf_amt);
            curr_elf_amt = 0;
        }
    }
    total_cal_vec.push(curr_elf_amt);
    total_cal_vec.sort();
    total_cal_vec.reverse();
    println!("Part 1: {}", total_cal_vec[0]);

    let part_2_total: u32 = total_cal_vec[..3].iter().sum();
    println!("Part 2: {}", part_2_total);
}