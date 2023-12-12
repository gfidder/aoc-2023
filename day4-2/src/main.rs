use std::{borrow::BorrowMut, cell::RefCell, collections::HashMap, fs::read_to_string, rc::Rc};

#[derive(Debug, Clone)]
struct Game {
    winning_numbers: Vec<u32>,
    played_numbers: Vec<u32>,
    matches: u32,
    game_number: u32,
    copies: u32,
}

impl Game {
    pub fn new(winning_numbers: Vec<u32>, played_numbers: Vec<u32>, game_number: u32) -> Self {
        Self {
            winning_numbers,
            played_numbers,
            matches: 0,
            game_number,
            copies: 1,
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

    pub fn get_game_copies(&self) -> Vec<u32> {
        let mut game_copies = Vec::new();
        let base_game = self.game_number;

        for i in 1..=self.matches {
            let copied_game = base_game + i;
            game_copies.push(copied_game);
        }

        game_copies
    }

    pub fn add_copy(&mut self) {
        self.copies += 1;
    }

    pub fn get_num_copies(&self) -> u32 {
        self.copies
    }
}

fn main() {
    let mut all_games = Vec::new();
    let game_inputs = read_lines();
    let mut sum: u32 = 0;
    let mut i = 0;
    let mut game_copies = HashMap::new();

    for input in &game_inputs {
        let game_number = i + 1;
        let mut game = process_game(input, game_number);

        game.calculate_matches();

        all_games.push(game);

        i += 1;
    }

    for i in 1..=all_games.len() {
        game_copies.insert(i, 1usize);
    }

    for game in all_games.iter() {
        let things = game.get_game_copies();

        for i in things {
            match game_copies.get_mut(&(i as usize)) {
                Some(new_game) => {
                    *new_game += 1;
                }
                None => {
                    panic!("Found game that doesn't exist: {}!", i);
                }
            }
        }
    }

    println!("{:?}", game_copies);

    for game in all_games.iter() {
        println!("{:?}", game);
        sum += game.get_num_copies();
    }

    println!("{}", sum);
}

fn process_game(input: &String, game_number: u32) -> Game {
    let x: Vec<&str> = input.split_terminator(&[':', '|'][..]).collect();

    let winning_numbers = get_numbers(x[1]);
    let played_numbers = get_numbers(x[2]);

    Game::new(winning_numbers, played_numbers, game_number)
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
