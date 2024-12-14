// Advent of Code 12.12.2024
// - read a 2d topographic garden map with planting information
//   - each position marks the plant on it with a character
//   - there are continuous regions that have the same plant
//   - the regions shall be surrounded by fences
//   - calculate the price of the fences
// - part 1:
//   - the price of the fences is calculated by multiplying area and perimeter
//     of a region
// - part 2:
//   - the price of the fences is calculated by multiplying area and number of
//     sides of a region

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let map = read_data("input.test");
    let area = inspect_area(&map);
    let regions = get_regions(&map, &area);
    let region_perimeters = get_region_perimeters(&map, &regions);
    assert_eq!(get_price(region_perimeters), 1930);

    let map = read_data("input");
    let area = inspect_area(&map);
    let regions = get_regions(&map, &area);
    let region_perimeters = get_region_perimeters(&map, &regions);
    let price = get_price(region_perimeters);
    assert_eq!(price, 1319878);
    println!("The price for all perimeters is {}", price);

    let map = read_data("input.test");
    let area = inspect_area(&map);
    let regions = get_regions(&map, &area);
    let region_perimeters = get_region_corners(&map, &regions);
    assert_eq!(get_price(region_perimeters), 1206);

    let map = read_data("input");
    let area = inspect_area(&map);
    let regions = get_regions(&map, &area);
    let region_perimeters = get_region_corners(&map, &regions);
    let price = get_price(region_perimeters);
    assert_eq!(price, 784982);
    println!("The discount price for all perimeters is {}", price);
}

// find the different plants and their positions
fn inspect_area(map: &[Vec<char>]) -> HashMap<char, Vec<(usize, usize)>> {
    let mut plants: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    (0..map.len()).for_each(|y| {
        (0..map[y].len()).for_each(|x| {
            plants
                .entry(map[y][x])
                .and_modify(|p: &mut Vec<(usize, usize)>| p.push((x, y)))
                .or_insert(vec![(x, y)]);
        });
    });
    plants
}

// detect the regions in the map
fn get_regions(
    map: &[Vec<char>],
    area: &HashMap<char, Vec<(usize, usize)>>,
) -> HashMap<char, Vec<Vec<(usize, usize)>>> {
    let mut regions: HashMap<char, Vec<Vec<(usize, usize)>>> = HashMap::new();
    for (plant, positions) in area {
        for position in positions {
            // check if the positions is already in a region
            let mut position_in_regions = false;
            if let Some(all_regions) = regions.get(plant) {
                for region in all_regions {
                    if region.contains(position) {
                        position_in_regions = true;
                        break;
                    }
                }
            } else {
                regions.insert(*plant, Vec::new());
            }
            if position_in_regions {
                continue;
            }
            // check if a neighbor is in a region and add the position to the
            // region or create a new region
            let neighbors = get_neighbors(map, position);
            let mut neighbour_in_regions = false;
            for neighbor in neighbors {
                if let Some(all_regions) = regions.get_mut(plant) {
                    for region in all_regions {
                        if region.contains(&neighbor) {
                            region.push(*position);
                            neighbour_in_regions = true;
                            break;
                        }
                    }
                }
                if neighbour_in_regions {
                    break;
                }
            }
            if !neighbour_in_regions {
                regions
                    .get_mut(plant)
                    .expect("No regions")
                    .push(vec![*position])
            }
        }

        let mut join = join_regions(map, regions.get_mut(plant).expect("Plant not found"));
        while join {
            join = join_regions(map, regions.get_mut(plant).expect("Plant not found"));
        }
        regions
            .get_mut(plant)
            .expect("Plant not found")
            .iter_mut()
            .for_each(|v| v.sort());
    }
    regions
}

// join regions in a vector, if they belong together
fn join_regions(map: &[Vec<char>], regions: &mut Vec<Vec<(usize, usize)>>) -> bool {
    let mut join = false;
    for i in 0..regions.len() {
        for j in 0..regions.len() {
            if i == j {
                continue;
            }
            for e in &regions[i] {
                let neighbors = get_neighbors(map, e);
                for neighbor in neighbors {
                    if regions[j].contains(&neighbor) {
                        join = true;
                        break;
                    }
                }
                if join {
                    break;
                }
            }
            if join {
                let mut t = regions.remove(j);
                regions[i].append(&mut t);
                break;
            }
        }
        if join {
            break;
        }
    }
    join
}

// get the perimeter details
fn get_region_perimeters(
    map: &[Vec<char>],
    regions: &HashMap<char, Vec<Vec<(usize, usize)>>>,
) -> HashMap<char, Vec<(usize, usize)>> {
    let mut perimeters: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for plant in regions.keys() {
        if let Some(all_regions) = regions.get(plant) {
            for region in all_regions {
                let mut region_perimeter = 0;
                let mut region_size = 0;
                for position in region {
                    let neighbors = get_neighbors(map, position);
                    let mut perimeter = match neighbors.len() {
                        3 => 1,
                        2 => 2,
                        _ => 0,
                    };
                    for neighbor in neighbors {
                        if map[neighbor.1][neighbor.0] != *plant {
                            perimeter += 1;
                        }
                    }
                    region_size += 1;
                    region_perimeter += perimeter;
                }
                perimeters
                    .entry(*plant)
                    .and_modify(|p: &mut Vec<(usize, usize)>| {
                        p.push((region_size, region_perimeter))
                    })
                    .or_insert(vec![(region_size, region_perimeter)]);
            }
        }
    }
    perimeters
}

// get the neighboring positions of a position
fn get_neighbors(map: &[Vec<char>], position: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    // left
    if position.0 > 0 {
        neighbors.push((position.0 - 1, position.1));
    }
    // right
    if position.0 < map[0].len() - 1 {
        neighbors.push((position.0 + 1, position.1));
    }
    // above
    if position.1 > 0 {
        neighbors.push((position.0, position.1 - 1));
    }
    // below
    if position.1 < map.len() - 1 {
        neighbors.push((position.0, position.1 + 1));
    }
    neighbors
}

// calculate the perimeter price
fn get_price(region_perimeters: HashMap<char, Vec<(usize, usize)>>) -> usize {
    let mut sum = 0;
    for all_regions in region_perimeters.values() {
        for region in all_regions {
            sum += region.0 * region.1;
        }
    }
    sum
}

// get the corners of each region
// a region has the same amount of corners as edges
// and corner detection is easier
fn get_region_corners(
    map: &[Vec<char>],
    regions: &HashMap<char, Vec<Vec<(usize, usize)>>>,
) -> HashMap<char, Vec<(usize, usize)>> {
    let mut corners: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for plant in regions.keys() {
        if let Some(all_regions) = regions.get(plant) {
            for region in all_regions {
                let mut region_edges = 0;
                let mut region_size = 0;
                for position in region {
                    region_size += 1;
                    region_edges += get_corners(map, position);
                }
                corners
                    .entry(*plant)
                    .and_modify(|p: &mut Vec<(usize, usize)>| p.push((region_size, region_edges)))
                    .or_insert(vec![(region_size, region_edges)]);
            }
        }
    }
    corners
}

// get the amount of corners of a position
// check all 8 surrounding positions and detect if it's a corner
// return the amount of corners on this position
fn get_corners(map: &[Vec<char>], position: &(usize, usize)) -> usize {
    let n = if position.1 > 0 {
        is_same(position.0, position.1 - 1, map[position.1][position.0], map)
    } else {
        false
    };
    let ne = if position.1 > 0 {
        is_same(
            position.0 + 1,
            position.1 - 1,
            map[position.1][position.0],
            map,
        )
    } else {
        false
    };
    let e = is_same(position.0 + 1, position.1, map[position.1][position.0], map);
    let se = is_same(
        position.0 + 1,
        position.1 + 1,
        map[position.1][position.0],
        map,
    );
    let s = is_same(position.0, position.1 + 1, map[position.1][position.0], map);
    let sw = if position.0 > 0 {
        is_same(
            position.0 - 1,
            position.1 + 1,
            map[position.1][position.0],
            map,
        )
    } else {
        false
    };
    let w = if position.0 > 0 {
        is_same(position.0 - 1, position.1, map[position.1][position.0], map)
    } else {
        false
    };
    let nw = if position.0 > 0 && position.1 > 0 {
        is_same(
            position.0 - 1,
            position.1 - 1,
            map[position.1][position.0],
            map,
        )
    } else {
        false
    };

    let mut corners = 0;
    if n && w && !nw {
        corners += 1;
    }
    if n && e && !ne {
        corners += 1;
    }
    if s && w && !sw {
        corners += 1;
    }
    if s && e && !se {
        corners += 1;
    }
    if !(n || w) {
        corners += 1;
    }
    if !(n || e) {
        corners += 1;
    }
    if !(s || w) {
        corners += 1;
    }
    if !(s || e) {
        corners += 1;
    }

    corners
}

// check if a position has the same plant and is in range
// because of usize we can't do x,y < 0 here
fn is_same(x: usize, y: usize, plant: char, map: &[Vec<char>]) -> bool {
    let xr = 0..map[0].len();
    let yr = 0..map.len();
    xr.contains(&x) && yr.contains(&y) && map[y][x] == plant
}

// read a garden map file with plant information
fn read_data(filename: &str) -> Vec<Vec<char>> {
    let mut map = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for y in lines.map_while(Result::ok) {
            map.push(y.chars().collect());
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
