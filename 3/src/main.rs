// AOC 2020
// Day 3: Toboggan Trajectory
// Count the number of trees given a 2D array and a trajectory
//
// How to run:
// cargo build && cargo run
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let map = build_map().unwrap();

    let mut results: Vec<usize> = Vec::new();
    let slopes = [
        // [right, down]
        [1, 1],
        [3, 1],
        [5, 1],
        [7, 1],
        [1, 2],
    ];

    for slope in &slopes {
        let result = traverse(&map, slope[0], slope[1]);
        println!("Slope{:?} trees = {}", slope, result);
        results.push(result);
    }

    let total = results.iter().product::<usize>();
    println!("Results multiplied = {}", total);

    assert_eq!(results, [55, 250, 54, 55, 39]);
    assert_eq!(total, 1592662500);
    return Ok(());
}

// parse input into a 2-dimensional array
fn build_map() -> io::Result<Vec<Vec<char>>> {
    let mut result: Vec<Vec<char>> = Vec::new();

    let file = File::open("input")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        result.push(line.unwrap().chars().collect::<Vec<char>>());
    }

    return Ok(result);
}

// traverse map in a right/down pattern, starting at position (0,0), and count the number of trees ("#")
fn traverse(map: &Vec<Vec<char>>, right: usize, down: usize) -> usize {
    let (mut total, mut row, mut col) = (0, 0, 0);
    let length = map[0].len();

    for _ in 0..map.len() {
        row = row + down;
        if row >= map.len() {
            break;
        }

        col = col + right;
        if col >= length {
            col = col - length;
        }

        if map[row][col] == '#' {
            total = total + 1;
        }
    }

    return total;
}
