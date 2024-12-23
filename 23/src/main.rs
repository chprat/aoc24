// Advent of Code 23.12.2024
// - read a file with computer connections:
//   - computer name 1 - computer name 2
//   - one connection per line
//   - connections aren't directional (a-b == b-a)
// - part 1:
//   - find the number of groups of three interconnected computers that contain
//     at least one node starting with a 't'
// - part 2:
//   - find the password for the biggest group of interconnected computers
//     (the password consists of the sorted computer names, separated by a ',',
//     no white space in between)

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let nodes = read_data("input");
    let count = group_three(&nodes);
    println!("There are {} clusters", count);
    let passwd = passwd(&nodes);
    println!("There password for the biggest cluster is {}", passwd);
}

// count the number of clusters (three connected nodes), where at least one
// node starts with a 't'
fn group_three(nodes: &HashMap<String, Vec<String>>) -> usize {
    let mut seen = HashSet::new();
    let mut res = 0;
    for key in nodes.keys() {
        if !key.starts_with('t') || seen.contains(key) {
            continue;
        }
        let reachable = nodes.get(key).unwrap();
        for (i, a) in reachable.iter().enumerate() {
            if seen.contains(a) {
                continue;
            }
            for b in reachable[i + 1..].iter() {
                if nodes.get(b).unwrap().contains(a) && !seen.contains(b) {
                    seen.insert(key);
                    res += 1;
                }
            }
        }
    }
    res
}

// find the password for the biggest group of connected nodes
// the password is the sorted computer names, joined with a ','
fn passwd(nodes: &HashMap<String, Vec<String>>) -> String {
    let mut longest = Vec::new();
    for start in nodes.keys() {
        let mut group = vec![start];
        for neighbour in nodes.get(start).unwrap() {
            if group
                .iter()
                .all(|node| nodes.get(neighbour).unwrap().contains(node))
            {
                group.push(neighbour);
            }
        }
        if group.len() > longest.len() {
            longest = group
        }
    }
    longest.sort();
    longest
        .into_iter()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

// read the network information
fn read_data(filename: &str) -> HashMap<String, Vec<String>> {
    let mut nodes: HashMap<String, Vec<String>> = HashMap::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            let splits = line.split_once('-').expect("Couldn't split line");
            nodes
                .entry(splits.0.to_string())
                .or_default()
                .push(splits.1.to_string());
            nodes
                .entry(splits.1.to_string())
                .or_default()
                .push(splits.0.to_string());
        }
    }
    nodes
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
        let nodes = read_data("input.test");
        assert_eq!(group_three(&nodes), 7);
    }

    #[test]
    fn part_1() {
        let nodes = read_data("input");
        assert_eq!(group_three(&nodes), 1304);
    }

    #[test]
    fn part_2_test() {
        let nodes = read_data("input.test");
        assert_eq!(passwd(&nodes), "co,de,ka,ta");
    }

    #[test]
    fn part_2() {
        let nodes = read_data("input");
        assert_eq!(passwd(&nodes), "ao,es,fe,if,in,io,ky,qq,rd,rn,rv,vc,vl");
    }
}
