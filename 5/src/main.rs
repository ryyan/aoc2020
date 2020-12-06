// AOC 2020 Day 5: Binary Boarding
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

const ROWS: usize = 127;
const COLS: usize = 7;

fn main() -> io::Result<()> {
    run_tests();
    return run_input();
}

fn run_tests() {
    let mut result = compute_seat("FBFBBFFRLR");
    assert_eq!(result.id, 357);
    assert_eq!(result.row, 44);
    assert_eq!(result.col, 5);

    result = compute_seat("BFFFBBFRRR");
    assert_eq!(result.id, 567);
    assert_eq!(result.row, 70);
    assert_eq!(result.col, 7);

    result = compute_seat("FFFBBBFRRR");
    assert_eq!(result.id, 119);
    assert_eq!(result.row, 14);
    assert_eq!(result.col, 7);

    result = compute_seat("BBFFBBFRLL");
    assert_eq!(result.id, 820);
    assert_eq!(result.row, 102);
    assert_eq!(result.col, 4);
}

fn run_input() -> io::Result<()> {
    let mut highest = 0;
    let mut seats = Vec::<Seat>::new();

    let file = File::open("input")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let result = compute_seat(line.unwrap().as_str());
        if result.id > highest {
            highest = result.id;
        }
        seats.push(result);
    }

    // part 1: find highest seat ID
    println!("Highest seat id: {}", highest);
    assert_eq!(highest, 864);

    // part 2: find missing seat ID
    // extract the seat ids into an array
    let seat_ids: Vec<usize> = seats.into_iter().map(|seat| seat.id).collect();
    for row in 1..ROWS {
        for col in 1..COLS {
            let id: usize = compute_seat_id(row, col);

            if seat_ids.contains(&id) {
                continue;
            }

            // the empty seat will have both adjacent seats taken
            if seat_ids.contains(&(id - 1)) && seat_ids.contains(&(id + 1)) {
                println!("Missing seat id: {}", id);
                assert_eq!(id, 739);
            }
        }
    }

    return Ok(());
}

#[derive(Debug)]
struct Seat {
    id: usize,
    row: usize,
    col: usize,
}

fn compute_seat(input: &str) -> Seat {
    let (mut row_min, mut row_max, mut col_min, mut col_max) = (0, ROWS, 0, COLS);
    let (mut last_row, mut last_col) = ("", "");
    let (mut row, mut col) = (0, 0);

    for letter in input.chars() {
        // the +1 makes it a ceiling result, otherwise rust defaults to floor
        let row_split = (row_max + 1 - row_min) / 2;
        let col_split = (col_max + 1 - col_min) / 2;

        match letter {
            'F' => {
                // front, lower half
                row_max -= row_split;
                last_row = "F";
            }
            'B' => {
                // back, upper half
                row_min += row_split;
                last_row = "B";
            }
            'L' => {
                // left, lower half
                col_max -= col_split;
                last_col = "L";
            }
            'R' => {
                // right, upper half
                col_min += col_split;
                last_col = "R";
            }
            _ => (),
        }
    }

    match last_row {
        "F" => row = row_min,
        "B" => row = row_max,
        _ => (),
    }

    match last_col {
        "L" => col = col_min,
        "R" => col = col_max,
        _ => (),
    }

    return Seat {
        id: compute_seat_id(row, col),
        row: row,
        col: col,
    };
}

fn compute_seat_id(row: usize, col: usize) -> usize {
    return (row * 8) + col;
}
