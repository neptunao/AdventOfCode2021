use anyhow::anyhow;
use std::{
    clone,
    fs::File,
    io::{BufRead, BufReader},
    str::{Chars, FromStr},
};

#[derive(Debug)]
struct Lines {
    max_x: i32,
    max_y: i32,
    lines: Vec<Line>,
}

#[derive(Debug)]
struct Line {
    start_x: i32,
    start_y: i32,
    end_x: i32,
    end_y: i32,
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = &mut s.chars();
        let start_x_str = scan_to(chars, ',').ok_or(anyhow!("can't parse start_x in {s}"))?;
        let start_y_str = scan_to(chars, ' ').ok_or(anyhow!("can't parse start_y in {s}"))?;
        scan_to(chars, ' ');
        let end_x_str = scan_to(chars, ',').ok_or(anyhow!("can't parse end_x in {s}"))?;
        let end_y_str = scan_to_end(chars);

        Ok(Line {
            start_x: i32::from_str(&start_x_str)?,
            start_y: i32::from_str(&start_y_str)?,
            end_x: i32::from_str(&end_x_str)?,
            end_y: i32::from_str(&end_y_str)?,
        })
    }
}

fn scan_to_end(chars: &mut Chars) -> String {
    let mut str = "".to_string();
    loop {
        match chars.next() {
            Some(ch) => str.push(ch),
            None => break,
        }
    }

    str
}

fn scan_to(chars: &mut Chars, separator: char) -> Option<String> {
    let mut str = "".to_string();

    loop {
        match chars.next() {
            Some(ch) => {
                if ch == separator {
                    return Some(str);
                }
                str.push(ch);
            }
            None => return None,
        }
    }
}

fn read_input() -> Result<Lines, anyhow::Error> {
    let mut max_x = -1;
    let mut max_y = -1;
    let mut lines = vec![];

    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);
    for line in reader.lines() {
        let line = Line::from_str(&line?)?;

        if line.end_x > max_x {
            max_x = line.end_x;
        }

        if line.end_y > max_y {
            max_y = line.end_y;
        }

        lines.push(line);
    }

    Ok(Lines {
        max_x,
        max_y,
        lines,
    })
}

fn vec_create_with_size<T: clone::Clone>(len: usize, value: T) -> Vec<T> {
    let mut vec = vec![];
    vec.resize(len, value);
    vec
}

#[allow(dead_code)]
fn print_field(field: &Vec<Vec<i32>>) {
    for row in field {
        let mut str = String::from("");
        for col in row {
            str += &col.to_string();
        }
        println!("{str}");
    }
}

fn in_range(start: i32, end: i32, val: i32) -> bool {
    if start < end && (val < start || val > end) {
        return false;
    }

    if start > end && (val > start || val < end) {
        return false;
    }

    return true;
}

fn in_boundaries(line: &Line, x: i32, y: i32) -> bool {
    return in_range(line.start_x, line.end_x, x) && in_range(line.start_y, line.end_y, y);
}

fn populate_field(field: &mut Vec<Vec<i32>>, lines: &Vec<Line>, skip_diagonal: bool) {
    for line in lines {
        if skip_diagonal && line.start_x != line.end_x && line.start_y != line.end_y {
            continue;
        }

        let mut x = line.start_x;
        let mut y = line.start_y;

        let mut step_x = 0i32;
        if line.start_x < line.end_x {
            step_x = 1
        } else if line.start_x > line.end_x {
            step_x = -1
        }

        let mut step_y = 0i32;
        if line.start_y < line.end_y {
            step_y = 1
        } else if line.start_y > line.end_y {
            step_y = -1
        }

        while in_boundaries(&line, x, y) {
            field[y as usize][x as usize] += 1;
            x += step_x;
            y += step_y;
        }
    }
}

fn intersections_count(field: &Vec<Vec<i32>>) -> usize {
    field.iter().flatten().filter(|x| **x > 1).count()
}

fn main() -> anyhow::Result<()> {
    let lines_input = read_input()?;

    let len_y = (lines_input.max_y + 1) as usize;
    let len_x = (lines_input.max_x + 1) as usize;
    let empty_field = vec_create_with_size(len_y, vec_create_with_size(len_x, 0));
    let mut field_no_diag = &mut empty_field.clone();

    populate_field(&mut field_no_diag, &lines_input.lines, true);

    println!(
        "intersections_count (no diagonals) = {}",
        intersections_count(&field_no_diag)
    );

    let mut field_diag = &mut empty_field.clone();

    populate_field(&mut field_diag, &lines_input.lines, false);

    println!(
        "intersections_count (diagonals) = {}",
        intersections_count(&field_diag)
    );

    Ok(())
}
