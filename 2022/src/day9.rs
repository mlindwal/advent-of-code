use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::cmp::max;
use std::fmt::format;
use std::num;

use itertools::Itertools;

#[derive(Hash, Eq, PartialEq, Clone)]
struct RopeNode {
    x: i32,
    y: i32
}

pub fn run() {
    let input_str = fs::read_to_string("input/day9.txt").unwrap();

    println!("------");
    println!("Day 9 challenge 1: {:?}", run_challenge(&input_str, 2));
    println!("Day 9 challenge 2: {:?}", run_challenge(&input_str, 10));
}

fn run_challenge(input: &str, rope_length: usize) -> usize {
    let mut visited = HashSet::new();
    let mut positions: Vec<RopeNode> = vec![RopeNode{x: 0, y: 0}; rope_length];

    visited.insert(RopeNode{x: 0, y: 0});

    for row in input.lines() {
        let mut split = row.split_ascii_whitespace();
        let direction = split.next().unwrap();
        let steps = split.next().unwrap().parse::<i32>().unwrap();

        for _ in 0..steps {
            match direction {
                "R" => {positions[0].x += 1},
                "L" => {positions[0].x -= 1},
                "U" => {positions[0].y += 1},
                "D" => {positions[0].y -= 1},
                _ => panic!("Invalid command")
            };

            for idx in 1..positions.len() {
                let head = &positions[idx - 1];
                let tail = &positions[idx];

                let dx = head.x - tail.x;
                let dy = head.y - tail.y;

                if dx.abs() < 2 && dy.abs() < 2 {
                    break;
                }

                positions[idx].x += dx.checked_div(dx.abs()).unwrap_or(0);
                positions[idx].y += dy.checked_div(dy.abs()).unwrap_or(0);
            }
            visited.insert(positions.last().unwrap().clone());
        }
    }

    visited.len()
}
