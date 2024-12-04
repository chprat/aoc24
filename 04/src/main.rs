// Advent of Code 04.12.2024
// - read a word search puzzle containing multiple lines with characters
// - part 1:
//   - find the word "XMAS" in the puzzle
//     - the word can occur
//       - horizontal
//       - vertical
//       - diagonal
//       - reversed
//       - overlapping with other occurrences
//     - count all occurrences of the word
// - part 2:
//   - the puzzle is an X-MAS puzzle!
//     - find all occurrences of two diagonal "MAS" in a cross shape
//     - the word can be reversed
//   - count all occurrences of the crossed MAS

use diagonal::{diagonal_pos_neg, diagonal_pos_pos, straight_y};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let puzzle = read_data("input.test");
    let sum = part1(puzzle);
    assert_eq!(sum, 18);

    let puzzle = read_data("input");
    let sum = part1(puzzle);
    assert_eq!(sum, 2297);
    println!("The word XMAS occures {} times in the puzzle", sum);

    let data = read_data("input2.test");
    let sum = part2(&data);
    assert_eq!(sum, 9);

    let data = read_data("input");
    let sum = part2(&data);
    assert_eq!(sum, 1745);
    println!("The sum of all MAS crosses is {}", sum);
}

// solver for part 1
fn part1(puzzle: Vec<String>) -> i32 {
    let mut sum = 0;

    sum += count_word(&puzzle);

    let t_puzzle = transform(&puzzle);
    sum += count_word(&t_puzzle);

    let d1_puzzle = dia1(&puzzle);
    sum += count_word(&d1_puzzle);

    let d2_puzzle = dia2(&puzzle);
    sum += count_word(&d2_puzzle);
    sum
}

// count all occurrences of XMAS in line
fn count_word_in_line(line: &str) -> i32 {
    line.matches("XMAS").count().try_into().unwrap()
}

// reverse the line
fn reverse(line: &str) -> String {
    line.chars().rev().collect::<String>()
}

// count all occurrences of XMAS in the (reversed) line
fn count_word(line: &Vec<String>) -> i32 {
    let mut sum = 0;
    for line in line {
        sum += count_word_in_line(line);
        sum += count_word_in_line(&reverse(line));
    }
    sum
}

// convert a vector of string to vector of vector of char
fn to_char_vec(data: &Vec<String>) -> Vec<Vec<char>> {
    let mut mat: Vec<Vec<char>> = Vec::new();
    for line in data {
        mat.push(line.chars().collect());
    }
    mat
}

// convert a vector of vector of char to vector of string
fn to_string_vec(data: Vec<Vec<&char>>) -> Vec<String> {
    let mut mat = Vec::new();
    for line in data {
        mat.push(String::from_iter(line));
    }
    mat
}

// transform a matrix column -> row
fn transform(puzzle: &Vec<String>) -> Vec<String> {
    let mat = to_char_vec(puzzle);
    let result = straight_y(&mat);
    to_string_vec(result)
}

// transform matrix diagonals to row (left -> right)
fn dia1(puzzle: &Vec<String>) -> Vec<String> {
    let mat = to_char_vec(puzzle);
    let result = diagonal_pos_pos(&mat);
    to_string_vec(result)
}

// transform matrix diagonals to row (right -> left)
fn dia2(puzzle: &Vec<String>) -> Vec<String> {
    let mat = to_char_vec(puzzle);
    let result = diagonal_pos_neg(&mat);
    to_string_vec(result)
}

// solver for part 2
fn part2(puzzle: &Vec<String>) -> i32 {
    let mut sum = 0;
    let mat = to_char_vec(puzzle);

    // iterate over row (x)
    for i in (0..mat[0].len()).collect::<Vec<usize>>() {
        // skip edges
        if i == 0 || i == mat[0].len() - 1 {
            continue;
        }

        // iterate over column (y)
        for j in (0..mat.len()).collect::<Vec<usize>>() {
            // skip edges
            if j == 0 || j == mat.len() - 1 {
                continue;
            }

            // center is an 'A'
            if mat[i][j].to_ascii_lowercase() == 'a' {
                // create tuples with diagonals:
                // 1 2 3
                // 4 A 6
                // 7 8 9
                //   -> [(1, 9), (3, 7)]
                let roi = [
                    (mat[i - 1][j - 1], mat[i + 1][j + 1]),
                    (mat[i + 1][j - 1], mat[i - 1][j + 1]),
                ];
                if inspect_roi(&roi) {
                    sum += 1;
                }
            }
        }
    }
    sum
}

// check if both diagonals could form "MAS" or "SAM"
fn inspect_roi(roi: &[(char, char)]) -> bool {
    inspect_dia(&roi[0]) & inspect_dia(&roi[1])
}

// check if a diagonal could form "MAS" or "SAM"
fn inspect_dia(dia: &(char, char)) -> bool {
    let mut found = false;
    if dia.0.to_ascii_lowercase() == 'm' && dia.1.to_ascii_lowercase() == 's' {
        found = true;
    }
    if dia.1.to_ascii_lowercase() == 'm' && dia.0.to_ascii_lowercase() == 's' {
        found = true;
    }
    found
}

// read a file with lines of characters
// and return as vector containing each row
fn read_data(filename: &str) -> Vec<String> {
    let mut array = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            array.push(line);
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
