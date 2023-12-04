use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let calibration_input = read_lines();

    let results: Vec<u32> = calibration_input.iter().map(|f| get_thing(f)).collect();

    let mut sum: u32 = 0;
    for res in results {
        sum += res;
    }

    println!("{}", sum);
}

fn get_thing(input: &String) -> u32 {
    let re = Regex::new("[0-9]").unwrap();

    let dates: Vec<u32> = re
        .find_iter(input)
        .map(|m| {
            let x = m.as_str().to_string();
            let new_int = x.parse::<u32>().unwrap();

            new_int
        })
        .collect();

    let first = dates[0];
    let last = dates[dates.len() - 1];

    let return_value = (first * 10) + last;

    return_value
}

fn read_lines() -> Vec<String> {
    read_to_string("input_file.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
