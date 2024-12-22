// Advent of Code 18.12.2024
// - read a list of positions in a grid, that will be blocked
// - navigate through the grid from start (0,0) to end (70,70)
//   (the example is smaller, so (6,6) is the end there)
// - part 1:
//   - take 1024 (12 in the example) of the blockers and get the minimal amount
//     of steps necessary to get to the end
// - part 2:
//   - find the block (from all blockers) that prohibits reaching the end

use pathfinding::prelude::{bfs, Grid};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type Point = (usize, usize);

fn main() {
    let steps = part1("input", 1024, (0, 0), (70, 70));
    println!("It takes {} steps to get to the end", steps);
    let (x, y) = part2("input", 1024, (0, 0), (70, 70));
    println!("After obstacle {},{} the end isn't reachable", x, y);
}

// find path through memory from start to end
fn get_path(obstacles: Vec<Point>, limit: usize, start: Point, end: Point) -> Option<Vec<Point>> {
    let mut grid = obstacles.into_iter().take(limit).collect::<Grid>();
    grid.invert();
    bfs(&start, |p| grid.neighbours(*p), |p| *p == end)
}

// solver for part 1
fn part1(filename: &str, limit: usize, start: Point, end: Point) -> usize {
    let obstacles = read_data(filename);
    let path = get_path(obstacles, limit, start, end).expect("No valid path found");
    // path contains the start, so -1 to get the steps
    path.len() - 1
}

// solver for part 2
fn part2(filename: &str, limit: usize, start: Point, end: Point) -> Point {
    let obstacles = read_data(filename);
    let mut left = limit;
    let mut right = obstacles.len() - 1;
    loop {
        if left + 1 == right {
            break;
        }
        let mid = (left + right) / 2;
        if get_path(obstacles.clone(), mid, start, end).is_some() {
            left = mid;
        } else {
            right = mid;
        }
    }
    obstacles[left]
}

// read a list of obstacles
fn read_data(filename: &str) -> Vec<Point> {
    let mut corrupted = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            let splits = line.split(',').collect::<Vec<&str>>();
            assert_eq!(splits.len(), 2);
            let x = splits
                .first()
                .expect("Couldn't find first element")
                .parse::<usize>()
                .expect("Couldn't parse first element");
            let y = splits
                .last()
                .expect("Couldn't find last element")
                .parse::<usize>()
                .expect("Couldn't parse last element");
            corrupted.push((x, y));
        }
    }
    corrupted
}

// read a file and get the lines
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        assert_eq!(22, part1("input.test", 12, (0, 0), (6, 6)));
    }
    #[test]
    fn part_2_test() {
        assert_eq!((6, 1), part2("input.test", 12, (0, 0), (6, 6)));
    }
    #[test]
    fn part_1() {
        assert_eq!(264, part1("input", 1024, (0, 0), (70, 70)));
    }
    #[test]
    fn part_2() {
        assert_eq!((41, 26), part2("input", 1024, (0, 0), (70, 70)));
    }
}
