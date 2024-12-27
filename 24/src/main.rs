// Advent of Code 24.12.2024
// - read a device logic output file:
//   - the first part contains initial line values (signal: value)
//   - the second part contains wiring information (signal op signal -> output)
//   - gate can be AND, OR, XOR
//   - the parts are separated by an empty line
//   - the output lines (starting with z) form a binary number (z00 being the
//     LSB)
// - part 1:
//   - find the decimal number the device displays
// - part 2:
//   - the device is supposed to add the x and y lines, the result are the z
//     lines (x00 and y00 again being the LSB)
//   - four wire pairs have been made up, so the result doesn't add up
//   - find those eight faulty wires
//   - the result are those 8 signal names, sorted alphabetically, separated by
//     ',' without white spaces

type SignalMap = HashMap<String, u8>;
type FunctionMap = HashMap<String, (String, String, String)>;
use std::collections::HashMap;

fn main() {
    println!("The computer outputs {}", part1("input"));
    part2("input");
}

// solver for part 1
fn part1(filename: &str) -> i64 {
    let (mut signals, functions) = read_data(filename);
    while !is_output_defined(&signals) {
        for undefined in get_undefined(&signals, &functions) {
            let new_val = calc(&signals, &undefined, &functions);
            signals.insert(undefined, new_val);
        }
    }
    get_output(&signals)
}

// solver for part 2
fn part2(filename: &str) {
    let (mut signals, functions) = read_data(filename);
    while !is_output_defined(&signals) {
        for undefined in get_undefined(&signals, &functions) {
            let new_val = calc(&signals, &undefined, &functions);
            signals.insert(undefined, new_val);
        }
    }
    print_xyz(&signals);
}

// calculate the output of a gate
fn calc(signals: &SignalMap, key: &String, functions: &FunctionMap) -> u8 {
    let mut res: u8 = u8::MAX;
    let values = functions.get(key).expect("Can't find function");
    let cur_val = signals.get(key).expect("Can't find key");
    let s1 = signals.get(&values.0).expect("Can't find value");
    let s2 = signals.get(&values.2).expect("Can't find value");
    if *cur_val == u8::MAX && *s1 != u8::MAX && *s2 != u8::MAX {
        let s1b = *s1 != 0;
        let s2b = *s2 != 0;
        let s3b = match values.1.as_str() {
            "AND" => s1b && s2b,
            "OR" => s1b || s2b,
            "XOR" => s1b ^ s2b,
            _ => unreachable!(),
        };
        res = if s3b { 1 } else { 0u8 };
    }
    res
}

// check if all z outputs are defined
fn is_output_defined(signals: &SignalMap) -> bool {
    signals
        .iter()
        .filter(|(k, v)| k.starts_with('z') && **v == u8::MAX)
        .count()
        == 0
}

// get undefined signals, whose inputs are known
fn get_undefined(signals: &SignalMap, functions: &FunctionMap) -> Vec<String> {
    let mut defineable = Vec::new();
    let undefined = signals
        .iter()
        .filter(|(_k, v)| **v == u8::MAX)
        .map(|(k, _v)| k)
        .collect::<Vec<&String>>();
    for undef in undefined {
        let (s1, _gate, s2) = functions.get(undef).expect("Can't find key");
        let s1_val = *signals.get(s1).expect("Can't find key") != u8::MAX;
        let s2_val = *signals.get(s2).expect("Can't find key") != u8::MAX;
        if s1_val && s2_val {
            defineable.push(undef.clone());
        }
    }
    defineable
}

// return (x,y,z) signals as binary string
fn get_num(signals: &SignalMap, c: char) -> String {
    let mut outputs = signals
        .keys()
        .filter(|k| k.starts_with(c))
        .collect::<Vec<&String>>();
    outputs.sort();
    outputs.reverse();
    let mut bin_out: Vec<char> = Vec::new();
    for output in &outputs {
        let out = signals.get(*output).expect("Can't find key");
        assert_ne!(*out, u8::MAX);
        let new_char = if *out == 0 { '0' } else { '1' };
        bin_out.push(new_char);
    }
    bin_out.iter().collect::<String>()
}

// print x, y and z numbers
fn print_xyz(signals: &SignalMap) {
    let x_str = get_num(signals, 'x');
    let y_str = get_num(signals, 'y');
    let z_str = get_num(signals, 'z');
    let x = i64::from_str_radix(x_str.as_str(), 2).expect("Can't convert binary");
    let y = i64::from_str_radix(x_str.as_str(), 2).expect("Can't convert binary");
    let z = x + y;
    println!("x  {}", x_str);
    println!("y  {}", y_str);
    println!("z {} (generated)", z_str);
    println!("z {:b} (x+y)", z);
}

// convert binary z to i64
fn get_output(signals: &SignalMap) -> i64 {
    let out = get_num(signals, 'z');
    i64::from_str_radix(out.as_str(), 2).expect("Can't convert binary")
}

// read the device information
fn read_data(filename: &str) -> (SignalMap, FunctionMap) {
    let mut signals: SignalMap = HashMap::new();
    let mut functions: FunctionMap = HashMap::new();
    let file = std::fs::read_to_string(filename).expect("Can't read input");
    let parts = file.trim().split_once("\n\n").expect("Can't split input");
    for line in parts.0.split("\n") {
        if line.is_empty() {
            continue;
        }
        let lp = line.split_once(": ").expect("Can't split input values");
        signals.insert(
            lp.0.to_string(),
            lp.1.parse::<u8>().expect("Can't parse signal value"),
        );
    }
    for line in parts.1.split("\n") {
        if line.is_empty() {
            continue;
        }
        let lp = line
            .split_once(" -> ")
            .expect("Can't split assignment values");
        let function = lp.0.split(" ").collect::<Vec<&str>>();
        assert_eq!(function.len(), 3);
        let s1 = function[0].to_string();
        let s2 = function[2].to_string();
        let s3 = lp.1.to_string();
        let gate = function[1].to_string();
        signals.entry(s1.clone()).or_insert(u8::MAX);
        signals.entry(s2.clone()).or_insert(u8::MAX);
        signals.entry(s3.clone()).or_insert(u8::MAX);
        functions.insert(s3, (s1, gate, s2));
    }
    (signals, functions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_1_test() {
        assert_eq!(part1("input1.test"), 4);
    }

    #[test]
    fn part_1_2_test() {
        assert_eq!(part1("input2.test"), 2024);
    }

    #[test]
    fn part_1() {
        assert_eq!(part1("input"), 55114892239566);
    }
}
