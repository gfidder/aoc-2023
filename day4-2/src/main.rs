use std::cmp::max;
use std::fs::read_to_string;

#[derive(Debug)]
struct Game {
    winning_numbers: Vec<u32>,
    played_numbers: Vec<u32>,
    matches: u32,
}

impl Game {
    pub fn new(winning_numbers: Vec<u32>, played_numbers: Vec<u32>) -> Self {
        Self {
            winning_numbers,
            played_numbers,
            matches: 0,
        }
    }

    pub fn calculate_matches(&mut self) {
        for winner in &self.winning_numbers {
            if self.played_numbers.contains(winner) {
                self.matches += 1;
            }
        }
    }

    pub fn get_game_total(&self) -> u32 {
        const BASE: u32 = 2;

        if self.matches != 0 {
            let power = self.matches - 1;

            return BASE.pow(power);
        }

        0
    }
}

fn main() {
    let mut all_games = Vec::<Game>::new();
    let game_inputs = read_lines();
    let mut sum: u32 = 0;

    for input in &game_inputs {
        all_games.push(process_game(input));
        let mut game = process_game(input);
        game.calculate_matches();

        sum += game.get_game_total();
    }

    println!("{}", sum);
}

fn process_game(input: &String) -> Game {
    let x: Vec<&str> = input.split_terminator(&[':', '|'][..]).collect();

    let winning_numbers = get_numbers(x[1]);
    let played_numbers = get_numbers(x[2]);

    Game::new(winning_numbers, played_numbers)
}

fn get_numbers(input: &str) -> Vec<u32> {
    let mut numbers: Vec<u32> = vec![];

    let mut x = input.split_whitespace();

    while let Some(number_str) = x.next() {
        let number = number_str.parse::<u32>().unwrap();

        numbers.push(number);
    }

    numbers
}

fn read_lines() -> Vec<String> {
    read_to_string("day4-2/input_file.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
