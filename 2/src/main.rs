// AOC 2020
// Day 2: Password Philosophy
// Verify passwords satisfy a given policy
//
// How to run:
// cargo build && cargo run
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    run_unit_test();
    return run_input();
}

fn run_unit_test() {
    let test_input: [&str; 3] = ["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];
    let mut test_results_1 = Vec::new();
    let mut test_results_2 = Vec::new();

    for input in test_input.iter() {
        let password = &parse(input);
        test_results_1.push(check_v1(password));
        test_results_2.push(check_v2(password));
    }

    assert_eq!(test_results_1, [true, false, true]);
    assert_eq!(test_results_2, [true, false, false]);
}

fn run_input() -> io::Result<()> {
    let mut total = 0;

    let file = File::open("input")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let input = &line?;
        let password = &parse(input);
        let is_valid = check_v2(password);

        if is_valid {
            println!("{}", input);
            total = total + 1;
        }
    }

    println!("Total valid passwords: {}", total);
    return Ok(());
}

struct Policy {
    letter: char, // character
    min: usize,   // minimum number
    max: usize,   // maximum number
}

struct Password {
    value: String,
    policy: Policy,
}

// parse password and policy from input line
fn parse(input: &str) -> Password {
    let mut iter = input.split_whitespace();
    let minmax: Vec<&str> = iter.next().unwrap().split("-").collect();
    let min = minmax[0].parse::<usize>().unwrap();
    let max = minmax[1].parse::<usize>().unwrap();
    let letter = iter
        .next()
        .unwrap()
        .trim_end_matches(":")
        .parse::<char>()
        .unwrap();
    let policy = Policy { letter, min, max };
    let value = iter.next().unwrap().to_string();
    return Password { value, policy };
}

// version 1: use min/max constraint, character must be used at least min and at most max
fn check_v1(password: &Password) -> bool {
    let num: usize = password
        .value
        .matches(password.policy.letter)
        .collect::<String>()
        .len();
    return num >= password.policy.min && num <= password.policy.max;
}

// version 2: use position constraint, character must exist at exactly one position of either min/max
fn check_v2(password: &Password) -> bool {
    // translate min/max positions to array positions
    let min = password.policy.min - 1;
    let max = password.policy.max - 1;

    // verify max position isn't greater than password length
    if max > password.value.len() {
        return false;
    }

    // verify at least one position has the required character
    if password.value.chars().nth(min).unwrap() != password.policy.letter
        && password.value.chars().nth(max).unwrap() != password.policy.letter
    {
        return false;
    }

    // verify the characters at min/max postions do not match
    if password.value.chars().nth(min).unwrap() == password.value.chars().nth(max).unwrap() {
        return false;
    }

    // password is valid
    return true;
}
