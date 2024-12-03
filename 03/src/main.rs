// Advent of Code 04.12.2024
// - read a corrupted memory dump containing multiple lines with instructions
// - part 1:
//   - filter out the multiply operators and sum up those multiply results
//     - the format of a multiply operation is "mul(x,y)"
//     - x and y are 1-3 digit numbers
//     - the format must exactly match, e.g. no white space, no other brackets
// - part 2:
//   - the instruction set is extended by "do()" and "don't()"
//   - do enables all following mul instructions
//   - don't disables all following mul instructions
//   - mul instructions are enabled at program start
//   - only the most recent do/don't instruction applies

use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let data = read_data("input.test");
    let sum = filter_instructions(&data);
    assert_eq!(sum, 161);

    let data = read_data("input");
    let sum = filter_instructions(&data);
    println!("The sum of all mul instructions is {}", sum);

    let data = read_data("input2.test");
    let sum = part2(data);
    assert_eq!(sum, 48);

    let data = read_data("input");
    let sum = part2(data);
    println!("The sum of all conditional mul instructions is {}", sum);
}

// filter valid mul instructions from memory dump
fn filter_instructions(data: &str) -> i32 {
    let mut sum = 0;
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let multiplications: Vec<&str> = re.find_iter(data).map(|m| m.as_str()).collect();
    for multiplication in multiplications {
        sum += mul(multiplication);
    }
    sum
}

// execute mul instruction
fn mul(expr: &str) -> i32 {
    let re = Regex::new(r"\d{1,3},\d{1,3}").unwrap();
    let factors_re: Vec<&str> = re.find_iter(expr).map(|m| m.as_str()).collect();
    assert_eq!(factors_re.len(), 1);
    let factors = factors_re[0].split(",").collect::<Vec<&str>>();
    assert_eq!(factors.len(), 2);
    let factor1 = factors[0].parse::<i32>().unwrap();
    let factor2 = factors[1].parse::<i32>().unwrap();
    factor1 * factor2
}

// solver for part 2
fn part2(data: String) -> i32 {
    let mut sum = 0;
    let subs = data.split("don't()").collect::<Vec<&str>>();
    sum += filter_instructions(subs[0]);
    for sub in &subs[1..] {
        if let Some(x) = sub.find("do()") {
            sum += filter_instructions(&sub[x..]);
        }
    }
    sum
}

// read a file with lines and return as string
// the data is corrupted, we can't trust lines!
fn read_data(filename: &str) -> String {
    let mut array = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            array.push(line);
        }
    }
    array.join("")
}

// read a file and get the lines
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
