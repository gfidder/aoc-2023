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
    let mut copy: String = input.clone();
    let mut continue_parsing = true;
    let mut numbers: Vec<u32> = vec![];

    while continue_parsing {
        let (num, string) = get_first_number_and_split_string(&copy);

        if let Some(found_num) = num {
            numbers.push(found_num);
        }

        if let Some(input) = string {
            copy = input;
        } else {
            continue_parsing = false;
        }
    }

    let first = numbers[0];
    let last = numbers[numbers.len() - 1];

    let return_value = (first * 10) + last;

    return_value
}

fn get_first_number_and_split_string(input: &String) -> (Option<u32>, Option<String>) {
    let numbers = vec![
        "one".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
        "five".to_string(),
        "six".to_string(),
        "seven".to_string(),
        "eight".to_string(),
        "nine".to_string(),
    ];
    let re = Regex::new("^[0-9]").unwrap();
    let mut ret_number: Option<u32> = None;
    let mut ret_string: Option<String> = None;
    let mut found = false;
    let mut process_string = input.clone();

    while !found {
        let dates: Vec<u32> = re
            .find_iter(&process_string.as_str())
            .map(|m| {
                let x = m.as_str().to_string();
                let new_int = x.parse::<u32>().unwrap();

                new_int
            })
            .collect();

        if !dates.is_empty() {
            found = true;
            ret_number = Some(dates[0]);
            let (_, end) = input.split_at(1);

            if end.to_string() == "" {
                ret_string = None;
            } else {
                ret_string = Some(end.to_string());
            }
        } else {
            for number in &numbers {
                if process_string.starts_with(number) {
                    found = true;

                    let index = process_string.find(number).unwrap();

                    ret_number = Some(get_uint_from_string(number));
                    let split_index = index + number.len();

                    let (_, end) = process_string.split_at(split_index);

                    if end.to_string() == "" {
                        ret_string = None
                    } else {
                        ret_string = Some(end.to_string());
                    }

                    break;
                }
            }
        }

        if !found {
            if process_string.is_empty() {
                break;
            } else {
                let (_, end) = process_string.split_at(1);
                process_string = end.to_string();
            }
        }
    }

    // println!("{:?}, {:?}", ret_number, ret_string);

    (ret_number, ret_string)
}

fn get_uint_from_string(input: &String) -> u32 {
    let ret_val: u32 = match input.as_str() {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => panic!("Input string did not match a number!"),
    };

    ret_val
}

fn read_lines() -> Vec<String> {
    read_to_string("input_file.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
