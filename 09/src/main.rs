// Advent of Code 09.12.2024
// - read a disk map with file and free space allocation
//   - the disk map is a vector where
//     - even indices represent a file
//     - odd indices represent free space
//     - the value at an index represents the amount of blocks occupied
//     - files are addressed with an ID, starting at 0, so the file at index 4
//       has file ID 2
//   - an antenna is marked by the frequency it's sending on
//   - valid frequencies are marked by [a-zA-Z0-9]
//   - two antennas of the same frequency create antinodes
//     - the antinodes occur on an imaginary line through the antennas
// - part 1:
//   - defragment the disk by moving the rightmost file block
//     to the left most free block
//   - calculate the new file system checksum by multiplying each block ID
//     (the leftmost block has ID 0) with the file ID it contains (free
//     space is skipped)
// - part 2:
//   - defragment the disk by moving the rightmost file to the left most
//     bin of free blocks (that can hold the file)
//   - calculate the new file system checksum as in part 1

fn main() {
    let disk_map = read_data("input.test");
    let mut disk_layout = generate_layout(&disk_map);
    naive_defragment_disk(&mut disk_layout);
    let checksum = calculate_checksum(&disk_layout);
    assert_eq!(checksum, 1928);

    let disk_map = read_data("input");
    let mut disk_layout = generate_layout(&disk_map);
    naive_defragment_disk(&mut disk_layout);
    let checksum = calculate_checksum(&disk_layout);
    assert_eq!(checksum, 6242766523059);
    println!("The new disk checksum is {}", checksum);

    let disk_map = read_data("input.test");
    let mut disk_layout = generate_layout(&disk_map);
    defragment_disk_2(&mut disk_layout);
    let checksum = calculate_checksum(&disk_layout);
    assert_eq!(checksum, 2858);

    let disk_map = read_data("input");
    let mut disk_layout = generate_layout(&disk_map);
    defragment_disk_2(&mut disk_layout);
    let checksum = calculate_checksum(&disk_layout);
    assert_eq!(checksum, 6272188244509);
    println!("The new file optimized disk checksum is {}", checksum);
}

// generate the disk layout from the disk_map
fn generate_layout(disk_map: &[i64]) -> Vec<i64> {
    let mut layout = Vec::new();
    let mut file_id: i64 = 0;
    (0..disk_map.len()).for_each(|index| {
        if index % 2 == 0 {
            (0..disk_map[index]).for_each(|_| layout.push(file_id));
            file_id += 1;
        } else {
            (0..disk_map[index]).for_each(|_| layout.push(-1));
        }
    });
    layout
}

// naive approach on disk defragmenting
fn naive_defragment_disk(disk_layout: &mut [i64]) {
    while !is_defragmented(disk_layout) {
        swap_blocks(disk_layout);
    }
}

// swap the rightmost file block and the leftmost free block
fn swap_blocks(disk_layout: &mut [i64]) {
    let first_free_pos = disk_layout
        .iter()
        .position(|&e| e == -1)
        .expect("No free space found");
    let last_used_pos = disk_layout.len()
        - disk_layout
            .iter()
            .rev()
            .position(|&e| e > -1)
            .expect("No file ID found")
        - 1;
    disk_layout[first_free_pos] = disk_layout[last_used_pos];
    disk_layout[last_used_pos] = -1;
}

// check if the disk is defragmented (naive approach)
fn is_defragmented(disk_layout: &[i64]) -> bool {
    let mut defragmented = true;
    let first_free_pos = disk_layout
        .iter()
        .position(|&e| e == -1)
        .expect("No free space found");
    for idx in &disk_layout[first_free_pos..] {
        if *idx > -1 {
            defragmented = false;
            break;
        }
    }
    defragmented
}

// calculate the file system checksum
fn calculate_checksum(disk_layout: &[i64]) -> i64 {
    let mut sum = 0;
    for (idx, val) in disk_layout.iter().enumerate() {
        if *val > -1 {
            sum += (idx as i64) * val;
        }
    }
    sum
}

// file based defragmenting
fn defragment_disk_2(disk_layout: &mut [i64]) {
    let last = disk_layout.last().expect("No last element");
    for file_id in (0..=*last).rev() {
        let (file_start, file_size) = get_file_block_data(file_id, disk_layout);
        if let Some(free_start) = find_free_block_bin(file_size, disk_layout) {
            if free_start < file_start {
                for idx in 0..file_size {
                    disk_layout[free_start + idx] = disk_layout[file_start + idx];
                    disk_layout[file_start + idx] = -1;
                }
            }
        }
    }
}

// get start and size of file block for file ID
fn get_file_block_data(file_id: i64, disk_layout: &[i64]) -> (usize, usize) {
    let file_start = disk_layout
        .iter()
        .position(|&e| e == file_id)
        .expect("File ID start not found");
    let file_end = disk_layout.len()
        - disk_layout
            .iter()
            .rev()
            .position(|&e| e == file_id)
            .expect("File ID end not found")
        - 1;
    let file_size = file_end - file_start + 1;
    (file_start, file_size)
}

// try to find a bin of free blocks with size
fn find_free_block_bin(size: usize, disk_layout: &[i64]) -> Option<usize> {
    let mut skip = 0;
    let mut free_start: usize = 0;
    let mut free_end: usize;
    let mut free_size: usize = 0;
    let mut valid = true;
    while free_size < size {
        if let Some(e) = disk_layout.iter().skip(skip).position(|&e| e == -1) {
            free_start = e + skip;
        } else {
            valid = false;
            break;
        };
        if let Some(e) = disk_layout.iter().skip(free_start).position(|&e| e > -1) {
            free_end = e + free_start;
        } else {
            valid = false;
            break;
        }
        skip = free_end;
        free_size = free_end - free_start;
    }
    if valid {
        Some(free_start)
    } else {
        None
    }
}

// read a disk map file
fn read_data(filename: &str) -> Vec<i64> {
    std::fs::read_to_string(filename)
        .expect("Can't read input")
        .trim()
        .chars()
        .map(|c| c.to_string().parse::<i64>().expect("Can't parse number"))
        .collect::<Vec<i64>>()
}
