// AOC 2020 Day 10: Adapter Array
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let adapters = parse_input().unwrap();

    // part 1 jolt differences
    let differences = compute_volt_differences(&adapters);
    let answer = differences.get(&1).unwrap() * differences.get(&3).unwrap();
    println!("Jolt differences: {:?}", differences);
    println!("1-jolt differences * 3-jolt differences: {}", answer);
    assert_eq!(answer, 2812);

    // part 2 total numbers of distinct arrangements
    let arrangements = compute_arrangements(&adapters);
    println!("Total possible arrangements = {}", arrangements);

    return Ok(());
}

fn parse_input() -> io::Result<Vec<usize>> {
    let mut results: Vec<usize> = Vec::new();

    let file = File::open("input")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        results.push(line.unwrap().parse::<usize>().unwrap());
    }

    // sort results
    results.sort();

    // add the outlet (always 0)
    results.insert(0, 0);

    // add the built-in joltage adapter (3 higher than the highest-rated adapter)
    results.push(results.get(results.len() - 1).unwrap() + 3);

    return Ok(results);
}

fn compute_volt_differences(adapters: &Vec<usize>) -> HashMap<usize, usize>  {
    let mut results: HashMap<usize, usize> = HashMap::new();
    for i in 0..adapters.len() -1 {
        let difference = adapters[i+1] - adapters[i];

        if results.contains_key(&difference) {
            results.insert(difference, results[&difference] + 1);
        } else {
            results.insert(difference, 1);
        }
    }

    return results;
}

fn compute_arrangements(adapters: &Vec<usize>) -> usize {
    let mut result = 1;

    for i in 0..adapters.len() - 1 {
        let adapter = adapters.get(i).unwrap();

        for x in i + 2..i + 7 {
            if x > adapters.len() - 1 {
                break;
            }

            let prev_adapter = adapters.get(x-1).unwrap();
            let next_adapter = adapters.get(x).unwrap();
            let diff = next_adapter - adapter;
            if diff > 3 {
                break;
            }

            println!("{}, {}, {}, {}, {}, {}", i, x, adapter, prev_adapter, next_adapter, diff);
            result += 1
        }
    }

    return result;
}

012
02

0123
013
023
03

0-23
1-3

01234
0124
0134
0234
014
024
034

0-23
1-34
2-4

012345
01235
01245
02345
0235
0245
0135
0145
025

0-23
1-34
2-45
3-5

012347
02347
01247
01347
0147
0247
0347

0-23
1-34
2-4

0123456
012346
012456
012356
023456