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

    let mut iter = reader.lines();
    loop {
        match iter.next() {
            Some(v) => {
                let line = v.unwrap();
                if line == "" {
                    results.push(Group {
                        size: size,
                        answers: answers.clone(),
                    });
                    size = 0;
                    answers.clear();
                    continue;
                }

                size += 1;
                for c in line.chars() {
                    if answers.contains_key(&c) {
                        *answers.get_mut(&c).unwrap() += 1;
                    } else {
                        answers.insert(c, 1);
                    }
                }
            }

            None => {
                // when input runs out, catch the final result
                results.push(Group {
                    size: size,
                    answers: answers.clone(),
                });

                break;
            }
        }
    }

    // part 1 count answers where anyone answered yes
    let sum_any: usize = results.iter().map(|result| result.answers.len()).sum();
    println!("Sum of any counts = {}", sum_any);
    assert_eq!(sum_any, 6625);

    // part 2 count answers where everyone answered yes
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
    size: usize,                   // group size
    answers: HashMap<char, usize>, // map of answer and number of times it was given
}
