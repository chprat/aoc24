// Advent of Code 22.12.2024
// - you're selling hiding spots to monkeys in the market, so that you can
//   bananas later and trade them in for a stolen device
// - prices are based on a secret number and if you can predict it, you can
//   maximize profit
// - secret generation is based on the current secret:
//   - multiply the secret with 64, bitwise XOR the result with the secret and
//     calculate this result modulo 16777216
//   - divide the secret by 32 (round down to the next integer), bitwise XOR
//     the result with the secret and calculate this result modulo 16777216
//   - multiply the secret with 2048, bitwise XOR the result with the secret and
//     calculate this result modulo 16777216
//   - divide the secret by 32 (round down to the next integer), bitwise XOR
// - part 1:
//   - for each secret in your input, calculate the 2000th secret number and
//     add them up
// - part 2:
//   - for each secret in your input, calculate up to 2000 secret numbers
//   - prices are the last digit of the secret
//   - keep track of four price changes, your trading monkey will sell as soon
//     as he sees these four changes in a row (for each buyer)
//   - with which price change sequence can you maximize your profit?

use itertools::{iterate, Itertools};
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    part1();
    let sum = part2("input");
    println!("The summed up maximal profit is {:?}", sum);
}

// solver for part 1
fn part1() {
    let secrets = read_data("input");
    let res = secrets
        .iter()
        .map(|&s| iterate(s, calc).nth(2000).unwrap())
        .sum::<i64>();
    println!("The summed up secrets result is {:?}", res);
}

// solver for part 2
fn part2(filename: &str) -> i64 {
    let secrets = read_data(filename);
    secrets
        .par_iter()
        .map(|&s| {
            let prices: Vec<_> = iterate(s, calc).take(2001).map(|n| n % 10).collect();
            prices
                .into_iter()
                .rev()
                .tuple_windows()
                .map(|(a, b, c, d, e)| ((d - e, c - d, b - c, a - b), a))
                .collect::<HashMap<_, _>>()
        })
        .reduce(HashMap::new, |mut acc, m| {
            m.into_iter()
                .for_each(|(k, v)| *acc.entry(k).or_insert(0) += v);
            acc
        })
        .into_values()
        .max()
        .unwrap()
}

// calculate new secret
fn calc(secret: &i64) -> i64 {
    let mut secret = (secret ^ (secret << 6)) & 0xFFFFFF;
    secret = (secret ^ (secret >> 5)) & 0xFFFFFF;
    secret = (secret ^ (secret << 11)) & 0xFFFFFF;
    secret
}

// read the secret information
fn read_data(filename: &str) -> Vec<i64> {
    let mut secrets = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            secrets.push(line.parse::<i64>().expect("Couldn't convert number"));
        }
    }
    secrets
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
    fn steps() {
        assert_eq!(iterate(123, calc).nth(1).unwrap(), 15887950);
        assert_eq!(iterate(123, calc).nth(2).unwrap(), 16495136);
        assert_eq!(iterate(123, calc).nth(3).unwrap(), 527345);
        assert_eq!(iterate(123, calc).nth(4).unwrap(), 704524);
        assert_eq!(iterate(123, calc).nth(5).unwrap(), 1553684);
        assert_eq!(iterate(123, calc).nth(6).unwrap(), 12683156);
        assert_eq!(iterate(123, calc).nth(7).unwrap(), 11100544);
        assert_eq!(iterate(123, calc).nth(8).unwrap(), 12249484);
        assert_eq!(iterate(123, calc).nth(9).unwrap(), 7753432);
        assert_eq!(iterate(123, calc).nth(10).unwrap(), 5908254);
    }

    #[test]
    fn part_1_test() {
        let secrets = read_data("input.test");
        assert_eq!(
            secrets
                .iter()
                .map(|&s| iterate(s, calc).nth(2000).unwrap())
                .sum::<i64>(),
            37327623
        );
    }

    #[test]
    fn part_1() {
        let secrets = read_data("input");
        assert_eq!(
            secrets
                .iter()
                .map(|&s| iterate(s, calc).nth(2000).unwrap())
                .sum::<i64>(),
            13764677935
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part2("input2.test"), 23);
    }
}
