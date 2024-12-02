// Advent of Code 02.12.2024
// - read a 2d array of numbers, separated by spaces
// - part 1:
//   - check if each line (report) is considered "safe"
//     - numbers (levels) are either all increasing of all decreasing
//     - two adjacent numbers differ by at least one and at most three
// - part 2:
//   - a report is now also considered safe, when one unsafe level is removed

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let array = read_vectors("input.test");
    let safe_reports = part1(array);
    assert_eq!(safe_reports, 2);

    let array = read_vectors("input");
    let safe_reports = part1(array);
    println!("The list contains {} safe reports", safe_reports);

    let array = read_vectors("input.test");
    let safe_reports = part2(array);
    assert_eq!(safe_reports, 4);

    let array = read_vectors("input");
    let safe_reports = part2(array);
    println!("The list contains {} dampener safe reports", safe_reports);
}

// solver for part 1
fn part1(array: Vec<Vec<i32>>) -> i32 {
    let mut safe_reports = 0;
    for elem in array {
        if is_safe(&elem) {
            safe_reports += 1;
        }
    }
    safe_reports
}

// check if a report is safe
fn is_safe(report: &[i32]) -> bool {
    let incr = report[0] < report[1];
    let mut prev = report[0];
    let mut safe = false;
    for elem in &report[1..] {
        safe = is_sorted(prev, *elem, incr);
        if !safe {
            break;
        }
        safe = is_range(prev, *elem);
        if !safe {
            break;
        }
        prev = *elem;
    }
    safe
}

// check if a number is smaller/bigger than an other
// when incr == true, the number has to be bigger
// equal is not allowed
fn is_sorted(n1: i32, n2: i32, incr: bool) -> bool {
    if incr {
        n1 < n2
    } else {
        n1 > n2
    }
}

// check if 1 <= |n1 - n2| <= 3
fn is_range(n1: i32, n2: i32) -> bool {
    let mut safe = false;
    let diff = if n1 < n2 { n2 - n1 } else { n1 - n2 };
    if (1..=3).contains(&diff) {
        safe = true;
    }
    safe
}

// solver for part 2
fn part2(array: Vec<Vec<i32>>) -> i32 {
    let mut safe_reports = 0;
    for elem in array {
        if is_safe(&elem) {
            safe_reports += 1;
        } else {
            for index in 0..elem.len() {
                let mut arr_copy = elem.clone();
                arr_copy.remove(index);
                if is_safe(&arr_copy) {
                    safe_reports += 1;
                    break;
                }
            }
        }
    }
    safe_reports
}

// read a file with lines containing numbers separated by spaces
// and return a vector containing each row
fn read_vectors(filename: &str) -> Vec<Vec<i32>> {
    let mut array = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            let mut row = Vec::new();
            let parts = line.split(" ");
            let collection = parts.collect::<Vec<&str>>();
            assert!(collection.len() > 1);
            for elem in collection {
                row.push(elem.parse::<i32>().unwrap());
            }
            array.push(row);
        }
    }
    array
}

// read a file and get the lines
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
