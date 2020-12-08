// AOC 2020 Day 7: Handy Haversacks
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::BufRead, BufReader};

fn main() -> io::Result<()> {
    let bags = parse_input().unwrap();

    // part 1 count bags that can contain at least one shiny gold bag
    let mut total = 0;
    for color in bags.keys() {
        if unpack(&bags, color).contains_key("shiny gold") {
            total += 1;
        }
    }
    println!("Total bags that can contain a shiny gold bag = {}", total);
    assert_eq!(total, 289);

    // part 2 count the total number of bags contained in one shiny gold bag
    total = total_bags(&bags, &"shiny gold".to_string());
    println!("Total bags within one shiny gold bag = {}", total);
    assert_eq!(total, 30055);

    return Ok(());
}

// recursively unpack all bags within a bag
fn unpack(
    all_bags: &HashMap<String, HashMap<String, usize>>,
    color: &String,
) -> HashMap<String, usize> {
    let inner_bags = all_bags.get(color).unwrap();

    if inner_bags.is_empty() {
        return HashMap::new();
    }

    let mut result: HashMap<String, usize> = HashMap::new();
    for (color, number) in inner_bags {
        result.insert(color.to_string(), *number);
        result.extend(unpack(all_bags, color));
    }

    return result;
}

// recursively count the number of bags contained within a bag
fn total_bags(all_bags: &HashMap<String, HashMap<String, usize>>, color: &String) -> usize {
    let inner_bags = all_bags.get(color).unwrap();

    if inner_bags.is_empty() {
        return 0;
    }

    let mut result = 0;
    for (color, number) in inner_bags {
        result += number;
        result += number * total_bags(all_bags, color);
    }

    return result;
}

fn parse_input() -> io::Result<HashMap<String, HashMap<String, usize>>> {
    // key=bag color, value=map of bags it can contain and the number of each
    let mut results: HashMap<String, HashMap<String, usize>> = HashMap::new();

    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut iter = reader.lines();
    while let Some(v) = iter.next() {
        let mut line = v.unwrap();
        line = line
            .replace(" contain ", "")
            .replace(", ", "")
            .replace("bags", "bag");

        let mut list = line.split("bag").collect::<Vec<&str>>();

        // pop off the first element, which is the current bag
        let color = list.remove(0).trim().to_string();

        // iterate over the rest of the bags which the current bag contains
        let mut bags: HashMap<String, usize> = HashMap::new();
        while let Some(item) = list.pop() {
            if item == "." || item == "no other " {
                continue;
            }

            // split at the first whitespace
            // 1 gold bag becomes ["1", "gold bag"]
            let bag = item.splitn(2, " ").collect::<Vec<&str>>();
            bags.insert(bag[1].trim().to_string(), bag[0].parse::<usize>().unwrap());
        }

        results.insert(color, bags);
    }

    return Ok(results);
}
