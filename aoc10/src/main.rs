use std::{fs::File, io::BufRead, io::BufReader};

fn read_input() -> Vec<String> {
    let f = File::open("input.txt").expect("can't open the file");
    let reader = BufReader::new(f);

    reader
        .lines()
        .collect::<Result<Vec<String>, _>>()
        .expect("can't read input from file")
}

fn get_score_corrupted(input: &Vec<String>) -> u32 {
    let mut score = 0;
    for line in input {
        let mut brackets = vec![];
        for ch in line.chars() {
            match ch {
                '(' => brackets.push('('),
                '[' => brackets.push('['),
                '{' => brackets.push('{'),
                '<' => brackets.push('<'),
                ')' => {
                    let last_opened_bracket = brackets.pop().expect("end of brackets stack");
                    if last_opened_bracket != '(' {
                        score += 3;
                    }
                }
                ']' => {
                    let last_opened_bracket = brackets.pop().expect("end of brackets stack");
                    if last_opened_bracket != '[' {
                        score += 57;
                    }
                }
                '}' => {
                    let last_opened_bracket = brackets.pop().expect("end of brackets stack");
                    if last_opened_bracket != '{' {
                        score += 1197;
                    }
                }
                '>' => {
                    let last_opened_bracket = brackets.pop().expect("end of brackets stack");
                    if last_opened_bracket != '<' {
                        score += 25137;
                    }
                }
                _ => (),
            };
        }
    }

    score
}

fn main() {
    let input = read_input();
    let score = get_score_corrupted(&input);

    println!("corrupted score={score}");
}
