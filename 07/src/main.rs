// Advent of Code 07.12.2024
// - read a math table with possible equations
//   - each row starts with the result followed by a colon and white space
//   - the rest of the line consists of numbers separated by a white space
//   - mathematical operators can be added between those numbers
//   - with the operators added, the equation might result in the result
//   - numbers are processed left to right and can't be changed
// - part 1:
//   - possible operators are add and multiply
//   - sum up the results that can be calculated with the numbers and operators
// - part 2:
//   - || is an additional operator, that concatenates adjacent numbers
//     (e.g. 12 || 34 becomes 1234)
//   - sum up the results that can be calculated with the numbers and operators

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let data = read_data("input.test");
    assert_eq!(solver(&data, false), 3749);

    let data = read_data("input");
    let sum = solver(&data, false);
    assert_eq!(sum, 2654749936343);
    println!("The sum of the valid expressions of part1 is {}", sum);

    let data = read_data("input.test");
    assert_eq!(solver(&data, true), 11387);

    let data = read_data("input");
    let sum = solver(&data, true);
    assert_eq!(sum, 124060392153684);
    println!("The sum of the valid expressions of part2 is {}", sum);
}

enum Operators {
    Add,
    Multiply,
    Concatenate,
}
impl Operators {
    fn calc(&self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Operators::Add => lhs + rhs,
            Operators::Multiply => lhs * rhs,
            Operators::Concatenate => lhs * 10i64.pow(rhs.ilog10() + 1) + rhs,
        }
    }
}

// solver for part 1 and 2
fn solver(data: &Vec<String>, part2: bool) -> i64 {
    let mut sum = 0;
    for line in data {
        let (result, numbers) = match line.split_once(':') {
            Some(x) => {
                let res = x.0.parse::<i64>().expect("Result couldn't be parsed");
                let num =
                    x.1.trim()
                        .split(" ")
                        .map(|x| x.parse::<i64>().expect("Number couldn't be parsed"))
                        .collect::<Vec<i64>>();
                (res, num)
            }
            None => panic!("No colon found in line"),
        };
        if do_math(result, &numbers[1..], numbers[0], part2) {
            sum += result;
        }
    }
    sum
}

// recursive calculation function
fn do_math(result: i64, numbers: &[i64], first: i64, part2: bool) -> bool {
    if numbers.is_empty() || first > result {
        return first == result;
    }
    (part2
        && do_math(
            result,
            &numbers[1..],
            Operators::Concatenate.calc(first, numbers[0]),
            part2,
        ))
        || do_math(
            result,
            &numbers[1..],
            Operators::Add.calc(first, numbers[0]),
            part2,
        )
        || do_math(
            result,
            &numbers[1..],
            Operators::Multiply.calc(first, numbers[0]),
            part2,
        )
}

// read a file with mathematical data and return as vector
fn read_data(filename: &str) -> Vec<String> {
    let mut map = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            map.push(line);
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
