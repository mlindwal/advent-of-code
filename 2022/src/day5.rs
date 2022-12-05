use std::collections::HashMap;
use std::fs;
use regex::Regex;
use lazy_static::lazy_static;
use itertools::Itertools;

lazy_static! {
    static ref RGX: Regex = Regex::new(r"\d+").unwrap();
}

pub fn run() {
    let input_str = fs::read_to_string("input/day5.txt").unwrap();

    let mut map_first_challenge = HashMap::new();
    let mut map_second_challenge = HashMap::new();

    input_str.lines()
        .for_each(|row| {
            if row.starts_with("[") {
                parse_boxes(row, &mut map_first_challenge);
            } else if row.is_empty() {
                for (_, v) in &mut map_first_challenge {
                    v.reverse();
                }
                map_second_challenge = map_first_challenge.clone();
            } else if row.starts_with("move") {
                let (move_count, from, to) = parse_operations(row);
                let mut stack = vec![];

                for _ in 0..move_count {
                    let taken = map_first_challenge.get_mut(&from).unwrap().pop();
                    map_first_challenge.get_mut(&to).unwrap().push(taken.unwrap());

                    let taken = map_second_challenge.get_mut(&from).unwrap().pop();
                    stack.push(taken.unwrap());
                }
                while let Some(val) = stack.pop() {
                    map_second_challenge.get_mut(&to).unwrap().push(val);
                }
            }
        });

    println!("------");
    println!("Day 5 challenge 1: {:?}", get_top_layer_str(map_first_challenge));
    println!("Day 5 challenge 2: {:?}", get_top_layer_str(map_second_challenge));
}

fn get_top_layer_str(map: HashMap<usize, Vec<char>>) -> String {
    map.keys().sorted()
        .map(|idx| *map.get(idx).unwrap().last().unwrap())
        .collect()
}

fn parse_operations(row: &str) -> (usize, usize, usize) {
    let mut iter = RGX.find_iter(row);
    if let (Some(move_count), Some(from), Some(to)) = (iter.next(), iter.next(), iter.next()) {
        (
            move_count.as_str().parse::<usize>().unwrap(),
            from.as_str().parse::<usize>().unwrap(),
            to.as_str().parse::<usize>().unwrap()
        )
    } else {
        panic!("Invalid move operator for input: \"{}\"", row)
    }
}

fn parse_boxes(str: &str, map: &mut HashMap<usize, Vec<char>>) {
    str.match_indices("[").for_each(|idx| {
        let char = str.chars().nth(idx.0 + 1).unwrap();
        let idx = (idx.0 / 4) + 1;

        if map.contains_key(&idx) {
            map.get_mut(&idx).unwrap().push(char);
        } else {
            map.insert(idx,  vec![char]);
        }
    });
}
