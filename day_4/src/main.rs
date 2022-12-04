use std::fs;

struct Range {
    start: u32,
    end: u32,
}

impl Range {
    pub fn fully_contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }
}

impl From<&str> for Range {
    fn from(s: &str) -> Self {
        let mut split = s.split('-');
        let start: u32 = split.next().unwrap().parse().unwrap();
        let end: u32 = split.next().unwrap().parse().unwrap();
        Range { start, end }
    }
}

fn main() {
    let file_contents: String = fs::read_to_string("input.txt").expect("Failed to read input");

    let total_score: u32 = file_contents
        .lines()
        .filter(|line| {
            let (first_range_str, second_range_str) = line.split_once(',').unwrap();
            let first_range: Range = first_range_str.into();
            let second_range: Range = second_range_str.into();

            first_range.fully_contains(&second_range) || second_range.fully_contains(&first_range)
        })
        .count() as u32;

    println!("Total score: {}", total_score);
}
