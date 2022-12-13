use std::{collections::VecDeque, fs};

fn main() {
    let file = fs::read_to_string("input.txt").expect("Unable to read file");
    let height_map: Vec<Vec<char>> = file.lines().map(|line| line.chars().collect()).collect();

    let mut start = (0, 0);

    for (y, row) in height_map.iter().enumerate() {
        for (x, point) in row.iter().enumerate() {
            if *point == 'S' {
                start = (x, y);
            }
        }
    }

    let path = bfs(&height_map, start);

    println!("Path length: {}", path.len());

    let mut start_positions: Vec<(usize, usize)> = Vec::new();

    for (y, row) in height_map.iter().enumerate() {
        for (x, point) in row.iter().enumerate() {
            if *point == 'a' || *point == 'S' {
                start_positions.push((x, y));
            }
        }
    }

    let mut shortest_path = std::usize::MAX;

    for start in start_positions {
        let path = bfs(&height_map, (start.0 as usize, start.1 as usize));
        if path.len() != 0 && path.len() < shortest_path {
            shortest_path = path.len();
        }
    }

    println!("Any shortest path length: {}", shortest_path);
}

fn check_direction(point_a: char, point_b: char) -> bool {
    if point_a == 'S' && (point_b == 'a' || point_b == 'b') {
        return true;
    }

    if point_a == 'z' && point_b == 'E' {
        return true;
    }

    if point_a != 'z' && point_b == 'E' {
        return false;
    }

    return point_b as i8 - point_a as i8 <= 1;
}

// Implement the breadth-first search algorithm and return the path
fn bfs(height_map: &Vec<Vec<char>>, start: (usize, usize)) -> Vec<(usize, usize)> {
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; height_map[0].len()]; height_map.len()];

    queue.push_back((start, vec![]));

    while let Some((current, path)) = queue.pop_front() {
        let (x, y) = current;

        if height_map[y][x] == 'E' {
            return path;
        }

        if visited[y][x] {
            continue;
        }

        visited[y][x] = true;

        if x > 0 && check_direction(height_map[y][x], height_map[y][x - 1]) {
            let mut new_path = path.clone();
            new_path.push((x - 1, y));
            queue.push_back(((x - 1, y), new_path));
        }

        if x < height_map[0].len() - 1 && check_direction(height_map[y][x], height_map[y][x + 1]) {
            let mut new_path = path.clone();
            new_path.push((x + 1, y));
            queue.push_back(((x + 1, y), new_path));
        }

        if y > 0 && check_direction(height_map[y][x], height_map[y - 1][x]) {
            let mut new_path = path.clone();
            new_path.push((x, y - 1));
            queue.push_back(((x, y - 1), new_path));
        }

        if y < height_map.len() - 1 && check_direction(height_map[y][x], height_map[y + 1][x]) {
            let mut new_path = path.clone();
            new_path.push((x, y + 1));
            queue.push_back(((x, y + 1), new_path));
        }
    }

    vec![]
}
