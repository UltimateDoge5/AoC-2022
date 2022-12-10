use std::{collections::HashSet, fs};

struct Rope {
    tail: [(i32, i32); 10],
    visited: HashSet<(i32, i32)>,
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("Unable to read the file!");

    let mut rope = Rope {
        tail: [(0, 0); 10],
        visited: HashSet::new(),
    };

    for line in file.lines() {
        let command = line.split_whitespace().collect::<Vec<&str>>();
        move_head(&mut rope, command[0], command[1].parse::<i32>().unwrap());
    }

    println!("Visited: {}", rope.visited.len() + 1);

    // Reset the rope for part 2
    rope = Rope {
        tail: [(0, 0); 10],
        visited: HashSet::new(),
    };

    for line in file.lines() {
        let command = line.split_whitespace().collect::<Vec<&str>>();
        move_rope(&mut rope, command[0], command[1].parse::<i32>().unwrap());
    }

    println!("Part 2 visited : {}", rope.visited.len());
}

fn move_head(rope: &mut Rope, direction: &str, steps: i32) {
    for _ in 0..steps {
        match direction {
            "U" => rope.tail[0].1 += 1,
            "D" => rope.tail[0].1 -= 1,
            "R" => rope.tail[0].0 += 1,
            "L" => rope.tail[0].0 -= 1,
            _ => panic!("Invalid direction!"),
        }

        let difference = (
            rope.tail[0].0 - rope.tail[1].0,
            rope.tail[0].1 - rope.tail[1].1,
        );

        if difference.0.abs() < 2 && difference.1.abs() < 2 {
            continue;
        }

        match direction {
            "U" => rope.tail[1].1 += 1,
            "D" => rope.tail[1].1 -= 1,
            "R" => rope.tail[1].0 += 1,
            "L" => rope.tail[1].0 -= 1,
            _ => panic!("Invalid direction!"),
        }

        if direction == "U" || direction == "D" {
            let diff = rope.tail[0].0 - rope.tail[1].0;
            rope.tail[1].0 += diff.signum();
        }

        if direction == "R" || direction == "L" {
            let diff = rope.tail[0].1 - rope.tail[1].1;
            rope.tail[1].1 += diff.signum();
        }

        rope.visited.insert(rope.tail[1]);
    }
}

fn move_rope(rope: &mut Rope, direction: &str, steps: i32) {
    for _ in 0..steps {
        match direction {
            "U" => rope.tail[0].1 += 1,
            "D" => rope.tail[0].1 -= 1,
            "R" => rope.tail[0].0 += 1,
            "L" => rope.tail[0].0 -= 1,
            _ => panic!("Invalid direction!"),
        }

        for i in 0..9 {
            let head = rope.tail[i];
            let knot = &mut rope.tail[i + 1];

            if (head.0 - knot.0).abs() < 2 && (head.1 - knot.1).abs() < 2 {
                continue;
            }

            match direction {
                "U" => knot.1 += 1,
                "D" => knot.1 -= 1,
                "R" => knot.0 += 1,
                "L" => knot.0 -= 1,
                _ => panic!("Invalid direction!"),
            }

            if direction == "U" || direction == "D" {
                let diff = head.0 - knot.0;
                knot.0 += diff.signum();
            }

            if direction == "R" || direction == "L" {
                let diff = head.1 - knot.1;
                knot.1 += diff.signum();
            }
        }

        rope.visited.insert(rope.tail[9]);
    }
}
