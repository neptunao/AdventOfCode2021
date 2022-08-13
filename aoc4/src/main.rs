use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{anyhow, Context, Result};

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
        let row_numbers = read_line_to_vec_i32(reader, ' ')?;
        if row_numbers.is_empty() {
            return Ok(None);
        }

        for j in 0..row_numbers.len() {
            b[i][j] = row_numbers[j];
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

fn find_number_index(board: &BingoBoard, number: i32) -> Option<(usize, usize)> {
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            if board[i][j] == number {
                return Some((i, j));
            }
        }
    }

    None
}

fn is_bingo(board: &BingoBoard, row: usize, col: usize) -> bool {
    let is_bingo_row = board[row].iter().all(|&x| x < 0);

    if is_bingo_row {
        return true;
    }

    board.iter().map(|arr| arr[col]).all(|x| x < 0)
}

fn sum_unmarked(board: &BingoBoard) -> i32 {
    board.iter().flatten().filter(|&&x| x >= 0).sum()
}

fn find_last_winning_board(
    draw_numbers: &Vec<i32>,
    boards: &mut Vec<BingoBoard>,
) -> Option<(BingoBoard, i32)> {
    let bingo_boards: &mut HashSet<BingoBoard> = &mut HashSet::new();
    let mut last_won_board = None;

    for draw_number in draw_numbers {
        if bingo_boards.len() == boards.len() {
            return last_won_board;
        }

        for board in boards.iter_mut() {
            if bingo_boards.contains(board) {
                continue;
            }

            if let Some((x, y)) = find_number_index(&board, *draw_number) {
                board[x][y] = -1;

                if is_bingo(&board, x, y) {
                    last_won_board = Some((*board, *draw_number));
                    bingo_boards.insert(*board);
                }
            }
        }
    }

    last_won_board
}

fn find_winning_score(draw_numbers: &Vec<i32>, boards: &mut Vec<BingoBoard>) -> Option<i32> {
    for draw_number in draw_numbers {
        for board in boards.iter_mut() {
            if let Some((x, y)) = find_number_index(&board, *draw_number) {
                board[x][y] = -1;

                if is_bingo(&board, x, y) {
                    let score = sum_unmarked(&board) * draw_number;
                    return Some(score);
                }
            }
        }
    }

    None
}

#[allow(dead_code)]
fn main_part_1() -> Result<()> {
    let (draw_numbers, mut boards) = read_game_input("input.txt")?;
    let score = find_winning_score(&draw_numbers, &mut boards);
    match score {
        Some(score) => {
            println!("score={score}");
            Ok(())
        }
        None => Err(anyhow!("Bingo setup is wrong")),
    }
}

fn main() -> Result<()> {
    let (draw_numbers, mut boards) = read_game_input("input.txt")?;
    let won_board = find_last_winning_board(&draw_numbers, &mut boards);
    match won_board {
        Some((board, draw_number)) => {
            let score = sum_unmarked(&board) * draw_number;
            println!("score={score}");
            Ok(())
        }
        None => Err(anyhow!("Bingo setup is wrong")),
    }
}
