// Advent of Code 09.12.2024
// - read a 2d topographic trail map with height information
//   - the height ranges from 0..9
//   - 0 marks a trail head, the start of a journey
//   - 9 is the top and is the end
//   - while walking, you can only climb a difference of 1 and only upwards
//   - only left, down, up and right walking is allowed (not diagonal)
// - part 1:
//   - the score of a trail head marks the amount of tops you can reach from it
//     (there might be multiple ways to this top, but it only counts once)
//   - calculate the trail head scores and sum them up
// - part 2:
//   - the rating of a trail head marks the amount of unique ways to a top it has
//   - calculate the trail head ratings and sum them up

use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let map = read_data("input.test");
    let trail_heads = get_trail_heads(&map);
    assert_eq!(trail_heads.len(), 9);
    let sum = assess_trail_heads(trail_heads, &map, false);
    assert_eq!(sum, 36);

    let map = read_data("input");
    let trail_heads = get_trail_heads(&map);
    let sum = assess_trail_heads(trail_heads, &map, false);
    assert_eq!(sum, 550);
    println!("The trail head score sum is {}", sum);

    let map = read_data("input.test");
    let trail_heads = get_trail_heads(&map);
    assert_eq!(trail_heads.len(), 9);
    let sum = assess_trail_heads(trail_heads, &map, true);
    assert_eq!(sum, 81);

    let map = read_data("input");
    let trail_heads = get_trail_heads(&map);
    let sum = assess_trail_heads(trail_heads, &map, true);
    assert_eq!(sum, 1255);
    println!("The trail head rating sum is {}", sum);
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Point {
    x: usize,
    y: usize,
    val: usize,
}
impl Point {
    fn new(x: usize, y: usize, val: usize) -> Self {
        Self { x, y, val }
    }
    // get point left of point
    fn left(&self, map: &[Vec<usize>]) -> Option<Self> {
        if self.x > 0 {
            let x = self.x - 1;
            let y = self.y;
            let v = map[x][y];
            Some(Point::new(x, y, v))
        } else {
            None
        }
    }
    // get point right of point
    fn right(&self, map: &[Vec<usize>]) -> Option<Self> {
        if self.x < map[0].len() - 1 {
            let x = self.x + 1;
            let y = self.y;
            let v = map[x][y];
            Some(Point::new(x, y, v))
        } else {
            None
        }
    }
    // get point above point
    fn above(&self, map: &[Vec<usize>]) -> Option<Self> {
        if self.y > 0 {
            let x = self.x;
            let y = self.y - 1;
            let v = map[x][y];
            Some(Point::new(x, y, v))
        } else {
            None
        }
    }
    // get point below point
    fn below(&self, map: &[Vec<usize>]) -> Option<Self> {
        if self.y < map.len() - 1 {
            let x = self.x;
            let y = self.y + 1;
            let v = map[x][y];
            Some(Point::new(x, y, v))
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
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

// walk from all the trail heads and score or rate them
fn assess_trail_heads(trail_heads: Vec<Point>, map: &[Vec<usize>], part2: bool) -> usize {
    let mut trails = HashMap::new();
    for point in trail_heads {
        let mut ways = walk_trail_head(point.clone(), map);
        if !part2 {
            ways.sort();
            ways.dedup();
        }
        trails.insert(point, ways);
    }
    trails.values().map(|way| way.len()).sum()
}

// walk from a trail head to the top (hopefully!)
fn walk_trail_head(point: Point, map: &[Vec<usize>]) -> Vec<Point> {
    let mut tops = Vec::new();
    let mut steps = vec![point];
    while let Some(step) = steps.pop() {
        if step.val != 9 {
            steps.append(&mut get_steps(&step, map));
        } else {
            tops.push(step);
        }
    }
    tops
}

// find the starting points for a trail
fn get_trail_heads(map: &[Vec<usize>]) -> Vec<Point> {
    let mut trail_heads = Vec::new();
    (0..map.len()).for_each(|x| {
        (0..map[x].len()).for_each(|y| {
            if map[x][y] == 0 {
                trail_heads.push(Point::new(x, y, 0));
            }
        });
    });
    trail_heads
}

// get the points we can walk to from a given point
fn get_steps(point: &Point, map: &[Vec<usize>]) -> Vec<Point> {
    let mut ways = Vec::new();
    let new_val = point.val + 1;
    if let Some(p) = point.left(map) {
        if p.val == new_val {
            ways.push(p);
        }
    }
    if let Some(p) = point.right(map) {
        if p.val == new_val {
            ways.push(p);
        }
    }
    if let Some(p) = point.above(map) {
        if p.val == new_val {
            ways.push(p);
        }
    }
    if let Some(p) = point.below(map) {
        if p.val == new_val {
            ways.push(p);
        }
    }
    ways
}

// read a topographical map file with height information
fn read_data(filename: &str) -> Vec<Vec<usize>> {
    let mut map = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for y in lines.map_while(Result::ok) {
            map.push(
                y.chars()
                    .map(|c| c.to_string().parse::<usize>().expect("Couldn't parse char"))
                    .collect(),
            );
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
