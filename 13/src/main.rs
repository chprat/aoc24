// Advent of Code 13.12.2024
// - read a list of claw machine configurations, that contains
//   - the X and Y movements when pressing button A
//   - the X and Y movements when pressing button B
//   - the position of the price
// - pressing button A costs 3 token
// - pressing button B costs 1 token
// - no button is pressed more than 100x
// - part 1:
//   - calculate the fewest amount of tokens to win all possible prizes
// - part 2:
//   - due to a unit conversion error, we need to add 10000000000000
//     to the prizes X and Y position
//   - calculate the fewest amount of tokens to win all possible prizes

use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let mut machines = read_data("input");
    let price: i64 = machines.iter_mut().map(|m| get_price(m, false)).sum();
    assert_eq!(price, 28887);
    println!("{} coins need to be spent", price);

    let mut machines = read_data("input");
    let price: i64 = machines.iter_mut().map(|m| get_price(m, true)).sum();
    assert_eq!(price, 96979582619758);
    println!("{} coins need to be spent", price);
}

#[derive(Debug)]
struct Machine {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}
impl Machine {
    fn new(a: (i64, i64), b: (i64, i64), prize: (i64, i64)) -> Self {
        Self { a, b, prize }
    }
}

// get the price to reach the prize by solving
// a_x * a + b_x * b = p_x
// a_y * a + b_y * b = p_y
fn get_price(m: &mut Machine, part2: bool) -> i64 {
    if part2 {
        m.prize.0 += 10000000000000;
        m.prize.1 += 10000000000000;
    }
    let b = (m.prize.1 * m.a.0 - m.prize.0 * m.a.1) / (m.b.1 * m.a.0 - m.b.0 * m.a.1);
    let a = (m.prize.0 - b * m.b.0) / m.a.0;
    let x = m.a.0 * a + m.b.0 * b;
    let y = m.a.1 * a + m.b.1 * b;
    if (x, y) != m.prize {
        return 0;
    }
    a * 3 + b
}

// read claw machine configurations
fn read_data(filename: &str) -> Vec<Machine> {
    let mut machines = Vec::new();
    let data = read_to_string(filename).expect("File not found");
    for machine in data.split("\n\n").collect::<Vec<&str>>() {
        let lines = machine.split("\n").collect::<Vec<&str>>();
        let button_x = Regex::new(r"X\+\d+").unwrap();
        let button_y = Regex::new(r"Y\+\d+").unwrap();
        let prize_x = Regex::new(r"X\=\d+").unwrap();
        let prize_y = Regex::new(r"Y\=\d+").unwrap();
        let (mut ax, mut ay, mut bx, mut by, mut px, mut py) = (0, 0, 0, 0, 0, 0);
        for line in &lines {
            if line.contains("Button A") {
                ax = button_x.find(line).expect("Button A X not found").as_str()[2..]
                    .parse::<i64>()
                    .expect("Couldn't convert to number");
                ay = button_y.find(line).expect("Button A Y not found").as_str()[2..]
                    .parse::<i64>()
                    .expect("Couldn't convert to number");
            }
            if line.contains("Button B") {
                bx = button_x.find(line).expect("Button B X not found").as_str()[2..]
                    .parse::<i64>()
                    .expect("Couldn't convert to number");
                by = button_y.find(line).expect("Button B Y not found").as_str()[2..]
                    .parse::<i64>()
                    .expect("Couldn't convert to number");
            }
            if line.contains("Prize") {
                px = prize_x.find(line).expect("Button B X not found").as_str()[2..]
                    .parse::<i64>()
                    .expect("Couldn't convert to number");
                py = prize_y.find(line).expect("Button B Y not found").as_str()[2..]
                    .parse::<i64>()
                    .expect("Couldn't convert to number");
            }
        }
        machines.push(Machine::new((ax, ay), (bx, by), (px, py)));
    }
    machines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let mut machines = read_data("input.test");
        let price: i64 = machines.iter_mut().map(|m| get_price(m, false)).sum();
        assert_eq!(price, 480);

        let mut machines = read_data("input");
        let price: i64 = machines.iter_mut().map(|m| get_price(m, false)).sum();
        assert_eq!(price, 28887);
    }
    #[test]
    fn part2() {
        let mut machines = read_data("input.test");
        let price: i64 = machines.iter_mut().map(|m| get_price(m, true)).sum();
        assert_eq!(price, 875318608908);

        let mut machines = read_data("input");
        let price: i64 = machines.iter_mut().map(|m| get_price(m, true)).sum();
        assert_eq!(price, 96979582619758);
    }
}
