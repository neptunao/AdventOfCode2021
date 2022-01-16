use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Error, Result};

fn get_rates_in_chars(counts: Vec<(i32, i32)>) -> (Vec<char>, Vec<char>) {
    let mut gamma_rate = Vec::new();
    let mut epsilon_rate = Vec::new();

    for (zeros_count, ones_count) in counts {
        if zeros_count > ones_count {
            gamma_rate.push('0');
            epsilon_rate.push('1');
        } else {
            gamma_rate.push('1');
            epsilon_rate.push('0');
        }
    }

    (gamma_rate, epsilon_rate)
}

fn chars_to_i16(chars: Vec<char>) -> Result<i16> {
    let mut x = 0;
    let mut shift = 0;
    for i in (0..chars.len()).rev() {
        let bit = chars[i]
            .to_digit(2)
            .ok_or(Error::msg("Char is not a binary number"))? as i16;

        x = x | (bit << shift);
        shift += 1;
    }

    Ok(x)
}

fn main() -> Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);
    let mut char_counts: Vec<(i32, i32)> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if char_counts.len() == 0 {
            char_counts.resize(line.len(), (0, 0));
        }
        for (i, ch) in line.char_indices() {
            let (zeros, ones) = char_counts[i];
            match ch {
                '0' => char_counts[i] = (zeros + 1, ones),
                '1' => char_counts[i] = (zeros, ones + 1),
                _ => (),
            }
        }
    }

    println!("arr={:?}", char_counts);

    let (gamma_rate, epsilon_rate) = get_rates_in_chars(char_counts);

    println!(
        "gamma_rate={:?} epsilon_rate={:?}",
        gamma_rate, epsilon_rate
    );

    let gamma_rate = chars_to_i16(gamma_rate)?;
    let epsilon_rate = chars_to_i16(epsilon_rate)?;
    let power_consumption = gamma_rate as i32 * epsilon_rate as i32;

    println!(
        "gamma_rate={:?} epsilon_rate={:?}",
        gamma_rate, epsilon_rate
    );

    println!("power_consumption={power_consumption}");

    Ok(())
}
