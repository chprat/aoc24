// Advent of Code 10.12.2024
// - read a list of magic stones with a number on them
//   - the number changes every time you blink
//     1. if the number on a stone is 0, it becomes 1
//     2. if the number on a stone has an even amount of digits, the stone
//        splits in half (the first half has the first half of the digits,
//        the other half has the remaining digits)
//     3. otherwise the number is multiplied by 2024
//   - the rules are processed in the numbered order
// - part 1:
//   - count the number of stones after you blink 25 times
// - part 2:
//   - count the number of stones after you blink 75 times

use rayon::prelude::*;
use std::collections::HashMap;

fn main() {
    let input = vec![125, 17];
    assert_eq!(run(&input, 1), 3);
    assert_eq!(run(&input, 2), 4);
    assert_eq!(run(&input, 3), 5);
    assert_eq!(run(&input, 4), 9);
    assert_eq!(run(&input, 5), 13);
    assert_eq!(run(&input, 6), 22);

    let input = vec![890, 0, 1, 935698, 68001, 3441397, 7221, 27];
    let stones = run(&input, 25);
    assert_eq!(stones, 194782);
    println!("There are {} stones after blinking 25 times", stones);

    let stones = run(&input, 75);
    assert_eq!(stones, 233007586663131);
    println!("There are {} stones after blinking 75 times", stones);
}

// blink for a given number of times
fn run(input: &[usize], runs: u8) -> usize {
    input
        .par_iter()
        .map(|&v| {
            let mut cache = HashMap::new();
            blink(v, 0, runs, &mut cache)
        })
        .sum()
}

// recursive blinking
fn blink(stone: usize, blinks: u8, stop: u8, cache: &mut HashMap<(usize, u8), usize>) -> usize {
    if blinks >= stop {
        return 1;
    }

    if let Some(&cached_result) = cache.get(&(stone, blinks)) {
        return cached_result;
    }

    let result = match stone {
        0 => blink(1, blinks + 1, stop, cache),
        num if (&num.ilog10() + 1) % 2 == 0 => {
            let digits = num.ilog10() + 1;
            let power = 10_usize.pow(digits / 2);
            let (l, r) = (num / power, num % power);
            blink(l, blinks + 1, stop, cache) + blink(r, blinks + 1, stop, cache)
        }
        _ => blink(stone * 2024, blinks + 1, stop, cache),
    };

    cache.insert((stone, blinks), result);
    result
}
