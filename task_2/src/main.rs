use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").expect("Unable to read file");
    let lines = file.lines();

    let mut score = 0;

    for line in lines {
        let strat: _ = line.split(" ").collect::<Vec<&str>>();
        score += calc_score(strat[0], strat[1]);
    }

    println!("Score part 1: {}", score);

    score = 0;

    let lines = file.lines();

    lines.for_each(|line| {
        let strat: _ = line.split(" ").collect::<Vec<&str>>();

        score += match strat[0] {
            // Rock
            "A" => match strat[1] {
                "X" => 3, // 0 + 3
                "Y" => 4, // 3 + 1
                "Z" => 8, // 6 + 2
                _ => panic!("Invalid input"),
            },
            // Paper
            "B" => match strat[1] {
                "X" => 1, // 0 + 1
                "Y" => 5, // 3 + 2
                "Z" => 9, // 6 + 3
                _ => panic!("Invalid input"),
            },
            // Scissors
            "C" => match strat[1] {
                "X" => 2, // 0 + 2
                "Y" => 6, // 3 + 3
                "Z" => 7, // 6 + 1
                _ => panic!("Invalid input"),
            },
            _ => panic!("Invalid input"),
        }
    });

    println!("Score part 2: {}", score);
}

// Part 1
fn calc_score(p1: &str, p2: &str) -> i32 {
    let mut score = 0;

    score += match p2 {
        "X" => {
            score += 1;
            match p1 {
                "A" => 3,
                "B" => 0,
                "C" => 6,
                _ => 0,
            }
        }
        "Y" => {
            score += 2;
            match p1 {
                "A" => 6,
                "B" => 3,
                "C" => 0,
                _ => 0,
            }
        }
        "Z" => {
            score += 3;
            match p1 {
                "A" => 0,
                "B" => 6,
                "C" => 3,
                _ => 0,
            }
        }
        _ => 0,
    };

    score
}
