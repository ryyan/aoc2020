// AOC 2020 Day 8: Handheld Halting
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::BufRead, BufReader};

fn main() -> io::Result<()> {
    let instructions = parse_input().unwrap();

    // part 1 what's the accumulator value before any instruction is executed a second time
    let (accumlator, position) = run(&instructions);
    println!(
        "Program started infinite loop with accumlator at {} and at position {}",
        accumlator, position
    );
    assert_eq!(accumlator, 1317);
    assert_eq!(position, 600);

    // part 2 fix the program by changing exactly one jmp to nop or one nop to jmp
    for i in 0..instructions.len() {
        let new_instruction: Instruction;

        // determine new instruction
        let instruction = instructions.get(i).unwrap();
        match instruction.operation.as_str() {
            "jmp" => {
                new_instruction = Instruction {
                    operation: "nop".to_string(),
                    argument: instruction.argument,
                }
            }
            "nop" => {
                new_instruction = Instruction {
                    operation: "jmp".to_string(),
                    argument: instruction.argument,
                }
            }
            _ => continue,
        }

        // replace instruction
        let mut new_instructions = instructions.clone();
        new_instructions.insert(i, new_instruction);
        new_instructions.remove(i + 1);

        let (accumlator, position) = run(&new_instructions);

        if position >= instructions.len() {
            println!(
                "Program terminated successfully with accumlator at {} and at position {}",
                accumlator, position
            );
            assert_eq!(accumlator, 1033);
            assert_eq!(position, 626);
            break;
        }
    }

    return Ok(());
}

#[derive(Clone, Debug)]
struct Instruction {
    operation: String,
    argument: isize,
}

fn parse_input() -> io::Result<Vec<Instruction>> {
    let mut results: Vec<Instruction> = Vec::new();

    let file = File::open("input")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        results.push(parse_line(line.unwrap()));
    }

    return Ok(results);
}

fn parse_line(line: String) -> Instruction {
    let split = line.split(" ").collect::<Vec<&str>>();
    return Instruction {
        operation: split[0].to_string(),
        argument: split[1].parse::<isize>().unwrap(),
    };
}

fn run(instructions: &Vec<Instruction>) -> (isize, usize) {
    let mut accumulator: isize = 0;
    let mut position: usize = 0;
    let mut positions: HashSet<usize> = HashSet::new();

    loop {
        // check if current position has been executed (start of infinite loop)
        if positions.contains(&position) {
            return (accumulator, position);
        }

        // check if current position is past the number of instructions (program terminated)
        if position >= instructions.len() {
            return (accumulator, position);
        }

        // save current position
        positions.insert(position);

        // execute instruction at current position
        let instruction = instructions.get(position).unwrap();
        match instruction.operation.as_str() {
            "nop" => {
                position = position + 1;
            }
            "acc" => {
                accumulator = accumulator + instruction.argument;
                position = position + 1;
            }
            "jmp" => {
                if instruction.argument < 0 {
                    position -= -instruction.argument as usize;
                } else {
                    position += instruction.argument as usize;
                }
            }
            &_ => (),
        }
    }
}
