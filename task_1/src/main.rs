use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").expect("Unable to read file");
    let lines = file.lines();

    let mut elve_index = 0;
    let mut elves: Vec<i32> = Vec::new();

    lines.for_each(|line| {
        if line == "" {
            elve_index += 1;
            return;
        }

        if elves.len() <= elve_index {
            elves.push(0);
        }

        let calories = line.parse::<i32>().unwrap();
        elves[elve_index] += calories;
    });

    // Part 1 answer
    println!("Max calories: {}", elves.iter().max().unwrap());

    // Part 2 answer
    elves.sort();
    let top_3 = elves.iter().rev().take(3);
    let mut total = 0;

    for elve in top_3 {
        total += elve;
    }

    println!("Total calories (top 3 elves): {}", total);
}
