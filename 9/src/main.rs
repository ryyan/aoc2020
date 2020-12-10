// AOC 2020 Day 9: Encoding Error
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let numbers = parse_input().unwrap();

    // part 1 what's the first number that is not a sum of two of its previous 25 numbers?
    let invalid_number = find_bugged_number(&numbers);
    println!("Invalid number = {}", invalid_number);
    assert_eq!(invalid_number, 105950735);

    // part 2 find contiguous set of at least 2 numbers that sum to the invalid number
    let encryption_weakness = find_encryption_weakness(&numbers, invalid_number);
    println!("Encryption weakness = {}", encryption_weakness);
    assert_eq!(encryption_weakness, 13826915);

    return Ok(());
}

fn parse_input() -> io::Result<Vec<usize>> {
    let mut results: Vec<usize> = Vec::new();

    let file = File::open("input")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        results.push(line.unwrap().parse::<usize>().unwrap());
    }

    return Ok(results);
}

fn find_bugged_number(numbers: &Vec<usize>) -> usize {
    for i in 25..numbers.len() {
        if has_summation(numbers, i) == false {
            return numbers[i];
        }
    }

    return 0;
}

fn has_summation(numbers: &Vec<usize>, i: usize) -> bool {
    let number = numbers[i];

    for x in i - 25..i {
        if numbers[x] > number {
            continue;
        }

        let remainder = number - numbers[x];
        if numbers[x..i].contains(&remainder) {
            return true;
        }
    }

    return false;
}

fn find_encryption_weakness(numbers: &Vec<usize>, invalid_number: usize) -> usize {
    for i in 0..numbers.len() {
        let mut total = numbers[i];

        for x in i + 1..numbers.len() {
            if total > invalid_number {
                break;
            }

            if total == invalid_number {
                return compute_encryption_weakness(&numbers[i..x].to_vec());
            }

            total += numbers[x];
        }
    }

    return 0;
}

// returns the sum of the smallest and largest numbers
fn compute_encryption_weakness(numbers: &Vec<usize>) -> usize {
    let mut min = usize::MAX;
    let mut max = 0;

    for num in numbers {
        if *num < min {
            min = *num;
        } else if *num > max {
            max = *num;
        }
    }

    return min + max;
}
