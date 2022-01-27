use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Context, Result};

type BingoBoard = [[i32; 5]; 5];

fn read_line_to_vec_i32(
    reader: &mut BufReader<File>,
    separator: char,
) -> Result<Vec<i32>, anyhow::Error> {
    let mut buf_str = String::new();
    reader.read_line(&mut buf_str)?;
    buf_str
        .trim_end()
        .split(separator)
        .filter(|s| !s.is_empty())
        .map(|s| str::parse::<i32>(s))
        .collect::<Result<Vec<i32>, _>>()
        .with_context(|| format!("Can't parse int value"))
}

fn read_board(reader: &mut BufReader<File>) -> Result<Option<BingoBoard>, anyhow::Error> {
    let mut b: BingoBoard = [[0; 5]; 5];

    for i in 0..5 {
        let row_nums = read_line_to_vec_i32(reader, ' ')?;
        if row_nums.is_empty() {
            return Ok(None);
        }

        for j in 0..row_nums.len() {
            b[i][j] = row_nums[j];
        }
    }

    Ok(Some(b))
}

fn read_game_input(input_path: &str) -> Result<(Vec<i32>, Vec<BingoBoard>), anyhow::Error> {
    let f = File::open(input_path)?;
    let mut reader = BufReader::new(f);
    let draw_numbers = read_line_to_vec_i32(&mut reader, ',')?;

    reader.read_line(&mut String::new())?; //skip line

    let mut boards: Vec<BingoBoard> = Vec::new();

    loop {
        let board = read_board(&mut reader)?;
        match board {
            Some(b) => boards.push(b),
            None => break,
        }
        reader.read_line(&mut String::new())?; //skip line
    }

    Ok((draw_numbers, boards))
}

fn main() -> Result<()> {
    let (draw_numbers, boards) = read_game_input("input.txt")?;
    Ok(())
}
