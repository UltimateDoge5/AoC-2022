use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").expect("File not found");
    // For part 1 size is 4
    // For part 2 size is 14
    let slice_size = 14;

    let len = file.len() - slice_size;

    for i in 0..len {
        let slice = &file[i..i + slice_size];
        println!("{slice}");

        //check if any of the characters repeats more than once
        if slice.chars().all(|c| slice.matches(c).count() == 1) {
            println!("Took {} iterations to find the marker", i + slice_size);
            break;
        }
    }
}
