use std::{
    collections::HashMap,
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

fn fishes_by_day(fishes: &Vec<i8>) -> HashMap<i8, i64> {
    let mut by_days: HashMap<i8, i64> = HashMap::new();

    for fish in fishes {
        match by_days.get(fish) {
            Some(f) => by_days.insert(*fish, f + 1),
            None => by_days.insert(*fish, 1),
        };
    }

    by_days
}

fn process_fishes(fishes: &mut HashMap<i8, i64>) {
    let mut changes: [i64; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];

    for fish in 0..=8 {
        match fishes.get(&fish) {
            Some(0) => {
                continue;
            }
            None => {
                fishes.insert(fish, 0);
            }
            Some(count) => {
                let i = fish as usize;

                changes[i] -= *count;

                if fish > 0 {
                    changes[i - 1] += *count;
                } else {
                    changes[6] += *count;
                    changes[8] += *count;
                }
            }
        }
    }

    for fish in 0..=8 {
        if let Some(count) = fishes.get(&fish) {
            fishes.insert(fish, count + changes[fish as usize]);
        }
    }
}

fn calculate_fishes_count(fishes: &Vec<i8>, days: i32) {
    let mut fishes_by_day = fishes_by_day(&fishes);

    for _ in 0..days {
        process_fishes(&mut fishes_by_day);
    }

    println!(
        "fishes count ({days} days): {}",
        fishes_by_day.values().sum::<i64>()
    );
}

fn main() -> anyhow::Result<()> {
    let fishes = read_input()?;

    calculate_fishes_count(&fishes, 80);
    calculate_fishes_count(&fishes, 256);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_map_from_array(expected: [i64; 9], map: &HashMap<i8, i64>) {
        for i in 0..expected.len() {
            assert_eq!(map.get(&(i as i8)), Some(&expected[i]));
        }
    }

    #[test]
    fn test_process_fishes() {
        let mut fishes = HashMap::new();
        fishes.insert(1, 1);
        fishes.insert(2, 1);
        fishes.insert(3, 2);
        fishes.insert(4, 1);

        process_fishes(&mut fishes);
        assert_map_from_array([1, 1, 2, 1, 0, 0, 0, 0, 0], &fishes);

        process_fishes(&mut fishes);
        assert_map_from_array([1, 2, 1, 0, 0, 0, 1, 0, 1], &fishes);

        process_fishes(&mut fishes);
        assert_map_from_array([2, 1, 0, 0, 0, 1, 1, 1, 1], &fishes);
    }
}
