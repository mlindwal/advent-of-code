use std::fs;
use std::ops::Range;

pub fn run() {
    let input_str = fs::read_to_string("input/day4.txt").unwrap();

    let mut points_first_challenge = 0;
    let mut points_second_challenge = 0;

    input_str.lines()
        .for_each(|row| {
            let mid = row.find(",").unwrap();

            let left = parse_range(&row[0..mid]);
            let right = parse_range(&row[mid + 1..]);

            if is_fully_overlapping(&left, &right) {
                points_first_challenge += 1
            }

            if left.start <= right.end && left.end >= right.start {
                points_second_challenge += 1;
            }
        });

    println!("------");
    println!("Day 4 challenge 1: {:?}", points_first_challenge);
    println!("Day 4 challenge 2: {:?}", points_second_challenge);
}

fn is_fully_overlapping(left: &Range<i32>, right: &Range<i32>) -> bool {
    (left.start <= right.start && left.end >= right.end) ||
    (right.start <= left.start && right.end >= left.end)
}

fn parse_range(val: &str) -> Range<i32> {
    let mid = val.find("-").unwrap();
    Range {
        start: val[0..mid].parse::<i32>().unwrap(),
        end: val[mid + 1..].parse::<i32>().unwrap()
    }
}
