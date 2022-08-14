use anyhow::anyhow;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::{Chars, FromStr},
};

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

fn read_input() -> Result<Vec<Line>, anyhow::Error> {
    let mut res = vec![];
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);
    for line in reader.lines() {
        res.push(Line::from_str(&line?)?);
    }

    Ok(res)
}

fn main() -> anyhow::Result<()> {
    let lines = read_input()?;
    println!("{:?}", lines);

    Ok(())
}
