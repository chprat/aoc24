// Advent of Code 15.12.2024
// - read a warehouse map with
//   - positions of walls (marked by #)
//   - boxes (marked by O)
//   - initial robot position (marked by @)
//   - a new line after the map
//   - robot movements after the new line
//     - < for left
//     - ^ for up
//     - > for right
//     - v for down
//   - the robot movements might be split to several lines, but they should be
//     treated as a single line
// - execute the robots movements, to get the final warehouse map
//   - the robot will move boxes when moving to their position
//   - the robot will push multiple boxes
//   - the robot can't move or push boxes, when they'd hit a wall
// - the Goods Positioning System (GPS) is used to track box coordinates
//   - the GPS position is 100 times the distance from the top edge of the map
//   - plus the distance from the left edge of the map
// - part 1:
//   - calculate the sum of all GPS coordinates on the final warehouse map
// - part 2:
//   - in another warehouse everything is twice as big
//     - while reading the original map
//       - every # becomes ##
//       - every O becomes []
//       - every . becomes ..
//       - every @ becomes @.
//   - pushing one box vertically might now affect multiple boxes horizontally
//   - the GPS position calculation doesn't change, the box edge closest to the
//     map edge is used for calculation

use std::collections::HashMap;
use std::collections::VecDeque;
use std::iter::FromIterator;

type PositionMap = HashMap<Type, Vec<Position>>;

fn main() {
    let (mut positions, mut movements) = read_data("input", false);
    let mut robot = positions.remove(&Type::Robot).expect("Robot not found")[0];
    moves(&mut robot, &mut positions, &mut movements, false);
    let sum = sum_coordinates(positions.get(&Type::Box).expect("No boxes found"), false);
    println!("The summed GPS positions are {}", sum);

    let (mut positions, mut movements) = read_data("input", true);
    let mut robot = positions.remove(&Type::Robot).expect("Robot not found")[0];
    moves(&mut robot, &mut positions, &mut movements, true);
    let sum = sum_coordinates(positions.get(&Type::Box).expect("No boxes found"), true);
    println!("The summed GPS positions in the wide warehouse are {}", sum);
}

#[derive(Clone, Debug, PartialEq)]
enum Direction {
    Left,
    Down,
    Up,
    Right,
    None,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Type {
    Wall,
    Box,
    BoxL,
    BoxR,
    Robot,
    Empty,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Position {
    x: usize,
    y: usize,
    typ: Type,
}

impl Position {
    fn new(x: usize, y: usize, typ: Type) -> Self {
        Self { x, y, typ }
    }
    fn step(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.y -= 1,
            Direction::Right => self.x += 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::None => panic!("Unkonw direction"),
        };
    }
    fn new_pos(&self, direction: &Direction, positions: &PositionMap) -> Self {
        let mut pos = Position::new(self.x, self.y, self.typ);
        match direction {
            Direction::Up => pos.y -= 1,
            Direction::Right => pos.x += 1,
            Direction::Down => pos.y += 1,
            Direction::Left => pos.x -= 1,
            Direction::None => panic!("Unkonw direction"),
        };
        if let Some(found) = positions
            .values()
            .flatten()
            .find(|p| p.x == pos.x && p.y == pos.y)
        {
            pos.typ = found.typ;
        } else {
            pos.typ = Type::Empty;
        }
        pos
    }
    fn other_box(&self, positions: &PositionMap) -> Option<Self> {
        let pos = if self.typ == Type::BoxL {
            match positions
                .values()
                .flatten()
                .find(|p| p.x == self.x + 1 && p.y == self.y)
            {
                Some(p) => assert_eq!(p.typ, Type::BoxR),
                None => panic!("Neighbor box not found"),
            };
            Some(Position::new(self.x + 1, self.y, Type::BoxR))
        } else if self.typ == Type::BoxR {
            match positions
                .values()
                .flatten()
                .find(|p| p.x == self.x - 1 && p.y == self.y)
            {
                Some(p) => assert_eq!(p.typ, Type::BoxL),
                None => panic!("Neighbor box not found"),
            };
            Some(Position::new(self.x - 1, self.y, Type::BoxL))
        } else {
            None
        };
        pos
    }
}

// execute all movements
fn moves(
    robot: &mut Position,
    positions: &mut PositionMap,
    movements: &mut Vec<Direction>,
    part2: bool,
) {
    while let Some(direction) = movements.pop() {
        let mut moveables = get_moves(robot, &direction, positions);
        // part 2 differs only in vertical movement and when boxes are affected
        if part2
            && (direction == Direction::Up || direction == Direction::Down)
            && moveables.len() > 1
        {
            let robot = moveables.pop().expect("No last moveable");
            assert_eq!(robot.typ, Type::Robot);
            loop {
                let (new_dangling, movs) = recalculate_moveables(&moveables, &direction, positions);
                moveables = movs;
                if !new_dangling {
                    break;
                }
            }
            if !moveables.is_empty() {
                moveables.push(robot);
            }
        }
        for pos in &moveables {
            if pos.typ != Type::Robot {
                let mbox = positions
                    .get_mut(&Type::Box)
                    .expect("No boxes found")
                    .iter_mut()
                    .find(|p| p.x == pos.x && p.y == pos.y && p.typ == pos.typ)
                    .expect("Position not in boxes");
                mbox.step(&direction);
            } else {
                robot.step(&direction);
            }
        }
    }
}

// get all moves that are required when the robot moves
// moving the robot might involve moving several boxes
fn get_moves(robot: &Position, direction: &Direction, positions: &PositionMap) -> Vec<Position> {
    let mut pos = *robot;
    let mut moveables = Vec::new();
    loop {
        pos = pos.new_pos(direction, positions);
        if pos.typ == Type::Empty || pos.typ == Type::Wall {
            break;
        } else {
            moveables.push(pos);
        }
    }
    if pos.typ == Type::Wall {
        return Vec::new();
    }
    moveables.push(*robot);
    moveables
}

// calculate the sum of all GPS positions
fn sum_coordinates(positions: &[Position], part2: bool) -> usize {
    if !part2 {
        positions.iter().map(|p| p.y * 100 + p.x).sum()
    } else {
        positions
            .iter()
            .filter(|p| p.typ == Type::BoxL)
            .map(|p| p.y * 100 + p.x)
            .sum()
    }
}

// check all affected boxes on vertical movement in part 2
fn recalculate_moveables(
    old_moveables: &[Position],
    direction: &Direction,
    positions: &PositionMap,
) -> (bool, Vec<Position>) {
    let mut new_dangling = false;
    let mut vd_moveables: VecDeque<Position> = VecDeque::from_iter(old_moveables.to_owned());
    let mut moveables = Vec::new();
    while let Some(moveable) = vd_moveables.pop_front() {
        let neighbor = moveable
            .other_box(positions)
            .expect("Neighbor box not found");
        if !old_moveables.contains(&neighbor) {
            moveables.push(neighbor);
            new_dangling = true;
        }
        let new_neighbor = neighbor.new_pos(direction, positions);
        if (new_neighbor.typ == Type::BoxL || new_neighbor.typ == Type::BoxR)
            && !old_moveables.contains(&new_neighbor)
        {
            moveables.push(new_neighbor);
            let neighbor = new_neighbor
                .other_box(positions)
                .expect("Neighbor box not found");
            if !old_moveables.contains(&neighbor) {
                moveables.push(neighbor);
            }
            new_dangling = true;
        }
        moveables.push(moveable);
    }
    moveables.sort();
    moveables.dedup();
    for moveable in &moveables {
        if moveable.new_pos(direction, positions).typ == Type::Wall {
            return (false, Vec::new());
        }
    }
    (new_dangling, moveables)
}

// read the warehouse information
fn read_data(filename: &str, part2: bool) -> (PositionMap, Vec<Direction>) {
    let file = std::fs::read_to_string(filename).expect("Can't read input");
    let parts = file.trim().split("\n\n").collect::<Vec<&str>>();
    assert_eq!(parts.len(), 2);
    let mut map = Vec::new();
    for line in parts[0].split("\n") {
        if line.is_empty() {
            continue;
        }
        map.push(line.chars().collect::<Vec<char>>());
    }
    let movements = parts[1]
        .chars()
        .rev()
        .filter(|&c| c == '>' || c == '<' || c == 'v' || c == '^')
        .map(|c| match c {
            '>' => Direction::Right,
            '<' => Direction::Left,
            '^' => Direction::Up,
            'v' => Direction::Down,
            _ => Direction::None,
        })
        .collect::<Vec<Direction>>();
    let positions = parse_map(&map, part2);
    assert!(positions.contains_key(&Type::Robot));
    assert_eq!(positions.get(&Type::Robot).unwrap().len(), 1);
    (positions, movements)
}

// create a HashMap from the map with robot, wall and box positions
fn parse_map(map: &[Vec<char>], part2: bool) -> PositionMap {
    let mut positions = HashMap::new();
    (0..map.len()).for_each(|y| {
        (0..map[y].len()).for_each(|x| {
            if !part2 {
                let pos = match map[y][x] {
                    '#' => Position::new(x, y, Type::Wall),
                    '@' => Position::new(x, y, Type::Robot),
                    'O' => Position::new(x, y, Type::Box),
                    _ => Position::new(x, y, Type::Empty),
                };
                if pos.typ != Type::Empty {
                    positions
                        .entry(pos.typ)
                        .and_modify(|p: &mut Vec<Position>| p.push(pos))
                        .or_insert(vec![pos]);
                }
            } else {
                let position = match map[y][x] {
                    '#' => vec![
                        Position::new(x * 2, y, Type::Wall),
                        Position::new(x * 2 + 1, y, Type::Wall),
                    ],
                    '@' => vec![
                        Position::new(x * 2, y, Type::Robot),
                        Position::new(x * 2 + 1, y, Type::Empty),
                    ],
                    'O' => vec![
                        Position::new(x * 2, y, Type::BoxL),
                        Position::new(x * 2 + 1, y, Type::BoxR),
                    ],
                    _ => vec![
                        Position::new(x * 2, y, Type::Empty),
                        Position::new(x * 2 + 1, y, Type::Empty),
                    ],
                };
                for pos in position {
                    if pos.typ != Type::Empty {
                        let pos_type = if pos.typ == Type::BoxL || pos.typ == Type::BoxR {
                            Type::Box
                        } else {
                            pos.typ
                        };
                        positions
                            .entry(pos_type)
                            .and_modify(|p: &mut Vec<Position>| p.push(pos))
                            .or_insert(vec![pos]);
                    }
                }
            }
        })
    });
    assert_eq!(
        positions.get(&Type::Robot).expect("Robot not found").len(),
        1
    );
    positions
}

#[allow(dead_code)]
fn print_map(positions: &PositionMap, robot: &Position, x: usize, y: usize) {
    for i in 0..y {
        let mut line = vec!['.'; x];
        let ps = positions
            .values()
            .flatten()
            .filter(|p| p.y == i)
            .copied()
            .collect::<Vec<Position>>();
        for p in ps {
            match p.typ {
                Type::Wall => line[p.x] = '#',
                Type::BoxL => line[p.x] = '[',
                Type::BoxR => line[p.x] = ']',
                Type::Robot => line[p.x] = '@',
                Type::Empty => line[p.x] = ' ',
                Type::Box => line[p.x] = '+',
            }
        }
        if robot.y == i {
            line[robot.x] = '@';
        }
        println!("{:?}", String::from_iter(line));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum() {
        let p = Position::new(4, 1, Type::Box);
        assert_eq!(sum_coordinates(&[p], false), 104);
    }
    #[test]
    fn part1_test() {
        let (mut positions, mut movements) = read_data("input.test", false);
        assert_eq!(
            positions
                .values()
                .flatten()
                .collect::<Vec<&Position>>()
                .len(),
            59
        );
        let mut robot = positions.remove(&Type::Robot).expect("Robot not found")[0];
        moves(&mut robot, &mut positions, &mut movements, false);
        assert_eq!(
            sum_coordinates(positions.get(&Type::Box).expect("No boxes found"), false),
            10092
        );
    }
    #[test]
    fn part1() {
        let (mut positions, mut movements) = read_data("input", false);
        let mut robot = positions.remove(&Type::Robot).expect("Robot not found")[0];
        moves(&mut robot, &mut positions, &mut movements, false);
        assert_eq!(
            sum_coordinates(positions.get(&Type::Box).expect("No boxes found"), false),
            1360570
        );
    }
    #[test]
    fn part2_test() {
        let (mut positions, mut movements) = read_data("input.test", true);
        assert_eq!(
            positions
                .values()
                .flatten()
                .collect::<Vec<&Position>>()
                .len(),
            117
        );
        let mut robot = positions.remove(&Type::Robot).expect("Robot not found")[0];
        moves(&mut robot, &mut positions, &mut movements, true);
        assert_eq!(
            sum_coordinates(positions.get(&Type::Box).expect("No boxes found"), true),
            9021
        );
    }
    #[test]
    fn part2() {
        let (mut positions, mut movements) = read_data("input", true);
        let mut robot = positions.remove(&Type::Robot).expect("Robot not found")[0];
        moves(&mut robot, &mut positions, &mut movements, true);
        assert_eq!(
            sum_coordinates(positions.get(&Type::Box).expect("No boxes found"), true),
            1381446
        );
    }
}
