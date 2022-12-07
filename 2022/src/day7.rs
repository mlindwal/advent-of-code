use std::collections::{HashMap};
use std::fs;

use itertools::Itertools;

const MAX_SIZE: i32 = 100000;
const MIN_CAPACITY: i32 = 40000000;

pub fn run() {
    let input_str = fs::read_to_string("input/day7.txt").unwrap();
    let path_map = create_path_map(input_str);

    println!("------");
    println!("Day 7 challenge 1: {:?}", challenge_one(&path_map));
    println!("Day 7 challenge 2: {:?}", challenge_two(&path_map));
}

fn challenge_one(path_map: &HashMap<String, i32>) -> i32 {
    path_map.values()
        .map(|a| *a)
        .filter(|dir_size| *dir_size < MAX_SIZE)
        .reduce(|a, b| a + b)
        .unwrap()
}

fn challenge_two(path_map: &HashMap<String, i32>) -> i32 {
    let total_size = *path_map.get("/").unwrap();

    path_map.values()
        .map(|a| *a)
        .sorted()
        .find(|dir_size| total_size - dir_size <= MIN_CAPACITY)
        .unwrap()
}

fn create_path_map(input: String) -> HashMap<String, i32> {
    let mut map = HashMap::new();
    let mut pwd = vec!["/"];

    input.lines().for_each(|row| {
        let mut split = row.split_ascii_whitespace();

        match split.next() {
            Some("$") => {
                match split.next() {
                    Some("ls") => {},
                    Some("cd") => {
                        match split.next() {
                            Some("/") => pwd = vec!["/"],
                            Some("..") => {
                                pwd.pop();
                            },
                            Some(path) => pwd.push(path),
                            _ => {}
                        }
                    },
                    _ => {}
                }
            },
            Some("dir") => {},
            Some(fs) => {
                let file_size = fs.parse::<i32>().unwrap();

                let mut path = String::new();
                for sub_path in &pwd {
                    path.push_str(sub_path);
                    map.entry(path.clone())
                        .and_modify(|size| *size += file_size)
                        .or_insert(file_size);
                }
            },
            _ => {}
        }
    });

    map
}
