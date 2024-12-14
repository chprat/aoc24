// Advent of Code 14.12.2024
// - read a list of robot positions an their moving speed
//   - p defines their current position
//   - v defines their velocity (movement in 1s)
//     - negative x movement means going left
//     - negative y movement means going up
// - the robots move in a 101x103 big area (it's 11x7 in the test)
// - 0,0 is the top left corner of the area
// - robots teleport to the other side when they reach the edge of the area
// - part 1:
//   - calculate the amount of robots in each quadrant (safety factor)
//     - robots on the horizontal or vertical center line are ignored
//     - count the amount of robots in each quadrant
//     - multiply the quadrant sums for the result
// - part 2:
//   - there's an Easter egg where the robots form a Christmas tree
//     - this somehow means all robots are on a distinct spot and there are no
//       overlaps
//     - count the amount of steps it takes until the Easter egg happens

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let map = (101, 103);
    let mut robots = read_data("input", map);
    for robot in &mut robots {
        robot.nstep(100);
    }
    let safety_factor = calc_safety(&robots, &map);
    println!("The safety factor is {}", safety_factor);

    let mut robots = read_data("input", map);
    let easter_egg_steps = calc_easter_egg(&mut robots);
    println!("The easter egg happens after {} steps", easter_egg_steps);
    print_positions(&robots, &map);
}

#[derive(Debug)]
struct Robot {
    pos: (i64, i64),
    vel: (i64, i64),
    map: (i64, i64),
}
impl Robot {
    fn new(pos: (i64, i64), vel: (i64, i64), map: (i64, i64)) -> Self {
        Self { pos, vel, map }
    }
    fn step(&mut self) {
        self.pos.0 += self.vel.0;
        if self.pos.0 < 0 {
            self.pos.0 += self.map.0;
        }
        if self.pos.0 > self.map.0 - 1 {
            self.pos.0 %= self.map.0;
        }
        self.pos.1 += self.vel.1;
        if self.pos.1 < 0 {
            self.pos.1 += self.map.1;
        }
        if self.pos.1 > self.map.1 - 1 {
            self.pos.1 %= self.map.1;
        }
    }
    fn nstep(&mut self, step: usize) {
        for _ in 0..step {
            self.step()
        }
    }
}

// calculate the safety factor
fn calc_safety(robots: &[Robot], map: &(i64, i64)) -> usize {
    let (mut q1, mut q2, mut q3, mut q4): (usize, usize, usize, usize) = (0, 0, 0, 0);
    for robot in robots {
        if robot.pos.0 < map.0 / 2 && robot.pos.1 < map.1 / 2 {
            q1 += 1;
        }
        if robot.pos.0 > map.0 / 2 && robot.pos.1 < map.1 / 2 {
            q2 += 1;
        }
        if robot.pos.0 < map.0 / 2 && robot.pos.1 > map.1 / 2 {
            q3 += 1;
        }
        if robot.pos.0 > map.0 / 2 && robot.pos.1 > map.1 / 2 {
            q4 += 1;
        }
    }
    q1 * q2 * q3 * q4
}

// calculate after how many steps the Easter egg happens
fn calc_easter_egg(robots: &mut [Robot]) -> usize {
    let mut steps = 0;
    while !unique_positions(robots) {
        for robot in &mut *robots {
            robot.step();
        }
        steps += 1;
    }
    steps
}

// is every robot on a unique position?
fn unique_positions(robots: &[Robot]) -> bool {
    let mut positions: Vec<(i64, i64)> = robots.iter().map(|p| p.pos).collect::<Vec<(i64, i64)>>();
    positions.sort();
    positions.dedup();
    positions.len() == robots.len()
}

// print the map with robot positions to see the Easter egg
fn print_positions(robots: &[Robot], map: &(i64, i64)) {
    let mut ee = Vec::new();
    (0..map.1).for_each(|_| {
        ee.push(vec!['.'; map.0 as usize]);
    });
    let positions: Vec<(i64, i64)> = robots.iter().map(|p| p.pos).collect::<Vec<(i64, i64)>>();
    for position in positions {
        ee[position.1 as usize][position.0 as usize] = '*';
    }
    for line in ee {
        println!("{}", line.into_iter().collect::<String>());
    }
}

// read robot configurations
fn read_data(filename: &str, map: (i64, i64)) -> Vec<Robot> {
    let mut machines = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            let data = line.split(" ").collect::<Vec<&str>>();
            assert_eq!(data.len(), 2);
            let pos = data[0]
                .split("=")
                .nth(1)
                .expect("Position string wrong")
                .split(",")
                .collect::<Vec<&str>>();
            assert_eq!(pos.len(), 2);
            let vel = data[1]
                .split("=")
                .nth(1)
                .expect("Velocity string wrong")
                .split(",")
                .collect::<Vec<&str>>();
            assert_eq!(vel.len(), 2);
            let x = pos[0].parse::<i64>().expect("Couldn't parse number");
            let y = pos[1].parse::<i64>().expect("Couldn't parse number");
            let vx = vel[0].parse::<i64>().expect("Couldn't parse number");
            let vy = vel[1].parse::<i64>().expect("Couldn't parse number");
            machines.push(Robot::new((x, y), (vx, vy), map));
        }
    }
    machines
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
        let mut robot = Robot::new((0, 0), (-1, -1), (3, 3));
        robot.step();
        assert_eq!(robot.pos, (2, 2));
        let mut robot = Robot::new((2, 0), (1, -1), (3, 3));
        robot.step();
        assert_eq!(robot.pos, (0, 2));
        let mut robot = Robot::new((2, 2), (1, 1), (3, 3));
        robot.step();
        assert_eq!(robot.pos, (0, 0));
        let mut robot = Robot::new((0, 2), (-1, 1), (3, 3));
        robot.step();
        assert_eq!(robot.pos, (2, 0));
        let mut robot = Robot::new((2, 4), (2, -3), (11, 7));
        robot.step();
        assert_eq!(robot.pos, (4, 1));
        robot.step();
        assert_eq!(robot.pos, (6, 5));
        robot.step();
        assert_eq!(robot.pos, (8, 2));
        robot.step();
        assert_eq!(robot.pos, (10, 6));
        robot.step();
        assert_eq!(robot.pos, (1, 3));
        let mut robot = Robot::new((2, 4), (2, -3), (11, 7));
        robot.nstep(5);
        assert_eq!(robot.pos, (1, 3));
    }
    #[test]
    fn part1() {
        let mut robots = read_data("input.test", (11, 7));
        for robot in &mut robots {
            robot.nstep(100);
        }
        assert_eq!(calc_safety(&robots, &(11, 7)), 12);

        let map = (101, 103);
        let mut robots = read_data("input", map);
        for robot in &mut robots {
            robot.nstep(100);
        }
        assert_eq!(calc_safety(&robots, &map), 211773366);
    }
    #[test]
    fn part2() {
        let map = (101, 103);
        let mut robots = read_data("input", map);
        assert_eq!(calc_easter_egg(&mut robots), 7344);
    }
}
