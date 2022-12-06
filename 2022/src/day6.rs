use std::collections::{VecDeque};
use std::fs;

pub fn run() {
    let input_str = fs::read_to_string("input/day6.txt").unwrap();

    println!("------");
    println!("Day 6 challenge 1: {:?}", find_marker(&input_str, 4));
    println!("Day 6 challenge 2: {:?}", find_marker(&input_str, 14));
}

fn find_marker(input: &String, nr_of_chars: usize) -> usize {
    let mut q = VecDeque::new();

    for (idx, chr) in input.chars().enumerate() {
        if q.contains(&chr) {
            while let Some(val) = q.pop_front() {
                if val == chr {
                    break;
                }
            }
        } else if q.len() == nr_of_chars {
            return idx;
        }
        q.push_back(chr);
    }
    panic!("No marker detected in input: \"{}\"", input)
}
