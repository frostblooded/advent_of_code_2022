use std::{fs, str::Split};

#[derive(PartialEq, Eq, Ord)]
enum Move {
    Rock,
    Paper,
    Scisscors,
}

impl Move {
    fn get_score(&self) -> u32 {
        match *self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scisscors => 3,
        }
    }
}

impl From<&str> for Move {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scisscors,
            _ => unimplemented!(),
        }
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match *self {
            Move::Rock => match *other {
                Move::Rock => Some(std::cmp::Ordering::Equal),
                Move::Paper => Some(std::cmp::Ordering::Less),
                Move::Scisscors => Some(std::cmp::Ordering::Greater),
            },
            Move::Paper => match *other {
                Move::Rock => Some(std::cmp::Ordering::Greater),
                Move::Paper => Some(std::cmp::Ordering::Equal),
                Move::Scisscors => Some(std::cmp::Ordering::Less),
            },
            Move::Scisscors => match *other {
                Move::Rock => Some(std::cmp::Ordering::Less),
                Move::Paper => Some(std::cmp::Ordering::Greater),
                Move::Scisscors => Some(std::cmp::Ordering::Equal),
            },
        }
    }
}

fn get_outcome_score(my_move: &Move, opponent_move: &Move) -> u32 {
    if my_move < opponent_move {
        0
    } else if my_move == opponent_move {
        3
    } else {
        6
    }
}

fn get_round_score(my_move: &Move, opponent_move: &Move) -> u32 {
    get_outcome_score(my_move, opponent_move) + my_move.get_score()
}

fn main() {
    let file_contents: String = fs::read_to_string("input.txt").expect("Failed to read input");

    let total_score: u32 = file_contents.lines().fold(0, |total_score_acc, line| {
        let mut split: Split<&str> = line.split(" ");
        let opponent_move: Move = split.next().unwrap().into();
        let my_move: Move = split.next().unwrap().into();
        let round_score: u32 = get_round_score(&my_move, &opponent_move);
        total_score_acc + round_score
    });

    println!("Total score: {}", total_score);
}
