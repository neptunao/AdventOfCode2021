use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_increase_count(depths: &Vec<i32>) -> i32 {
    let mut increase_count = 0;
    for i in 1..depths.len() {
        if depths[i] > depths[i - 1] {
            increase_count += 1;
        }
    }

    increase_count
}

fn get_increase_count_windowed(depths: &Vec<i32>) -> i32 {
    let mut window = depths[0] + depths[1] + depths[2];
    let mut increase_count = 0;
    for i in 3..depths.len() {
        let new_window = window - depths[i - 3] + depths[i];
        if new_window > window {
            increase_count += 1;
        }
        window = new_window;
    }

    increase_count
}

fn main() {
    let f = File::open("input.txt").expect("Can't open file");
    let reader = BufReader::new(f);
    let depths = reader
        .lines()
        .collect::<Result<Vec<String>, _>>()
        .expect("Error reading from file")
        .iter()
        .map(|s| str::parse::<i32>(s.as_str()))
        .collect::<Result<Vec<i32>, _>>()
        .expect("Expect all strings to be numbers");

    let increase_count = get_increase_count(&depths);
    let increase_count_windowed = get_increase_count_windowed(&depths);

    println!("Number of increases: {increase_count}");
    println!("Number of increases (windows): {increase_count_windowed}");
}
