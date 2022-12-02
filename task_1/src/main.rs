use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").expect("Unable to read file");
    let lines = file.lines();

    let mut elves: Vec<i32> = vec![0];

    lines.for_each(|line| {
        if line == "" {
            elves.push(0);
            return;
        }

        let calories = line.parse::<i32>().unwrap();
        *elves.last_mut().unwrap() += calories;
    });

    // Part 1 answer
    println!("Max calories: {}", elves.iter().max().unwrap());

    // Part 2 answer
    elves.sort();
    let top_3 = elves.iter().rev().take(3);

    println!("Total calories (top 3 elves): {}", top_3.sum::<i32>());
}
