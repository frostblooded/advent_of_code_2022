use std::fs;

fn get_duplicate(s1: &str, s2: &str) -> Option<char> {
    for c1 in s1.chars() {
        for c2 in s2.chars() {
            if c1 == c2 {
                return Some(c1);
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

    let total_score: u32 = file_contents.lines().fold(0, |acc, line: &str| {
        let line_split_position: usize = line.len() / 2;
        let (first_compartment, second_compartment): (&str, &str) =
            line.split_at(line_split_position);
        let duplicate: char = get_duplicate(first_compartment, second_compartment).unwrap();

        acc + get_char_score(&duplicate)
    });

    println!("Total score: {}", total_score);
}
