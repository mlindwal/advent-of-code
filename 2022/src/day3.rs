use std::fs;

pub fn run() {
    let input_str = fs::read_to_string("input/day3.txt").unwrap();

    let mut points_first_challenge = 0;

    input_str.lines()
        .for_each(|row| {
            let left = &row[0..row.len() / 2];

            let right = &row[row.len() / 2..];

            let mut matches = left.chars().filter(|chr| right.contains(*chr)).collect::<Vec<char>>();
            matches.sort();
            matches.dedup();
            matches.iter().for_each(|chr| points_first_challenge += get_chr_value(*chr));
        });


    let mut points_second_challenge = 0;
    let mut lines = input_str.lines();

    while let (Some(a), Some(b), Some(c)) = (lines.next(), lines.next(), lines.next()) {
        match a.chars().find(|chr| b.contains(*chr) && c.contains(*chr)) {
            Some(chr) => points_second_challenge += get_chr_value(chr),
            _ => {}
        }
    }

    println!("------");
    println!("Day 3 challenge 1: {:?}", points_first_challenge);
    println!("Day 3 challenge 2: {:?}", points_second_challenge);
}

fn get_chr_value(chr: char) -> u32 {
    if chr.is_lowercase() {
        chr as u32 - 96
    } else {
        chr as u32 - 64 + 26
    }
}

