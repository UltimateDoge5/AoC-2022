use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").expect("Unable to read file");
    let lines = file.lines();
    let alphabet = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    let mut sum = 0;

    for line in lines.clone() {
        let len = line.len();

        let second_comp = &line[len / 2..len];

        for letter in line[0..len / 2].chars() {
            let duplicate = second_comp.char_indices().position(|(_, c)| c == letter);

            if duplicate.is_some() {
                let duplicate_pos = alphabet
                    .iter()
                    .position(|&c| c == letter.to_ascii_lowercase())
                    .unwrap();

                if letter.is_uppercase() {
                    sum += duplicate_pos + 27;
                } else {
                    sum += duplicate_pos + 1;
                }
                break;
            }
        }
    }

    println!("Part 1 sum: {}", sum);

    //Part 2
    let len = lines.clone().count();

    let mut i = 0;
    sum = 0;

    while i < len {
        let sack = [
            lines.clone().nth(i).unwrap(),
            lines.clone().nth(i + 1).unwrap(),
            lines.clone().nth(i + 2).unwrap(),
        ];

        let common_char = sack[0]
            .chars()
            .find(|c| sack[1].contains(*c) && sack[2].contains(*c))
            .unwrap();

        let duplicate_pos = alphabet
            .iter()
            .position(|&c| c == common_char.to_ascii_lowercase())
            .unwrap();

        if common_char.is_uppercase() {
            sum += duplicate_pos + 27;
        } else {
            sum += duplicate_pos + 1;
        }

        i += 3;
    }

    println!("Part 2 sum: {}", sum);
}
