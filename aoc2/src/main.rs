use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;

#[derive(Debug)]
struct SubmarinePosition {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl SubmarinePosition {
    fn new() -> SubmarinePosition {
        SubmarinePosition {
            aim: 0,
            depth: 0,
            horizontal: 0,
        }
    }

    #[allow(dead_code)]
    fn forward_simple(self: &mut SubmarinePosition, moves: i32) {
        self.horizontal += moves;
    }

    #[allow(dead_code)]
    fn down_simple(self: &mut SubmarinePosition, moves: i32) {
        self.depth += moves;
    }

    #[allow(dead_code)]
    fn up_simple(self: &mut SubmarinePosition, moves: i32) {
        self.depth -= moves;
    }

    fn down_with_aim(self: &mut SubmarinePosition, moves: i32) {
        self.aim += moves;
    }

    fn up_with_aim(self: &mut SubmarinePosition, moves: i32) {
        self.aim -= moves;
    }

    fn forward_with_aim(self: &mut SubmarinePosition, moves: i32) {
        self.horizontal += moves;
        self.depth += self.aim * moves;
    }
}

fn main() -> Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);

    let mut position = SubmarinePosition::new();

    for command in reader.lines() {
        let command = command?;
        let mut command_parts = command.split_whitespace();
        let direction = command_parts
            .next()
            .ok_or(anyhow::Error::msg("Missing direction"))?;
        let moves = command_parts
            .next()
            .map(|s| str::parse::<i32>(s))
            .ok_or(anyhow::Error::msg("Missing move count"))??;

        match direction {
            "forward" => position.forward_with_aim(moves),
            "down" => position.down_with_aim(moves),
            "up" => position.up_with_aim(moves),
            &_ => eprintln!("Invalid command"),
        }
    }

    println!("position={:?}", position);
    println!("course={}", position.horizontal * position.depth);

    Ok(())
}
