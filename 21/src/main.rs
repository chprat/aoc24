// Advent of Code 21.12.2024
// - you need to enter a pin on a numerical keypad that looks like
//   [789|456|123| 0A] (top to bottom, left to right, the first spot in the
//   last row can't be accessed)
// - you can't directly interact with the keypad, but control robots via
//   directional keypads [ ^A|<v>] (top to bottom, left to right, the first spot
//   in the first row can't be accessed)
// - A on the directional keypads let the robot push the currently facing button
// - A on the numerical keypad ends the PIN input
// - generate the sequence of keys you need to press to enter a PIN on the
//   numerical keypad
// - as you're in a hurry, find one of the shortest sequences
// - the complexity of an input sequence is calculated by multiplying the
//   length of the input sequence and the numerical part of the PIN you entered
// - part 1:
//   - there are 4 keypads involved:
//     - one directional you control
//     - two directionals controlled by robots
//     - one numerical that is controlled by a robot
//   - calculate the sum of all PIN complexities from your input
// - part 2:
//   - same as part 1, but now there are 25 robot controlled directional keypads
//     in between you and the robot operating on the numerical keypad

use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};
use std::iter;
use std::path::Path;

fn main() {
    println!("The complexity of part 1 is {}", solver("input", false));
    println!("The complexity of part 2 is {}", solver("input", true));
}

// solver for part 1 and part 2
fn solver(filename: &str, part2: bool) -> usize {
    let max_depth = if part2 { 25 } else { 2 };
    let n_pad = Keypad::numeric();
    let d_pad = Keypad::directional();
    let mut cache = HashMap::new();
    let input = read_data(filename);
    input
        .iter()
        .map(|pin| {
            find_shortest_len(&n_pad, &d_pad, pin.to_string(), 0, max_depth, &mut cache)
                * pin[0..pin.len() - 1].parse::<usize>().unwrap()
        })
        .sum::<usize>()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Point(usize, usize);

struct Keypad {
    keys: HashMap<char, Point>,
    forbidden: Point,
}

impl Keypad {
    fn numeric() -> Self {
        let keys = HashMap::from([
            ('7', Point(0, 0)),
            ('8', Point(0, 1)),
            ('9', Point(0, 2)),
            ('4', Point(1, 0)),
            ('5', Point(1, 1)),
            ('6', Point(1, 2)),
            ('1', Point(2, 0)),
            ('2', Point(2, 1)),
            ('3', Point(2, 2)),
            ('0', Point(3, 1)),
            ('A', Point(3, 2)),
        ]);
        let forbidden = Point(3, 0);
        Keypad { keys, forbidden }
    }

    fn directional() -> Self {
        let keys = HashMap::from([
            ('^', Point(0, 1)),
            ('A', Point(0, 2)),
            ('<', Point(1, 0)),
            ('v', Point(1, 1)),
            ('>', Point(1, 2)),
        ]);
        let forbidden = Point(0, 0);
        Keypad { keys, forbidden }
    }

    // get the available paths from one PIN key to another
    fn get_paths(&self, first: char, second: char) -> Vec<String> {
        let mut q = VecDeque::from([(self.keys[&first], String::new())]);
        let mut res = Vec::new();
        while let Some((Point(x, y), mut path)) = q.pop_front() {
            if Point(x, y) == self.keys[&second] {
                path.push('A');
                res.push(path);
                continue;
            }
            // left
            if self.keys[&second].1 < y
                && !(self.forbidden.0 == x
                    && self.forbidden.1 < y
                    && self.forbidden.1 >= self.keys[&second].1)
            {
                let mut new_path = path.clone();
                new_path.extend(iter::repeat('<').take(y - self.keys[&second].1));
                q.push_back((Point(x, self.keys[&second].1), new_path));
            }
            // up
            if self.keys[&second].0 < x
                && !(self.forbidden.1 == y
                    && self.forbidden.0 < x
                    && self.forbidden.0 >= self.keys[&second].0)
            {
                let mut new_path = path.clone();
                new_path.extend(iter::repeat('^').take(x - self.keys[&second].0));
                q.push_back((Point(self.keys[&second].0, y), new_path));
            }
            // right
            if self.keys[&second].1 > y
                && !(self.forbidden.0 == x
                    && self.forbidden.1 > y
                    && self.forbidden.1 <= self.keys[&second].1)
            {
                let mut new_path = path.clone();
                new_path.extend(iter::repeat('>').take(self.keys[&second].1 - y));
                q.push_back((Point(x, self.keys[&second].1), new_path));
            }
            // down
            if self.keys[&second].0 > x
                && !(self.forbidden.1 == y
                    && self.forbidden.0 > x
                    && self.forbidden.0 <= self.keys[&second].0)
            {
                let mut new_path = path.clone();
                new_path.extend(iter::repeat('v').take(self.keys[&second].0 - x));
                q.push_back((Point(self.keys[&second].0, y), new_path));
            }
        }
        res
    }
}

// find the shortest length for a PIN key
fn find_shortest_len(
    n_pad: &Keypad,
    d_pad: &Keypad,
    pin: String,
    depth: usize,
    max_depth: usize,
    cache: &mut HashMap<(usize, String), usize>,
) -> usize {
    if let Some(&cached) = cache.get(&(depth, pin.clone())) {
        return cached;
    }

    let pad = if depth == 0 { n_pad } else { d_pad };
    let len = iter::once('A')
        .chain(pin.chars())
        .tuple_windows()
        .map(|(a, b)| {
            let paths = pad.get_paths(a, b);
            if depth == max_depth {
                paths.iter().map(String::len).min().unwrap()
            } else {
                paths
                    .into_iter()
                    .map(|path| find_shortest_len(n_pad, d_pad, path, depth + 1, max_depth, cache))
                    .min()
                    .unwrap()
            }
        })
        .sum::<usize>();

    cache.insert((depth, pin), len);
    len
}

// read the PIN information
fn read_data(filename: &str) -> Vec<String> {
    let mut pins = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            pins.push(line);
        }
    }
    pins
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
        assert_eq!(126384, solver("input.test", false));
    }
    #[test]
    fn part_1() {
        assert_eq!(94426, solver("input", false));
    }
    #[test]
    fn part_2() {
        assert_eq!(118392478819140, solver("input", true));
    }
}
