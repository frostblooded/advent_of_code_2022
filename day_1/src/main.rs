use std::fs;

fn remove_by_value(vec: &mut Vec<u32>, val: u32) {
    let index = vec.iter().position(|x| *x == val).unwrap();
    vec.remove(index);
}

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

    let max: u32 = find_max(&people_calories).unwrap();
    remove_by_value(&mut people_calories, max);

    let second_max: u32 = find_max(&people_calories).unwrap();
    remove_by_value(&mut people_calories, second_max);

    let third_max: u32 = find_max(&people_calories).unwrap();

    println!("Result: {:?}", max + second_max + third_max);
}
