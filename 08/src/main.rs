// Advent of Code 08.12.2024
// - read a 2d map with antenna positions
//   - an antenna is marked by the frequency it's sending on
//   - valid frequencies are marked by [a-zA-Z0-9]
//   - two antennas of the same frequency create antinodes
//     - the antinodes occur on an imaginary line through the antennas
// - part 1:
//   - there's one antinode before/behind the antennas, at the same distance
//     as between the antennas
//   - count the number of antinodes
//     - each position is counted as an antinode only once (even if it's an
//       antinode of multiple antenna combinations)
//     - antinodes can occur on positions that have an antenna
// - part 2:
//   - because of harmonic frequencies, multiple antinodes are created by a
//     pair of antennas (at the same distance between the antennas)
//   - count the number of antinodes
//     - also the antenna positions count as antinodes

use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let map = read_data("input.test");
    assert_eq!(part1(&map), 14);

    let map = read_data("input");
    let sum_antinodes = part1(&map);
    assert_eq!(sum_antinodes, 228);
    println!("The antennas create {} antinodes", sum_antinodes);

    let map = read_data("input.test");
    assert_eq!(part2(&map), 34);

    let map = read_data("input");
    let sum_antinodes = part2(&map);
    assert_eq!(sum_antinodes, 766);
    println!("The antennas create {} harmonic antinodes", sum_antinodes);
}

#[derive(Debug, Clone)]
struct Point {
    x: usize,
    y: usize,
}
impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    fn distance(&self, other: &Self) -> (i64, i64) {
        let x = self.x as i64 - other.x as i64;
        let y = self.y as i64 - other.y as i64;
        (x, y)
    }
    fn with_delta(&self, x: i64, y: i64, max_x: usize, max_y: usize) -> Option<Self> {
        let new_x: i64 = self.x as i64 + x;
        let new_y: i64 = self.y as i64 + y;
        if new_x >= 0 && new_y >= 0 && new_x < max_x as i64 && new_y < max_y as i64 {
            Some(Point::new(new_x as usize, new_y as usize))
        } else {
            None
        }
    }
}
impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.x.cmp(&other.x).then(self.y.cmp(&other.y))
    }
}
impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Point {}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

// solver for part 1
fn part1(map: &[Vec<char>]) -> usize {
    let positions = read_map(map);
    let antinodes = find_antinodes(&positions, map[0].len(), map.len(), false);
    let mut all_antinodes = Vec::new();
    antinodes.values().for_each(|vals| {
        all_antinodes.append(&mut vals.clone());
    });
    all_antinodes.sort();
    all_antinodes.dedup();
    all_antinodes.len()
}

// solver for part 2
fn part2(map: &[Vec<char>]) -> usize {
    let positions = read_map(map);
    let antinodes = find_antinodes(&positions, map[0].len(), map.len(), true);
    let mut all_antinodes = Vec::new();
    antinodes.values().for_each(|vals| {
        all_antinodes.append(&mut vals.clone());
    });
    all_antinodes.sort();
    all_antinodes.dedup();
    all_antinodes.len()
}
// detect all antennas on the map
fn read_map(map: &[Vec<char>]) -> HashMap<char, Vec<Point>> {
    let mut positions: HashMap<char, Vec<Point>> = HashMap::new();
    (0..map.len()).for_each(|row| {
        (0..map[row].len()).for_each(|col| {
            // we can't use regex on a single char
            let antennas = vec![
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F',
                'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V',
                'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
            ];
            let key = &map[row][col];
            let pos = Point::new(col, row);
            if antennas.contains(key) {
                if positions.contains_key(key) {
                    positions.get_mut(key).unwrap().push(pos);
                } else {
                    positions.insert(*key, vec![pos]);
                }
            }
        })
    });
    positions
}

// find all antinodes for each antenna combination
fn find_antinodes(
    antennas: &HashMap<char, Vec<Point>>,
    x: usize,
    y: usize,
    part2: bool,
) -> HashMap<char, Vec<Point>> {
    let mut antinodes: HashMap<char, Vec<Point>> = HashMap::new();
    antennas.keys().for_each(|key| {
        let positions = antennas.get(key).unwrap();
        (0..positions.len()).for_each(|first| {
            (0..positions.len()).for_each(|second| {
                if first != second {
                    // as we're trying all point combinations, we only have to
                    // worry about one point, the other one will be handled when
                    // the points are reversed
                    let (dx, dy) = &positions[second].distance(&positions[first]);
                    // add the antenna positions as antinodes for part 2
                    if part2 {
                        if antinodes.contains_key(key) {
                            antinodes
                                .get_mut(key)
                                .unwrap()
                                .push(positions[first].clone());
                            antinodes
                                .get_mut(key)
                                .unwrap()
                                .push(positions[second].clone());
                        } else {
                            antinodes.insert(
                                *key,
                                vec![positions[first].clone(), positions[second].clone()],
                            );
                        }
                    }
                    let mut p = positions[second].clone();
                    while p.with_delta(*dx, *dy, x, y).is_some() {
                        p = p.with_delta(*dx, *dy, x, y).unwrap();
                        if antinodes.contains_key(key) {
                            antinodes.get_mut(key).unwrap().push(p.clone());
                        } else {
                            antinodes.insert(*key, vec![p.clone()]);
                        }
                        if !part2 {
                            break;
                        }
                    }
                }
            })
        })
    });
    antinodes
}

// read a map file
fn read_data(filename: &str) -> Vec<Vec<char>> {
    let mut map = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            map.push(line.chars().collect());
        }
    }
    map
}

// read a file and get the lines
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
