// Advent of Code 06.12.2024
// - read a map with a guards position and predict its way
//   - the guards position is marked with <, v, ^, >
//     (depending on the direction their looking at)
//   - obstacles on the map are marked with a #
//   - a guard walks forward until reaching an obstacle
//   - when reaching an obstacle, the guard turns 90° clock wise
// - part 1:
//   - count the distinct positions a guard visits
// - part 2:
//   - we need to lock the guard in a loop!
//     - we can only add one obstacle to the map
//     - it can't be the guards starting position
//   - count the positions, where adding an obstacle traps
//     the guard in a loop

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let map = read_data("input.test");
    let pos = find_guard(&map);
    assert_eq!(pos.x, 4);
    assert_eq!(pos.y, 6);
    assert_eq!(pos.direction, Direction::Up);
    assert_eq!(part1(&map), 41);

    let map = read_data("input");
    let steps = part1(&map);
    assert_eq!(steps, 5329);
    println!("The guard made {} steps", steps);

    let mut map = read_data("input.test");
    assert_eq!(part2(&mut map), 6);

    let mut map = read_data("input");
    let loops = part2(&mut map);
    assert_eq!(loops, 2162);
    println!("Found {} positions to trap the guard in a loop", loops);
}

#[derive(PartialEq, Debug, Default)]
enum Direction {
    #[default]
    Left,
    Down,
    Up,
    Right,
    End,
}

#[derive(Default, Debug)]
struct Position {
    x: usize,
    y: usize,
    direction: Direction,
}
impl Position {
    fn new() -> Self {
        Default::default()
    }
}

// solver for part 1
fn part1(map: &[Vec<char>]) -> usize {
    let mut pos = find_guard(map);
    let mut steps: Vec<(usize, usize)> = Vec::new();
    let mut i = 0;
    while pos.direction != Direction::End {
        let mut res = walk(map, &pos);
        steps.append(&mut res.0);
        pos = res.1;
        i += 1;
        // just to ensure we don't accidentally dead-lock
        if i > 10000 {
            break;
        }
    }
    // sort and remove duplicates, as we only want the
    // distinct positions the guard visits
    steps.sort();
    steps.dedup();
    steps.len()
}

// detect the starting position of the guard
fn find_guard(map: &[Vec<char>]) -> Position {
    let mut pos = Position::new();
    (0..map.len()).for_each(|row| {
        (0..map[row].len()).for_each(|col| match map[row][col] {
            '<' | 'v' | '^' | '>' => {
                pos.x = col;
                pos.y = row;
                pos.direction = match map[row][col] {
                    '<' => Direction::Left,
                    'v' => Direction::Down,
                    '^' => Direction::Up,
                    '>' => Direction::Right,
                    _ => panic!(),
                }
            }
            _ => (),
        })
    });
    pos
}

// predict the guards route to the next obstacle
// and return the steps taken and new position
fn walk(map: &[Vec<char>], pos: &Position) -> (Vec<(usize, usize)>, Position) {
    // row/col the guard is currently walking in
    let way = match pos.direction {
        Direction::Left | Direction::Right => map[pos.y].iter().collect::<Vec<_>>(),
        Direction::Down | Direction::Up => map.iter().map(|item| &item[pos.x]).collect::<Vec<_>>(),
        Direction::End => panic!(),
    };
    let mut new_pos = Position::new();
    match pos.direction {
        // left is reverse movement in way
        Direction::Left => {
            if let Some(e) = way
                .iter()
                .rev()
                .skip(map[0].len() - pos.x - 1)
                .position(|&&e| e == '#')
            {
                new_pos.x = pos.x - e + 1;
                new_pos.direction = Direction::Up;
            } else {
                new_pos.x = 0;
                new_pos.direction = Direction::End
            }
            new_pos.y = pos.y;
        }
        Direction::Down => {
            if let Some(e) = way.iter().skip(pos.y).position(|&&e| e == '#') {
                new_pos.y = pos.y + e - 1;
                new_pos.direction = Direction::Left;
            } else {
                new_pos.y = map.len() - 1;
                new_pos.direction = Direction::End
            }
            new_pos.x = pos.x;
        }
        // up is reverse movement in way
        Direction::Up => {
            if let Some(e) = way
                .iter()
                .rev()
                .skip(map.len() - pos.y - 1)
                .position(|&&e| e == '#')
            {
                new_pos.y = pos.y - e + 1;
                new_pos.direction = Direction::Right;
            } else {
                new_pos.y = 0;
                new_pos.direction = Direction::End
            }
            new_pos.x = pos.x;
        }
        Direction::Right => {
            if let Some(e) = way.iter().skip(pos.x).position(|&&e| e == '#') {
                new_pos.x = pos.x + e - 1;
                new_pos.direction = Direction::Down;
            } else {
                new_pos.x = map[0].len() - 1;
                new_pos.direction = Direction::End
            }
            new_pos.y = pos.y;
        }
        Direction::End => panic!(),
    }
    let steps = get_steps(pos, &new_pos);
    (steps, new_pos)
}

// get the steps are guard makes from one position to an other
fn get_steps(pos: &Position, new_pos: &Position) -> Vec<(usize, usize)> {
    let mut steps = Vec::new();
    // vertical movement
    if pos.x == new_pos.x && pos.y != new_pos.y {
        // up is reverse movement in way
        let up = pos.y > new_pos.y;
        let range = if !up {
            pos.y + 1..new_pos.y + 1
        } else {
            new_pos.y..pos.y
        };
        for iter in range {
            steps.push((pos.x, iter));
        }
        if up {
            steps.reverse()
        }
    }
    // horizontal movement
    if pos.x != new_pos.x && pos.y == new_pos.y {
        // left is reverse movement in way
        let left = pos.x > new_pos.x;
        let range = if !left {
            pos.x + 1..new_pos.x + 1
        } else {
            new_pos.x..pos.x
        };
        for iter in range {
            steps.push((iter, pos.y));
        }
        if left {
            steps.reverse()
        }
    }
    steps
}

// solver for part 2
// we just brute-force loop detection...
fn part2(map: &mut [Vec<char>]) -> usize {
    let mut loops = 0;
    (0..map.len()).for_each(|row| {
        (0..map[row].len()).for_each(|col| {
            let old_char = map[row][col];
            if old_char == '.' {
                map[row][col] = '#';
                let mut pos = find_guard(map);
                let mut i = 0;
                while pos.direction != Direction::End {
                    let res = walk(map, &pos);
                    pos = res.1;
                    i += 1;
                    // too many iterations are probably a loop
                    if i > 10000 {
                        loops += 1;
                        break;
                    }
                }
                map[row][col] = old_char;
            }
        })
    });
    loops
}

// read a file with map data and return as vector
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
