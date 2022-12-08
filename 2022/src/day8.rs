use std::collections::{HashMap};
use std::fs;
use std::cmp::max;

use itertools::Itertools;

pub fn run() {
    let input_str = fs::read_to_string("input/day8.txt").unwrap();

    println!("------");
    println!("Day 8 challenge 2: {:?}", challenge(create_path_map(input_str)));
}

fn challenge(path_map: HashMap<usize, Vec<i32>>) -> i32 {
    let width = path_map.get(&0).unwrap().len();
    let height = path_map.keys().len();

    let mut highest_score = 0;
    let mut visible_trees = 0;

    path_map.iter().sorted().for_each(|(row, trees)| {
        let row = *row;

        if row == 0 || row == height - 1 {
            visible_trees += width;
        } else {
            for (column, tree) in trees.iter().enumerate() {
                if column == 0 || column == width - 1 {
                    visible_trees += 1;
                } else {
                    let mut left_row = trees.get(0..column).unwrap().to_vec();
                    left_row.reverse();

                    let right_row = trees.get(column + 1..).unwrap().to_vec();

                    let mut above_column = (0..row)
                        .map(|y| *path_map.get(&y).unwrap().get(column).unwrap())
                        .collect_vec();

                    above_column.reverse();

                    let below_column = (row + 1..height)
                        .map(|y| *path_map.get(&y).unwrap().get(column).unwrap())
                        .collect_vec();

                    let all_rows = vec![left_row, right_row, above_column, below_column];

                    if any_free_path(*tree, &all_rows) {
                        visible_trees += 1;
                    }

                    highest_score = max(highest_score, second_challenge(*tree, &all_rows));
                }
            }
        }
    });
    println!("Day 8 challenge 1: {}", visible_trees);
    highest_score
}

fn any_free_path(height: i32, axis: &Vec<Vec<i32>>) -> bool {
    for v in axis {
        if v.iter().find(|v| *v >= &height).is_none() {
            return true;
        }
    }
    false
}

fn second_challenge(height: i32, axis: &Vec<Vec<i32>>) -> i32 {
    axis.iter().map(|v| v.iter().enumerate()
        .find(|(_, tree)| *tree >= &height)
        .map(|(idx, _)| idx + 1)
        .unwrap_or(v.len()) as i32)
        .reduce(|a, b| a * b)
        .unwrap()
}

fn create_path_map(input: String) -> HashMap<usize, Vec<i32>> {
    let mut map = HashMap::<usize, Vec<i32>>::new();

    input.lines().enumerate().for_each(|(idx, row)| {
        let v: Vec<i32> = row.chars().map(|chr| chr as i32 - 48).collect_vec();
        map.insert(idx, v);
    });

    map
}
