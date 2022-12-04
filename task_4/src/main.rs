use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").expect("Unable to read file");
    let lines = file.lines();

    let mut full_overlaps = 0;
    let mut partial_overlaps = 0;

    for line in lines.clone() {
        let ranges = line.split(",").collect::<Vec<&str>>();

        let left = ranges[0]
            .split("-")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let right = ranges[1]
            .split("-")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        if left[0] <= right[0] && left[1] >= right[1] {
            full_overlaps += 1;
        } else if left[0] >= right[0] && left[1] <= right[1] {
            full_overlaps += 1;
        }

        //Partial overlap - part 2
        if left[0] <= right[0] && left[1] >= right[0] {
            partial_overlaps += 1;
        } else if left[0] <= right[1] && left[1] >= right[1] {
            partial_overlaps += 1;
        } else if left[0] >= right[0] && left[1] <= right[1] {
            partial_overlaps += 1;
        } else if left[0] <= right[0] && left[1] >= right[1] {
            partial_overlaps += 1;
        }
    }

    println!("Overlaping zones: {}", full_overlaps);
    println!("Partial overlaps: {}", partial_overlaps);
}
