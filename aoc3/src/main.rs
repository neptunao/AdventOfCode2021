use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Error, Result};

fn get_rates_in_chars(counts: &Vec<(i32, i32)>) -> (Vec<char>, Vec<char>) {
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

fn get_char_counts(lines: &Vec<String>) -> Vec<(i32, i32)> {
    let mut char_counts: Vec<(i32, i32)> = Vec::new();

    for line in lines {
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

    char_counts
}

fn get_oxygen_ratings_chars(lines: &Vec<String>) -> Vec<char> {
    let mut oxygen_ratings = lines.clone();
    let mut i = 0;

    while oxygen_ratings.len() > 1 {
        let (zeros, ones) = get_char_counts(&oxygen_ratings)[i];
        let most_common = if zeros > ones { '0' } else { '1' };
        oxygen_ratings = oxygen_ratings
            .iter()
            .filter(|x| x.chars().nth(i) == Some(most_common))
            .map(|x| x.clone())
            .collect();
        i += 1;
    }

    oxygen_ratings[0].chars().collect()
}

fn get_co2_ratings_chars(lines: &Vec<String>) -> Vec<char> {
    let mut co2_ratings = lines.clone();
    let mut i = 0;

    while co2_ratings.len() > 1 {
        let (zeros, ones) = get_char_counts(&co2_ratings)[i];
        let least_common = if zeros > ones { '1' } else { '0' };
        co2_ratings = co2_ratings
            .iter()
            .filter(|x| x.chars().nth(i) == Some(least_common))
            .map(|x| x.clone())
            .collect();
        i += 1;
    }

    co2_ratings[0].chars().collect()
}

fn main() -> Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);

    let lines = reader.lines().collect::<Result<Vec<String>, _>>()?;
    let char_counts = get_char_counts(&lines);

    let (gamma_rate, epsilon_rate) = get_rates_in_chars(&char_counts);

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

    let oxygen_rating = get_oxygen_ratings_chars(&lines);
    let co2_rating = get_co2_ratings_chars(&lines);
    println!(
        "oxygen_rating={:?} co2_rating={:?}",
        oxygen_rating, co2_rating
    );

    let oxygen_rating = chars_to_i16(oxygen_rating)?;
    let co2_rating = chars_to_i16(co2_rating)?;
    let life_support_rating = oxygen_rating as i32 * co2_rating as i32;

    println!("oxygen_rating={oxygen_rating} co2_rating={co2_rating} life_support_rating={life_support_rating}");

    Ok(())
}
