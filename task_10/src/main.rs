use std::fs;

struct CRT {
    row: String,
}

impl CRT {
    fn new() -> Self {
        CRT { row: String::new() }
    }

    fn print(&mut self, register: i32, cycle: i32) {
        let current_pixel = cycle % 40 - 1;
        let sprite = register - 1..=register + 1;

        if sprite.contains(&current_pixel) {
            self.row.push('#');
        } else {
            self.row.push('.');
        }

        if cycle % 40 == 0 {
            println!("{}", self.row);
            self.row.clear();
        }
    }
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("Unable to read the file!");

    let mut register = 1;
    let mut cycle = 1;
    let mut signal = 0;

    let signals = [20, 60, 100, 140, 180, 220];

    let mut crt = CRT::new();
    crt.print(register, cycle);

    for instruction in file.lines() {
        let mut instruction = instruction.split_whitespace();
        let command = instruction.next().unwrap();

        match command {
            "addx" => {
                let x = instruction.next().unwrap().parse::<i32>().unwrap();

                cycle += 1;

                if signals.contains(&cycle) {
                    let strength = cycle * register;
                    signal += strength;
                }

                crt.print(register, cycle);

                cycle += 1;
                register += x;

                if signals.contains(&cycle) {
                    let strength = cycle * register;
                    signal += strength;
                }

                crt.print(register, cycle);
            }
            "noop" => {
                cycle += 1;

                if signals.contains(&cycle) {
                    let strength = cycle * register;
                    signal += strength;
                }

                crt.print(register, cycle);
            }
            _ => panic!("Invalid command!"),
        }
    }

    println!("Register sum: {}", signal);
}
