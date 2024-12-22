// Advent of Code 20.12.2024
// - read a race track map with start and end positions, as well as walls and
//   the (empty) race track spots
// - navigate through the grid (you can only move left, right, up, down) from
//   start to end to get base time
// - you can cheat once during the run to go through walls on two consecutive
//   steps (in one direction)
// - part 1:
//   - count the amount of cheats that save you at least 100 steps
// - part 2:
//   - the task is the same as for part 1, but
//     - a cheat can now be enabled for up to 20 steps (not all steps have to be
//       used, unused steps can't be used)
//     - you can switch directions during cheating
//     - cheats with the same start and end position count as a single cheat,
//       although they might take a different route

use pathfinding::prelude::{dijkstra, Grid};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type Point = (usize, usize);
type PointMap = HashMap<Point, Vec<Point>>;

fn main() {
    let count = solver("input", false);
    println!("There are {} cheats that save at least 100 steps", count);
    assert_eq!(1369, count);

    let count = solver("input", true);
    println!("There are {} cheats that save at least 100 steps", count);
    assert_eq!(979012, count);
}

// solver for part 1 and 2
fn solver(filename: &str, part2: bool) -> usize {
    let (grid, start, end) = read_data(filename);
    let (path, orig_len) = walk_track(&grid, &start, &end).unwrap();
    let cheats = find_cheat_positions(&grid, &path, part2);
    let cheat_lengths = evaluate_cheats(&cheats, &path, &orig_len);
    let mut cheat_count = 0;
    for (length, count) in cheat_lengths {
        if length >= 100 {
            cheat_count += count;
        }
    }
    cheat_count
}

// walk the track from the start to end position
fn walk_track(grid: &Grid, start: &Point, end: &Point) -> Option<(Vec<Point>, usize)> {
    dijkstra(
        start,
        |p| {
            let mut successors = Vec::new();
            for neighbour in grid.neighbours(*p) {
                successors.push((neighbour, 1))
            }
            successors
        },
        |p| p == end,
    )
}

// find possible positions, where we can cheat
fn find_cheat_positions(grid: &Grid, path: &[Point], part2: bool) -> PointMap {
    let max_d = if part2 { 20 } else { 2 };
    let mut cheats: PointMap = HashMap::new();
    for (x, y) in path {
        let mut cheat_points: Vec<Point> = Vec::new();
        for vertex in grid {
            let d = grid.distance(vertex, (*x, *y));
            if d > 1 && d <= max_d && path.contains(&vertex) {
                cheat_points.push(vertex);
            }
        }
        cheats.insert((*x, *y), cheat_points);
    }
    cheats
}

// calculate the lengths of the cheat routes
fn evaluate_cheats(cheats: &PointMap, path: &[Point], orig_len: &usize) -> HashMap<usize, usize> {
    let mut lengths: HashMap<usize, usize> = HashMap::new();
    for (key, values) in cheats {
        let old_len = path
            .iter()
            .position(|(x, y)| *x == key.0 && *y == key.1)
            .expect("Position not found in path");
        for value in values {
            let mut new_len = path
                .iter()
                .position(|(x, y)| *x == value.0 && *y == value.1)
                .expect("Position not found in path");
            let d = ((key.0 as i64 - value.0 as i64).abs() + (key.1 as i64 - value.1 as i64).abs())
                as usize;
            new_len = path.len() - new_len - 1 + old_len + d;
            if new_len < *orig_len {
                lengths
                    .entry(*orig_len - new_len)
                    .and_modify(|p: &mut usize| *p += 1)
                    .or_insert(1);
            }
        }
    }
    lengths
}

// read the race track information
fn read_data(filename: &str) -> (Grid, Point, Point) {
    let mut track: Vec<Point> = Vec::new();
    let (mut start, mut end): (Point, Point) = ((0, 0), (0, 0));
    if let Ok(lines) = read_lines(filename) {
        for (y, line) in lines.map_while(Result::ok).enumerate() {
            for (x, c) in line.chars().enumerate() {
                // -1 to adjust for removing the border walls
                match c {
                    'S' => {
                        track.push((x - 1, y - 1));
                        start = (x - 1, y - 1);
                    }
                    'E' => {
                        track.push((x - 1, y - 1));
                        end = (x - 1, y - 1);
                    }
                    '.' => track.push((x - 1, y - 1)),
                    '#' => (),
                    _ => unreachable!(),
                }
            }
        }
    }
    let grid = track.into_iter().collect::<Grid>();
    (grid, start, end)
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
        let (grid, start, end) = read_data("input.test");
        let (path, orig_len) = walk_track(&grid, &start, &end).unwrap();
        let cheats = find_cheat_positions(&grid, &path, false);
        let cheat_lengths = evaluate_cheats(&cheats, &path, &orig_len);
        assert_eq!(14, *cheat_lengths.get(&2).expect("Entry not found"));
        assert_eq!(14, *cheat_lengths.get(&4).expect("Entry not found"));
        assert_eq!(2, *cheat_lengths.get(&6).expect("Entry not found"));
        assert_eq!(4, *cheat_lengths.get(&8).expect("Entry not found"));
        assert_eq!(2, *cheat_lengths.get(&10).expect("Entry not found"));
        assert_eq!(3, *cheat_lengths.get(&12).expect("Entry not found"));
        assert_eq!(1, *cheat_lengths.get(&20).expect("Entry not found"));
        assert_eq!(1, *cheat_lengths.get(&36).expect("Entry not found"));
        assert_eq!(1, *cheat_lengths.get(&38).expect("Entry not found"));
        assert_eq!(1, *cheat_lengths.get(&40).expect("Entry not found"));
        assert_eq!(1, *cheat_lengths.get(&64).expect("Entry not found"));
    }

    #[test]
    fn part_2_test() {
        let (grid, start, end) = read_data("input.test");
        let (path, orig_len) = walk_track(&grid, &start, &end).unwrap();
        let cheats = find_cheat_positions(&grid, &path, true);
        let cheat_lengths = evaluate_cheats(&cheats, &path, &orig_len);
        assert_eq!(32, *cheat_lengths.get(&50).expect("Entry not found"));
        assert_eq!(31, *cheat_lengths.get(&52).expect("Entry not found"));
        assert_eq!(29, *cheat_lengths.get(&54).expect("Entry not found"));
        assert_eq!(39, *cheat_lengths.get(&56).expect("Entry not found"));
        assert_eq!(25, *cheat_lengths.get(&58).expect("Entry not found"));
        assert_eq!(23, *cheat_lengths.get(&60).expect("Entry not found"));
        assert_eq!(20, *cheat_lengths.get(&62).expect("Entry not found"));
        assert_eq!(19, *cheat_lengths.get(&64).expect("Entry not found"));
        assert_eq!(12, *cheat_lengths.get(&66).expect("Entry not found"));
        assert_eq!(14, *cheat_lengths.get(&68).expect("Entry not found"));
        assert_eq!(12, *cheat_lengths.get(&70).expect("Entry not found"));
        assert_eq!(22, *cheat_lengths.get(&72).expect("Entry not found"));
        assert_eq!(4, *cheat_lengths.get(&74).expect("Entry not found"));
        assert_eq!(3, *cheat_lengths.get(&76).expect("Entry not found"));
    }
}
