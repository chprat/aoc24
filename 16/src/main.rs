// Advent of Code 16.12.2024
// - read a 2d reindeer maze map that contains
//   - a start position (S)
//   - a end position (E)
//   - walls (#)
//   - empty spots (.)
// - reindeer start at the start position facing east
// - they need to find the end position
// - reindeer can move forward (increasing the score by 1)
// - reindeer can turn 90° (counter-)clockwise (increasing the score by 1000)
// - part 1:
//   - calculate the minimum score to complete the maze
// - part 2:
//   - count the number of unique positions on all of the best routes

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type PositionMap = HashMap<Type, Vec<Position>>;
type WayMap = HashMap<Position, Vec<Position>>;

fn main() {
    let r1 = solver("input", false);
    println!("The minimum score is {}", r1);
    assert_eq!(r1, 147628);

    let r2 = solver("input", true);
    println!("The seat count is {}", r2);
    assert_eq!(r2, 670);
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Left,
    Down,
    Up,
    Right,
    None,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Type {
    Empty,
    End,
    Start,
    Wall,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: usize,
    y: usize,
    typ: Type,
    direction: Direction,
    score: usize,
}
impl Position {
    fn new(x: usize, y: usize, typ: Type) -> Self {
        Self {
            x,
            y,
            typ,
            direction: Direction::None,
            score: 0,
        }
    }
    fn left(&self, positions: &PositionMap) -> Self {
        *positions
            .values()
            .flatten()
            .find(|p| p.x == self.x - 1 && p.y == self.y)
            .expect("Left position not found")
    }
    fn right(&self, positions: &PositionMap) -> Self {
        *positions
            .values()
            .flatten()
            .find(|p| p.x == self.x + 1 && p.y == self.y)
            .expect("Right position not found")
    }
    fn above(&self, positions: &PositionMap) -> Self {
        *positions
            .values()
            .flatten()
            .find(|p| p.x == self.x && p.y == self.y - 1)
            .expect("Above position not found")
    }
    fn below(&self, positions: &PositionMap) -> Self {
        *positions
            .values()
            .flatten()
            .find(|p| p.x == self.x && p.y == self.y + 1)
            .expect("Below position not found")
    }
}

// solver for the parts
fn solver(input: &str, part2: bool) -> usize {
    let positions = read_data(input, false);
    let start = positions.get(&Type::Start).expect("No start found")[0];
    let mut ways = find_ways(&positions);
    let mut routes = find_routes(&start, &mut ways);
    let score = walk_and_score_maze(&start, &mut routes);
    if part2 {
        let positions = read_data(input, true);
        let start = positions.get(&Type::Start).expect("No start found")[0];
        let mut ways = find_ways(&positions);
        let mut routes_reverse = find_routes(&start, &mut ways);
        walk_and_score_maze(&start, &mut routes_reverse);
        count_seats(&routes, &routes_reverse, score)
    } else {
        score
    }
}

// count the number of seats on the best routes
// uses the fact that on the best route the sum of the score of a point and the
// score of the point reversed (start and end switched) is the score of the route
fn count_seats(
    routes: &HashMap<Position, Position>,
    routes_reverse: &HashMap<Position, Position>,
    score: usize,
) -> usize {
    let mut ways: Vec<Position> = Vec::new();
    let mut used: Vec<(Position, Position)> = Vec::new();
    let starts: Vec<&Position> = routes_reverse
        .keys()
        .filter(|s| s.typ == Type::Start)
        .collect();
    ways.push(**starts.first().expect("No start found"));
    while let Some(way) = ways.pop() {
        let false_direction = match way.direction {
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            _ => Direction::None,
        };
        routes_reverse
            .iter()
            .filter(|(key, _value)| {
                key.x == way.x && key.y == way.y && key.direction != false_direction
            })
            .for_each(|(key, value)| {
                let old_v = routes
                    .iter()
                    .filter(|(_k, v)| v.x == value.x && v.y == value.y && v.score <= score)
                    .collect::<Vec<(&Position, &Position)>>();
                if value.typ == Type::End {
                    used.push((*key, *value));
                }
                for old in old_v {
                    // reverse score is not aligned that reindeer start facing east, therefore - 1000
                    if old.1.score + value.score == score
                        || old.1.score + value.score - 1000 == score
                    {
                        ways.push(*value);
                        used.push((*key, *value));
                    }
                }
            });
    }
    let mut coords: Vec<(usize, usize)> = Vec::new();
    used.iter().for_each(|(key, value)| {
        let x = (key.x as i64 - value.x as i64).abs();
        let y = (key.y as i64 - value.y as i64).abs();
        let d = std::cmp::max(x, y) as usize;
        if x == 0 {
            if key.y > value.y {
                for i in 0..=d {
                    coords.push((key.x, value.y + i));
                }
            } else {
                for i in 0..=d {
                    coords.push((key.x, key.y + i));
                }
            }
        } else if key.x > value.x {
            for i in 0..=d {
                coords.push((value.x + i, key.y));
            }
        } else {
            for i in 0..=d {
                coords.push((key.x + i, key.y));
            }
        }
    });
    coords.sort();
    coords.dedup();
    coords.len()
}

// score each way to the end and return the minimum score
fn score_step(first: &Position, second: &Position) -> usize {
    let mut sum = 0;
    if first.direction != second.direction {
        sum += 1000;
    }
    let x = (first.x as i64 - second.x as i64).abs();
    let y = (first.y as i64 - second.y as i64).abs();
    sum += std::cmp::max(x, y) as usize;
    sum
}

// walk the maze and score the positions
fn walk_and_score_maze(start: &Position, routes: &mut HashMap<Position, Position>) -> usize {
    let mut ways: Vec<Position> = Vec::new();
    let starts: Vec<&Position> = routes
        .keys()
        .filter(|s| s.x == start.x && s.y == start.y)
        .collect();
    for sp in starts {
        ways.push(*sp);
    }
    while let Some(way) = ways.pop() {
        let false_direction = match way.direction {
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            _ => Direction::None,
        };
        routes
            .iter_mut()
            .filter(|(key, _value)| {
                key.x == way.x && key.y == way.y && key.direction != false_direction
            })
            .for_each(|(_key, value)| {
                let mut score = way.score + score_step(&way, value);
                if way.typ == Type::Start {
                    match way.direction {
                        Direction::Up => score += 1000,
                        Direction::Down => score += 1000,
                        Direction::Left => score += 2000,
                        _ => (),
                    }
                }
                if value.score == 0 || score < value.score {
                    value.score = score;
                    if value.typ != Type::End {
                        ways.push(*value);
                    }
                }
            });
    }
    routes
        .values()
        .filter(|v| v.typ == Type::End)
        .map(|v| v.score)
        .min()
        .unwrap_or(0)
}

// find all valid routes from an intersection, when walking from the maze start
fn find_routes(start: &Position, ways: &mut WayMap) -> HashMap<Position, Position> {
    let mut points: Vec<Position> = vec![*ways
        .keys()
        .find(|s| s.x == start.x && s.y == start.y)
        .expect("No start found")];
    let mut routes = HashMap::new();
    while let Some(point) = points.pop() {
        let false_direction = match point.direction {
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            _ => Direction::None,
        };
        let all_points = ways
            .keys()
            .filter(|s| s.x == point.x && s.y == point.y && s.direction != false_direction)
            .copied()
            .collect::<Vec<Position>>();
        for all_point in all_points {
            if let Some(intersection) = ways.get(&all_point) {
                let last = intersection.last().expect("No last position found");
                routes.insert(all_point, *last);
                points.push(*last);
                ways.remove(&all_point);
            }
        }
    }
    routes.retain(|_, v| v.typ != Type::Start);
    let mut routes_len = routes.len();
    loop {
        routes = clean_routes(&routes);
        let new_len = routes.len();
        if new_len == routes_len {
            break;
        }
        routes_len = new_len;
    }
    routes
}

// remove dead ends from the routes
fn clean_routes(routes: &HashMap<Position, Position>) -> HashMap<Position, Position> {
    let mut clean_routes = HashMap::new();
    for (k, v) in routes {
        let starts = routes
            .keys()
            .find(|p| p.x == v.x && p.y == v.y)
            .into_iter()
            .collect::<Vec<&Position>>();
        if !starts.is_empty() || v.typ == Type::End {
            clean_routes.insert(*k, *v);
        }
    }
    clean_routes
}

// check if a position is an intersection
// corners are intersections, too
fn is_intersection(steps: &[Position]) -> bool {
    match steps.len() {
        l if l > 2 => true,
        2 => {
            let first = steps[0].direction;
            let second = steps[1].direction;
            match first {
                Direction::Right => second != Direction::Left,
                Direction::Left => second != Direction::Right,
                Direction::Up => second != Direction::Down,
                Direction::Down => second != Direction::Up,
                Direction::None => unreachable!(),
            }
        }
        _ => false,
    }
}

// walk from each intersection (and start) in each possible direction
// up to a wall or the next intersection
fn find_ways(positions: &PositionMap) -> WayMap {
    let mut splits: WayMap = HashMap::new();
    // find all intersections
    for empty in positions.get(&Type::Empty).expect("No empties found") {
        let steps = find_next_steps(empty, positions);
        if is_intersection(&steps) {
            for step in steps {
                let mut directional = *empty;
                directional.direction = step.direction;
                splits.insert(directional, vec![step]);
            }
        }
    }
    // special treatment for start
    let start = positions.get(&Type::Start).expect("No start found")[0];
    let steps = find_next_steps(&start, positions);
    for step in steps {
        let mut directional = start;
        directional.direction = step.direction;
        splits.insert(directional, vec![step]);
    }
    // walk from an intersection in each direction
    for way in splits.values_mut() {
        loop {
            let pos = way.last().expect("No last element found");
            let mut next_pos = match pos.direction {
                Direction::Up => pos.above(positions),
                Direction::Down => pos.below(positions),
                Direction::Left => pos.left(positions),
                Direction::Right => pos.right(positions),
                _ => unreachable!(),
            };
            next_pos.direction = pos.direction;
            if next_pos.typ == Type::Wall {
                break;
            }
            let next_steps = find_next_steps(&next_pos, positions);
            way.push(next_pos);
            if next_pos.typ == Type::Start
                || next_pos.typ == Type::End
                || is_intersection(&next_steps)
            {
                break;
            }
        }
    }
    splits
}

// find the next valid positions from a given position
fn find_next_steps(position: &Position, positions: &PositionMap) -> Vec<Position> {
    let mut next_steps = Vec::new();
    let mut left = position.left(positions);
    left.direction = Direction::Left;
    let mut right = position.right(positions);
    right.direction = Direction::Right;
    let mut above = position.above(positions);
    above.direction = Direction::Up;
    let mut below = position.below(positions);
    below.direction = Direction::Down;
    match position.direction {
        Direction::Up => {
            if above.typ != Type::Wall {
                next_steps.push(above);
            }
            if left.typ != Type::Wall {
                next_steps.push(left);
            }
            if right.typ != Type::Wall {
                next_steps.push(right);
            }
        }
        Direction::Left => {
            if left.typ != Type::Wall {
                next_steps.push(left);
            }
            if above.typ != Type::Wall {
                next_steps.push(above);
            }
            if below.typ != Type::Wall {
                next_steps.push(below);
            }
        }
        Direction::Right => {
            if right.typ != Type::Wall {
                next_steps.push(right);
            }
            if above.typ != Type::Wall {
                next_steps.push(above);
            }
            if below.typ != Type::Wall {
                next_steps.push(below);
            }
        }
        Direction::Down => {
            if below.typ != Type::Wall {
                next_steps.push(below);
            }
            if left.typ != Type::Wall {
                next_steps.push(left);
            }
            if right.typ != Type::Wall {
                next_steps.push(right);
            }
        }
        Direction::None => {
            if left.typ != Type::Wall {
                next_steps.push(left);
            }
            if right.typ != Type::Wall {
                next_steps.push(right);
            }
            if above.typ != Type::Wall {
                next_steps.push(above);
            }
            if below.typ != Type::Wall {
                next_steps.push(below);
            }
        }
    };
    next_steps
}

// read a reindeer maze map file
fn read_data(filename: &str, reverse: bool) -> PositionMap {
    let mut map: Vec<Vec<char>> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for y in lines.map_while(Result::ok) {
            map.push(y.chars().collect());
        }
    }
    parse_map(&map, reverse)
}

// print the map
#[allow(dead_code)]
fn print_map(filename: &str, coords: &[(usize, usize)]) {
    let mut map: Vec<Vec<char>> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for y in lines.map_while(Result::ok) {
            map.push(y.chars().collect());
        }
    }
    for coord in coords {
        map[coord.1][coord.0] = 'O';
    }
    for y in map {
        println!("{}", y.iter().collect::<String>())
    }
}
// parse the reindeer maze map
// reverse to switch start and end for part 2
fn parse_map(map: &[Vec<char>], reverse: bool) -> PositionMap {
    let mut positions = HashMap::new();
    (0..map.len()).for_each(|y| {
        (0..map[y].len()).for_each(|x| {
            let pos = match map[y][x] {
                '#' => Position::new(x, y, Type::Wall),
                'S' => {
                    if !reverse {
                        Position::new(x, y, Type::Start)
                    } else {
                        Position::new(x, y, Type::End)
                    }
                }
                'E' => {
                    if !reverse {
                        Position::new(x, y, Type::End)
                    } else {
                        Position::new(x, y, Type::Start)
                    }
                }
                _ => Position::new(x, y, Type::Empty),
            };
            positions
                .entry(pos.typ)
                .and_modify(|p: &mut Vec<Position>| p.push(pos))
                .or_insert(vec![pos]);
        })
    });
    assert_eq!(
        positions.get(&Type::Start).expect("No start found").len(),
        1
    );
    assert_eq!(positions.get(&Type::End).expect("No end found").len(), 1);
    assert!(positions.get(&Type::Empty).expect("No empties found").len() > 1);
    positions
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
    fn part1_1() {
        assert_eq!(solver("input1.test", false), 7036);
    }
    #[test]
    fn part1_2() {
        assert_eq!(solver("input2.test", false), 11048);
    }
    #[test]
    fn part2_1() {
        assert_eq!(solver("input1.test", true), 45);
    }
    #[test]
    fn part2_2() {
        assert_eq!(solver("input2.test", true), 64);
    }
}
