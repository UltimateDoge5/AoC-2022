use std::{collections::HashSet, fs};

fn main() {
    let file = fs::read_to_string("input.txt").expect("Unable to read file");

    let mut cubes: HashSet<(i32, i32, i32)> = HashSet::new();

    for line in file.lines() {
        let mut coords = line.split(',').map(|x| x.parse::<i32>().unwrap());
        cubes.insert((
            coords.next().unwrap(),
            coords.next().unwrap(),
            coords.next().unwrap(),
        ));
    }

    let mut surface_area = 0;

    for cube in &cubes {
        let mut sides = 0;

        if cubes.contains(&(cube.0 + 1, cube.1, cube.2)) {
            sides += 1;
        }

        if cubes.contains(&(cube.0 - 1, cube.1, cube.2)) {
            sides += 1;
        }

        if cubes.contains(&(cube.0, cube.1 + 1, cube.2)) {
            sides += 1;
        }

        if cubes.contains(&(cube.0, cube.1 - 1, cube.2)) {
            sides += 1;
        }

        if cubes.contains(&(cube.0, cube.1, cube.2 + 1)) {
            sides += 1;
        }

        if cubes.contains(&(cube.0, cube.1, cube.2 - 1)) {
            sides += 1;
        }

        surface_area += 6 - sides;
    }

    println!("Surface area: {}", surface_area);
}
