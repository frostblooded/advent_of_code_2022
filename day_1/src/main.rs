use std::{fs, str::Lines};

fn find_max(vec: &Vec<u32>) -> Option<u32> {
    if vec.is_empty() {
        return None;
    }

    let mut current_max: u32 = vec[0];

    for el in vec {
        if *el > current_max {
            current_max = *el;
        }
    }

    Some(current_max)
}

fn main() {
    let file_contents: String = fs::read_to_string("input.txt").expect("Failed to read input");
    let mut people_calories: Vec<u32> = vec![0];

    for line in file_contents.lines() {
        if line.is_empty() {
            people_calories.push(0);
            continue;
        }

        let new_calories: u32 = line.parse().expect("Failed to parse line as number");
        let person_calories: &mut u32 = people_calories.last_mut().unwrap();
        *person_calories += new_calories;
    }

    println!("Result: {:?}", find_max(&people_calories));
}
