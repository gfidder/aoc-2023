use std::cmp::max;
use std::fs::read_to_string;

#[derive(Debug)]
struct Game {
    max_blue: u32,
    max_red: u32,
    max_green: u32,
}

impl Game {
    pub fn new(max_blue: u32, max_red: u32, max_green: u32) -> Self {
        Self {
            max_blue,
            max_red,
            max_green,
        }
    }

    pub fn get_product(&self) -> u32 {
        self.max_red * self.max_blue * self.max_green
    }
}

#[derive(Debug)]
struct SetResult {
    pub blue: u32,
    pub red: u32,
    pub green: u32,
}

fn main() {
    let mut all_games: Vec<Game> = vec![];
    let game_input = read_lines();

    let mut sum: u32 = 0;

    for input in &game_input {
        all_games.push(game_process(input));
    }

    for game in all_games {
        sum += game.get_product();
    }

    // for game in possible_games {
    // }

    println!("{}", sum);
}

fn game_process(input: &String) -> Game {
    let mut max_red: u32 = 0;
    let mut max_blue: u32 = 0;
    let mut max_green: u32 = 0;
    let mut set_results: Vec<SetResult> = vec![];
    let x: Vec<&str> = input.split_terminator(&[':', ';'][..]).collect();

    for i in 1..x.len() {
        set_results.push(calculate_set(x[i]));
    }

    for res in set_results {
        max_red = max(max_red, res.red);
        max_blue = max(max_blue, res.blue);
        max_green = max(max_green, res.green);
    }

    let game = Game::new(max_blue, max_red, max_green);

    println!("{:?}", game);

    game
}

fn calculate_set(input: &str) -> SetResult {
    let mut green: u32 = 0;
    let mut red: u32 = 0;
    let mut blue: u32 = 0;
    let x: Vec<&str> = input.split_terminator(',').collect();

    for color in x {
        let mut colors_iter = color.split_whitespace();

        let number_str = colors_iter.next().unwrap();
        let color = colors_iter.next().unwrap();
        let number = number_str.parse::<u32>().unwrap();

        match color {
            "red" => red = number,
            "green" => green = number,
            "blue" => blue = number,
            _ => {
                panic!()
            }
        }
    }

    SetResult { blue, red, green }
}

fn read_lines() -> Vec<String> {
    read_to_string("day2-2/input_file.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
