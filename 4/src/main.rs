// AOC 2020
// Day 4: Passport Processing
// Count the number of valid passports
//
// How to run:
// cargo build && cargo run
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let passports = parse_input().unwrap();

    let total_1 = count_valid_passports_v1(&passports);
    assert_eq!(total_1, 213);
    println!("Total valid passports v1: {}", total_1);

    let total_2 = count_valid_passports_v2(&passports);
    assert_eq!(total_2, 147);
    println!("Total valid passports v2: {}", total_2);
    return Ok(());
}

struct Passport {
    fields: HashMap<String, String>,
}

fn parse_input() -> io::Result<Vec<Passport>> {
    let mut result: Vec<Passport> = Vec::new();
    let mut fields = HashMap::new();

    let file = File::open("input")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_string = line.unwrap();
        if line_string == "" {
            result.push(Passport {
                fields: fields.clone(),
            });

            fields.clear();
            continue;
        }

        parse_line(line_string, &mut fields);
    }

    // push final passport since reader.lines() doesn't return final newline
    result.push(Passport {
        fields: fields.clone(),
    });

    return Ok(result);
}

fn parse_line(line: String, fields: &mut HashMap<String, String>) {
    let pairs = line.split(" ").collect::<Vec<&str>>();
    for pair in pairs {
        let kv = pair.split(":").collect::<Vec<&str>>();
        fields.insert(kv[0].to_string(), kv[1].to_string());
    }
}

// v1 valid passports have all fields, except for cid (country id) which is optional
fn count_valid_passports_v1(passports: &Vec<Passport>) -> usize {
    let mut count = 0;

    for pass in passports {
        if pass.fields.len() == 8
            || (pass.fields.len() == 7 && pass.fields.contains_key("cid") == false)
        {
            // passport is valid
            count += 1;
        }
    }

    return count;
}

// v2 valid passports have all fields with valid data, except for cid which is optional and not validated
fn count_valid_passports_v2(passports: &Vec<Passport>) -> usize {
    let mut count = 0;

    for pass in passports {
        if pass.fields.len() == 8
            || (pass.fields.len() == 7 && pass.fields.contains_key("cid") == false)
        {
            // validate birth year
            let byr = pass.fields.get("byr").unwrap().parse::<usize>().unwrap();
            if byr < 1920 || byr > 2002 {
                continue;
            }

            // validate issue year
            let iyr = pass.fields.get("iyr").unwrap().parse::<usize>().unwrap();
            if iyr < 2010 || iyr > 2020 {
                continue;
            }

            // validate expiration year
            let eyr = pass.fields.get("eyr").unwrap().parse::<usize>().unwrap();
            if eyr < 2020 || eyr > 2030 {
                continue;
            }

            // validate height
            let hgt = pass.fields.get("hgt").unwrap();
            if hgt.contains("cm") {
                let cm = hgt.trim_end_matches("cm").parse::<usize>().unwrap();
                if cm < 150 || cm > 193 {
                    continue;
                }
            } else if hgt.contains("in") {
                let inches = hgt.trim_end_matches("in").parse::<usize>().unwrap();
                if inches < 59 || inches > 76 {
                    continue;
                }
            } else {
                // if it's not "cm" or "in", it's invalid
                continue;
            }

            // validate hair color
            let hcl = pass.fields.get("hcl").unwrap();
            if hcl.contains("#") == false {
                continue;
            }
            let hair_color = hcl.trim_start_matches("#");
            if hair_color.chars().all(char::is_alphanumeric) == false {
                continue;
            }

            // validate eye color
            let ecl = pass.fields.get("ecl").unwrap();
            match ecl.as_str() {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => (), // do nothing
                _ => continue,
            }

            // validate passport id
            let pid = pass.fields.get("pid").unwrap();
            if pid.len() != 9 {
                continue;
            }
            match pid.parse::<usize>() {
                Ok(_) => (), // do nothing
                Err(_) => continue,
            }

            // passport is valid
            count += 1;
        }
    }

    return count;
}
