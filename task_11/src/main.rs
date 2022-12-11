// For part 1 - true
// For part 2 - false
const RELIEF: bool = false;

struct Monkey {
    items: Vec<u128>,
    operation: fn(u128) -> u128,
    divisible_by: u128,
    on_true: i32,
    on_false: i32,
    inspections: u128,
}

impl Monkey {
    fn check_item(&mut self, divisor: u128) -> Delivery {
        let item = self.items[0];
        self.items.remove(0);

        let mut worry_level = (self.operation)(item);
        worry_level = if RELIEF {
            worry_level / 3
        } else {
            worry_level % divisor
        };

        self.inspections += 1;

        if worry_level % self.divisible_by == 0 {
            return Delivery {
                monkey: self.on_true,
                item: worry_level,
            };
        } else {
            return Delivery {
                monkey: self.on_false,
                item: worry_level,
            };
        }
    }
}

struct Delivery {
    monkey: i32,
    item: u128,
}

fn main() {
    // It was just easier for me to hardcode the monkeys insteas of reading from a file
    let mut monkeys: Vec<Monkey> = vec![
        Monkey {
            items: vec![96, 60, 68, 91, 83, 57, 85],
            operation: |old: u128| -> u128 { old * 2 },
            divisible_by: 17,
            on_true: 2,
            on_false: 5,
            inspections: 0,
        },
        Monkey {
            items: vec![75, 78, 68, 81, 73, 99],
            operation: |old: u128| -> u128 { old + 3 },
            divisible_by: 13,
            on_true: 7,
            on_false: 4,
            inspections: 0,
        },
        Monkey {
            items: vec![69, 86, 67, 55, 96, 69, 94, 85],
            operation: |old: u128| -> u128 { old + 6 },
            divisible_by: 19,
            on_true: 6,
            on_false: 5,
            inspections: 0,
        },
        Monkey {
            items: vec![88, 75, 74, 98, 80],
            operation: |old: u128| -> u128 { old + 5 },
            divisible_by: 7,
            on_true: 7,
            on_false: 1,
            inspections: 0,
        },
        Monkey {
            items: vec![82],
            operation: |old: u128| -> u128 { old + 8 },
            divisible_by: 11,
            on_true: 0,
            on_false: 2,
            inspections: 0,
        },
        Monkey {
            items: vec![72, 92, 92],
            operation: |old: u128| -> u128 { old * 5 },
            divisible_by: 3,
            on_true: 6,
            on_false: 3,
            inspections: 0,
        },
        Monkey {
            items: vec![74, 61],
            operation: |old: u128| -> u128 { old * old },
            divisible_by: 2,
            on_true: 3,
            on_false: 1,
            inspections: 0,
        },
        Monkey {
            items: vec![76, 86, 83, 55],
            operation: |old: u128| -> u128 { old + 4 },
            divisible_by: 5,
            on_true: 4,
            on_false: 0,
            inspections: 0,
        },
    ];

    let common_divisor = monkeys.iter().fold(1, |acc, x| acc * x.divisible_by);

    let iterations = match RELIEF {
        true => 20,
        false => 10000,
    };

    for _ in 0..iterations {
        for i in 0..8 {
            let monkey = &monkeys[i];

            for _ in 0..monkey.items.len() {
                let delivery = monkeys[i].check_item(common_divisor);

                let next_monkey = &mut monkeys[delivery.monkey as usize];
                next_monkey.items.push(delivery.item);
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspections.cmp(&a.inspections));
    let monkey_business: u128 = monkeys[0].inspections * monkeys[1].inspections;

    println!("Monkey business: {}", monkey_business)
}
