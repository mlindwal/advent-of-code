use std::fs;

use GameMove::{Paper, Rock, Scissors};
use WinLossDraw::{Draw, Loss, Win};

#[derive(Clone, Copy)]
enum GameMove {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Clone, Copy)]
enum WinLossDraw {
    Win = 6,
    Loss = 0,
    Draw = 3,
}

impl WinLossDraw {
    fn from_op(op_code: char) -> Self {
        match op_code {
            'X' => Loss,
            'Y' => Draw,
            _ => Win
        }
    }

    fn get_move_for_outcome(self: &Self, elf_move: &GameMove) -> GameMove {
        match (self, elf_move) {
            (Win, Paper) | (Draw, Scissors) | (Loss, Rock) => Scissors,
            (Win, Scissors) | (Draw, Rock) | (Loss, Paper) => Rock,
            _ => Paper
        }
    }
}

impl GameMove {
    fn from_op(op_code: char) -> Self {
        match op_code {
            'X' | 'A' => Rock,
            'Y' | 'B' => Paper,
            _ => Scissors
        }
    }

    fn play_game(self: &Self, elf_move: &Self) -> i32 {
        *self as i32 + match (self, elf_move) {
            (Scissors, Paper) | (Paper, Rock) | (Rock, Scissors) => 6,
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => 3,
            _ => 0
        }
    }
}

pub fn run() {
    let first_challenge = fs::read_to_string("input/day2.txt").unwrap()
        .lines()
        .map(get_move_and_op)
        .map(|(elf, op)| GameMove::from_op(op).play_game(&elf))
        .sum::<i32>();

    let second_challenge = fs::read_to_string("input/day2.txt").unwrap()
        .lines()
        .map(get_move_and_op)
        .map(|(elf, op)| WinLossDraw::from_op(op)
            .get_move_for_outcome(&elf)
            .play_game(&elf)
        )
        .sum::<i32>();

    println!("------");
    println!("Day 2 challenge 1: {:?}", first_challenge);
    println!("Day 2 challenge 2: {:?}", second_challenge);
}

fn get_move_and_op(row: &str) -> (GameMove, char) {
    let elf_op = row.chars().nth(0).unwrap();
    let elf = GameMove::from_op(elf_op);

    let op = row.chars().nth(2).unwrap();

    (elf, op)
}
