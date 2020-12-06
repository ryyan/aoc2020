// AOC 2020 Day 6: Custom Customs
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let mut results: Vec<Group> = Vec::new();
    let mut size = 0;
    let mut answers = HashMap::new();

    let file = File::open("input")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_string = line.unwrap();
        if line_string == "" {
            results.push(Group {
                size: size,
                answers: answers.clone(),
            });
            size = 0;
            answers.clear();
            continue;
        }

        size += 1;
        for c in line_string.chars() {
            if answers.contains_key(&c) {
                *answers.get_mut(&c).unwrap() += 1;
            } else {
                answers.insert(c, 1);
            }
        }
    }

    // push final answer group
    results.push(Group {
        size: size,
        answers: answers.clone(),
    });

    // part 1 anyone answered yes
    let sum_any: usize = results.iter().map(|result| result.answers.len()).sum();
    println!("Sum of any counts = {}", sum_any);
    assert_eq!(sum_any, 6625);

    // part 1 everyone answered yes
    let mut sum_all = 0;
    for group in results {
        for (_, size) in group.answers.into_iter() {
            if size == group.size {
                sum_all += 1;
            }
        }
    }
    println!("Sum of all counts = {}", sum_all);
    assert_eq!(sum_all, 3360);

    return Ok(());
}

struct Group {
    size: usize,
    answers: HashMap<char, usize>,
}
