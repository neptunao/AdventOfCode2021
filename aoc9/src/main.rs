use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Context;

fn read_input() -> Result<Vec<Vec<u8>>, anyhow::Error> {
    let mut res = vec![];

    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let mut row = vec![];

        for ch in line?.chars() {
            let ch = char::to_digit(ch, 10).with_context(|| format!("Invalid char: {}", ch))?;
            row.push(ch as u8);
        }

        res.push(row);
    }

    Ok(res)
}

fn is_low_point(height_map: &[Vec<u8>], x: usize, y: usize) -> bool {
    let height = height_map[y][x];

    if x > 0 && height_map[y][x - 1] <= height {
        return false;
    }

    if x < height_map[y].len() - 1 && height_map[y][x + 1] <= height {
        return false;
    }

    if y > 0 && height_map[y - 1][x] <= height {
        return false;
    }

    if y < height_map.len() - 1 && height_map[y + 1][x] <= height {
        return false;
    }

    true
}

fn find_low_points(height_map: &[Vec<u8>]) -> Vec<u8> {
    let mut res = vec![];

    for y in 0..height_map.len() {
        for x in 0..height_map[y].len() {
            if is_low_point(height_map, x, y) {
                res.push(height_map[y][x]);
            }
        }
    }

    res
}

fn main() -> anyhow::Result<()> {
    let input = read_input()?;
    let low_points = find_low_points(&input);
    let sum_of_risk_levels = low_points.iter().map(|x| (x + 1) as u64).sum::<u64>();
    println!("Sum of risk levels: {}", sum_of_risk_levels);

    Ok(())
}
