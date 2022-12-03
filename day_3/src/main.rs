use std::{fs, slice::Chunks};

use itertools::Itertools;

fn get_duplicate(s1: &str, s2: &str, s3: &str) -> Option<char> {
    for c1 in s1.chars() {
        for c2 in s2.chars() {
            for c3 in s3.chars() {
                if c1 == c2 && c1 == c3 {
                    return Some(c1);
                }
            }
        }
    }

    None
}

fn get_char_score(c: &char) -> u32 {
    match *c {
        'A'..='Z' => (*c as u32) - ('A' as u32) + 27,
        'a'..='z' => (*c as u32) - ('a' as u32) + 1,
        _ => unimplemented!(),
    }
}

fn main() {
    let file_contents: String = fs::read_to_string("input.txt").expect("Failed to read input");

    let total_score: u32 = file_contents
        .lines()
        .chunks(3)
        .into_iter()
        .fold(0, |acc, mut chunk| {
            let line1 = chunk.next().unwrap();
            let line2 = chunk.next().unwrap();
            let line3 = chunk.next().unwrap();

            let duplicate: char = get_duplicate(line1, line2, line3).unwrap();
            acc + get_char_score(&duplicate)
        });

    println!("Total score: {}", total_score);
}
