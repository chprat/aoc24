// Advent of Code 19.12.2024
// - read a list of color patterns
//   - the first part contains the available patterns (comma separated)
//   - the second part contains the desired designs (one pattern per line)
//   - the parts are separated by an empty line
// - part 1:
//   - find the amount of designs that can be created with the patterns
// - part 2:
//   - find the amount of different ways to create all designs

use pathfinding::prelude::count_paths;

fn main() {
    println!(
        "{} of the designs are constructable",
        solver("input", false)
    );
    println!(
        "{} different ways exist to construct the designs",
        solver("input", true)
    );
}

// solver for part 1 and 2
fn solver(filename: &str, part2: bool) -> usize {
    let (patterns, designs) = read_data(filename);
    let mut constructable = 0;
    let mut design_count = 0;
    for design in designs {
        let count = count_paths(
            Pos(0),
            |p| p.successors(&patterns, &design),
            |p| p.0 == design.len(),
        );
        if count > 0 {
            constructable += 1;
            design_count += count;
        }
    }
    if part2 {
        design_count
    } else {
        constructable
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Pos(usize);
impl Pos {
    fn successors(&self, patterns: &[String], design: &str) -> Vec<Pos> {
        let pattern_max_length = patterns
            .iter()
            .map(|p| p.len())
            .max()
            .expect("No longest pattern found");
        let mut result = Vec::new();
        for i in 1..pattern_max_length + 1 {
            let next = design.get(self.0..self.0 + i);
            if let Some(next) = next {
                for pattern in patterns.iter() {
                    if next == *pattern {
                        result.push(Pos(self.0 + i));
                    }
                }
            }
        }
        result
    }
}

// read the pattern/design information
fn read_data(filename: &str) -> (Vec<String>, Vec<String>) {
    let file = std::fs::read_to_string(filename).expect("Can't read input");
    let parts = file.trim().split("\n\n").collect::<Vec<&str>>();
    assert_eq!(parts.len(), 2);
    let mut patterns = Vec::new();
    for pattern in parts[0].split(", ") {
        patterns.push(pattern.to_string());
    }
    let mut designs = Vec::new();
    for design in parts[1].split("\n") {
        if design.is_empty() {
            continue;
        }
        designs.push(design.to_string());
    }
    (patterns, designs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        assert_eq!(6, solver("input.test", false));
    }
    #[test]
    fn part_2_test() {
        assert_eq!(16, solver("input.test", true));
    }
    #[test]
    fn part_1() {
        assert_eq!(327, solver("input", false));
    }
    #[test]
    fn part_2() {
        assert_eq!(772696486795255, solver("input", true));
    }
}
