use std::fs;

pub fn run() {
    let mut entries: Vec<i32> = vec![];
    let mut current = 0;

    fs::read_to_string("input/day1.txt").unwrap()
        .lines()
        .for_each(|row| if row.is_empty() {
            entries.push(current);
            current = 0;
        } else {
            current += row.parse::<i32>().unwrap();
        });

    if current > 0 {
        entries.push(current);
    }

    entries.sort_by(|a, b| b.cmp(a));

    println!("------");
    println!("Day 1 challenge 1: {:?}", entries[0]);
    println!("Day 1 challenge 2: {:?}", &entries[0..3].iter().sum::<i32>());
}
