use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").expect("Unable to read the file");
    let mut trees: Vec<Vec<u32>> = Vec::new();
    let mut visible_trees = 0;

    for line in file.lines() {
        trees.push(
            line.chars()
                .map(|v| v.to_digit(10).unwrap())
                .collect::<Vec<u32>>(),
        );
    }

    let tree_len = trees.len();
    for (i, _) in trees.iter().enumerate() {
        let row_len = trees[i].len();

        for (j, _) in trees[i].iter().enumerate() {
            if j == 0 || trees[i][0..j].iter().max().unwrap() < &trees[i][j] {
                visible_trees += 1;
            } else if j == row_len - 1
                || trees[i][j + 1..row_len].iter().max().unwrap() < &trees[i][j]
            {
                visible_trees += 1;
            } else if i == 0 || trees[0..i].iter().map(|v| v[j]).max().unwrap() < trees[i][j] {
                visible_trees += 1;
            } else if i == tree_len - 1
                || trees[i + 1..tree_len].iter().map(|v| v[j]).max().unwrap() < trees[i][j]
            {
                visible_trees += 1;
            }
        }
    }

    println!("Visible trees: {}", visible_trees);
}
