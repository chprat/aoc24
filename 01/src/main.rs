// Advent of Code 01.12.2024
// - read a list of format "number   number"
// - part 1:
//   - sort the numbers of the rows
//   - calculate the similarity (difference) of each line
//   - sum up the similarity of the whole file
// - part 2:
//   - calculate the weighted similarity of the first row
//     - how often appears a number from the first row in the second?
//     - multiply that by the number
//   - sum up the weighted similarity for each number of the first row

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let rows = read_vectors("input");
    let similarity1 = part1(rows.0, rows.1);
    println!("The total similarity is {}", similarity1);

    let rows = read_vectors("input");
    let similarity2 = part2(rows.0, rows.1);
    println!("The total weighted similarity is {}", similarity2);
}

// solver for part 1
fn part1(mut row1: Vec<i32>, mut row2: Vec<i32>) -> i32 {
    row1.sort();
    row2.sort();
    let mut differences = 0;
    for i in 0..row1.len() {
        differences += get_difference(row1[i], row2[i]);
    }
    assert_eq!(differences, 2166959);
    differences
}

// get the difference of two numbers (bigger - smaller)
fn get_difference(n1: i32, n2: i32) -> i32 {
    if n1 > n2 {
        n1 - n2
    } else {
        n2 - n1
    }
}

// solver for part 2
fn part2(row1: Vec<i32>, row2: Vec<i32>) -> i32 {
    let mut similarity = 0;
    for number in row1 {
        let count_result = i32::try_from(row2.iter().filter(|&n| *n == number).count());
        let count = match count_result {
            Ok(c) => c,
            Err(e) => panic!("Can't convert {e:?}"),
        };
        similarity += count * number;
    }
    assert_eq!(similarity, 23741109);
    similarity
}

// read a file with lines of format "number   number"
// and return a vector for each row
fn read_vectors(filename: &str) -> (Vec<i32>, Vec<i32>) {
    let mut row1 = Vec::new();
    let mut row2 = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            let parts = line.split("   ");
            let collection = parts.collect::<Vec<&str>>();
            assert_eq!(collection.len(), 2);
            row1.push(collection[0].parse::<i32>().unwrap());
            row2.push(collection[1].parse::<i32>().unwrap());
        }
    }
    assert_eq!(row1.len(), row2.len());
    (row1, row2)
}

// read a file and get the lines
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
