use regex::Regex;
use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").expect("File not found");

    // Load the stack data into a 2D vector
    let mut crates: Vec<Vec<String>> = Vec::new();
    let regex = Regex::new(r"(?m)\[(.)\]| (\W{3})").unwrap();

    // Initialize the stack
    for _ in regex.captures_iter(file.lines().next().unwrap()) {
        crates.push(Vec::new());
    }

    for line in file.lines().rev().skip_while(|l| !l.is_empty()) {
        let matches = regex.captures_iter(line);

        for (i, m) in (matches).enumerate() {
            if m[0].trim().is_empty() {
                continue;
            }

            crates[i].push(m[1].to_string());
        }
    }

    // Move the crates
    let digit_regex = Regex::new(r"(m?)(\d+)").unwrap();

    for line in file.lines().skip_while(|l| !l.is_empty()).skip(1) {
        let digits: Vec<i32> = digit_regex
            .find_iter(line)
            .filter_map(|digits| digits.as_str().parse().ok())
            .collect();

        // For part 1
        // move_crate(
        //     &mut crates,
        //     digits[0],
        //     (digits[1] - 1).try_into().unwrap(),
        //     (digits[2] - 1).try_into().unwrap(),
        // );

        // For part 2
        move_stack(
            &mut crates,
            digits[0],
            (digits[1] - 1).try_into().unwrap(),
            (digits[2] - 1).try_into().unwrap(),
        );
    }

    // Print the crates on top of each stack in a single line
    for stack in crates {
        print!("{}", stack.last().unwrap_or(&String::from(" ")));
    }

    println!();
}

fn move_crate(crates: &mut Vec<Vec<String>>, count: i32, from: usize, to: usize) {
    for _ in 0..count {
        let stack_crate = crates[from].pop().unwrap();
        crates[to].push(stack_crate.to_string());
    }
}

// Move multpile crates from one stack to another,preserving the original order, instead of one at a time
fn move_stack(crates: &mut Vec<Vec<String>>, count: i32, from: usize, to: usize) {
    let len = crates[from].len();

    let mut stack = crates[from].split_off(len - count as usize);
    crates[to].append(&mut stack);
}
