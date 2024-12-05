// Advent of Code 05.12.2024
// - read an instruction manual print order and the ordering rules
//   - the first part are printing rules of format "X|Y"
//     - X needs to be printed before Y
//   - the second part is the printing order of format "a,b,c,d,e..."
// - part 1:
//   - verify the correctness of the order with the rules from above
//   - for all correctly ordered printing orders, detect the middle page
//   - sum up the middle page numbers
// - part 2:
//   - reorder all incorrectly ordered printing jobs
//   - detect the middle page of the reordered printing jobs
//   - sum up the middle page numbers (from reordered printing orders)

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let (rules, order) = read_data("input.test");
    assert!(check_sorted(&rules, &order[0]));
    assert!(check_sorted(&rules, &order[1]));
    assert!(check_sorted(&rules, &order[2]));
    assert!(!check_sorted(&rules, &order[3]));
    assert!(!check_sorted(&rules, &order[4]));
    assert!(!check_sorted(&rules, &order[5]));
    assert_eq!(part1(rules, order), 143);

    let (rules, order) = read_data("input");
    let sum = part1(rules, order);
    println!(
        "The sum of all correctly ordered printing jobs center pages is {}",
        sum
    );

    let (rules, order) = read_data("input.test");
    let sum = part2(rules, order);
    assert_eq!(sum, 123);

    let (rules, order) = read_data("input");
    let sum = part2(rules, order);
    assert_eq!(sum, 6004);
    println!(
        "The sum of all reordered printing jobs center pages is {}",
        sum
    );
}

// solver for part 1
fn part1(rules: Vec<(i32, i32)>, order: Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for row in order {
        let sorted = check_sorted(&rules, &row);
        if sorted {
            let len: usize = (row.len() as f32 / 2.0).floor() as usize;
            sum += row[len];
        }
    }
    sum
}

// check if a row is sorted according to the rules
// could be done easier with:
//   order.filter(|row| row.is_sorted_by(|a,b| !rules.contains(&(*b,*a))))
fn check_sorted(rules: &[(i32, i32)], row: &[i32]) -> bool {
    let mut sorted = true;
    for (cur_pos, elem) in row.iter().enumerate() {
        let apply_rules = rules
            .iter()
            .filter(|&e| e.0 == *elem)
            .collect::<Vec<&(i32, i32)>>();
        for rule in apply_rules {
            let rule_pos = match row.iter().position(|&e| e == rule.1) {
                Some(x) => x,
                None => continue,
            };
            if rule_pos < cur_pos {
                sorted = false;
                break;
            }
        }
        if !sorted {
            break;
        }
    }
    sorted
}

// solver for part 2
fn part2(rules: Vec<(i32, i32)>, order: Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for mut row in order {
        let sorted = check_sorted(&rules, &row);
        if !sorted {
            row.sort_by(|a, b| rules.contains(&(*a, *b)).cmp(&true));
            let len: usize = (row.len() as f32 / 2.0).floor() as usize;
            sum += row[len];
        }
    }
    sum
}

// read a file with printing rules and orders
// and return as separate vectors
fn read_data(filename: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let mut rules = Vec::new();
    let mut order = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            if line.matches("|").count() == 1 {
                let parts = line.split("|").collect::<Vec<&str>>();
                assert_eq!(parts.len(), 2);
                let x = parts[0].parse::<i32>().unwrap();
                let y = parts[1].parse::<i32>().unwrap();
                rules.push((x, y));
            }
            if line.matches(",").count() > 0 {
                let parts = line.split(",").collect::<Vec<&str>>();
                assert!(parts.len() > 1);
                let mut row = Vec::new();
                for elem in parts {
                    row.push(elem.parse::<i32>().unwrap());
                }
                order.push(row);
            }
        }
    }
    (rules, order)
}

// read a file and get the lines
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
