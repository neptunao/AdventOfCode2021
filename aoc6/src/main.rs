use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

fn read_input() -> Result<Vec<i8>, anyhow::Error> {
    let f = File::open("input.txt")?;
    let mut reader = BufReader::new(f);
    let mut text: String = String::new();
    reader.read_line(&mut text)?;

    let res: Result<Vec<i8>, _> = text.split(',').map(|s| i8::from_str(s)).collect();
    Ok(res?)
}

fn process_fishes(fishes: &mut Vec<i8>) {
    let new_fishes = &mut vec![];
    for fish in fishes.iter_mut() {
        if *fish > 0 {
            *fish -= 1;
        } else {
            *fish = 6;
            new_fishes.push(8);
        }
    }

    // much effective to join linked lists but input is so tiny so...
    fishes.append(new_fishes);
}

fn main() -> anyhow::Result<()> {
    let mut fishes = read_input()?;
    for _ in 0..80 {
        process_fishes(&mut fishes);
    }
    println!("fishes count: {}", fishes.len());
    Ok(())
}
