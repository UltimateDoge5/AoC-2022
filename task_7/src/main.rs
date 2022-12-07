use std::{collections::HashMap, fs, path::PathBuf};

fn main() {
    let file = fs::read_to_string("input.txt").expect("Unable to read file");
    let mut files = HashMap::new();
    let mut path = Vec::new();

    for line in file.lines() {
        if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        }

        let args: Vec<&str> = line.split(" ").collect();

        match args[..] {
            ["$", "cd", ".."] => {
                path.pop();
            }
            ["$", "cd", name] => {
                path.push(name);
            }
            [size, _] => {
                let size: u32 = size.parse().unwrap();
                for i in 0..path.len() {
                    let temp_path = PathBuf::from_iter(&path[..=i]);
                    *files.entry(temp_path).or_insert(0) += size;
                }
            }
            _ => {}
        };
    }

    // Part 1
    // println!(
    //     "Sum of all the files: {}",
    //     &files
    //         .into_values()
    //         .filter(|size| *size <= 100_000)
    //         .sum::<u32>()
    // );

    // Part 2
    let disk_size = 70_000_000;
    let update_space = 30_000_000;
    let used = files.get(&PathBuf::from("/")).unwrap();
    let free = disk_size - *used;

    println!(
        "Smallest folder to delete: {}",
        files
            .into_values()
            .filter(|size| free + size >= update_space)
            .min()
            .unwrap()
    );
}
