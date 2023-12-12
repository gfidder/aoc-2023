use std::fs::read_to_string;

mod matrix;

use matrix::{Matrix, X_SIZE, Y_SIZE};

fn main() {
    let lines = read_lines();

    let calc = Calculation::new(Matrix::new(lines));

    // m.print();

    println!("Part number is: {}", calc.get_part_number());
}

fn read_lines() -> Vec<String> {
    read_to_string("day3-2/input_file.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

struct Calculation {
    matrix: Matrix,
}

impl Calculation {
    pub fn new(matrix: Matrix) -> Self {
        Self { matrix }
    }

    pub fn get_part_number(&self) -> u32 {
        let mut sum: u32 = 0;

        let gear_locations = self.find_all_gears();
        let numbers = self.find_numbers();

        for gear_location in gear_locations {
            let mut neighbor_numbers = Vec::<NumberGrid>::new();
            for num in &numbers {
                if num.is_neighbor(gear_location) {
                    neighbor_numbers.push(num.clone());
                }
            }

            if neighbor_numbers.len() == 2 {
                let product = neighbor_numbers[0].get_value() * neighbor_numbers[1].get_value();
                sum += product;
            }
        }

        sum
    }

    fn find_numbers(&self) -> Vec<NumberGrid> {
        let mut numbers: Vec<NumberGrid> = vec![];

        for i in 0..Y_SIZE {
            let mut none_return = false;
            let mut start_pos: usize = 0;

            while !none_return {
                if let Some(found_num) = self.matrix.get_num_coords(i, start_pos) {
                    let (number_begin, number_end) = found_num;

                    let mut number_vec: Vec<char> = vec![];

                    for j in number_begin.1..=number_end.1 {
                        number_vec.push(self.matrix.get(i, j));
                    }

                    let number_str: String = number_vec.iter().collect();
                    let number = number_str.parse::<u32>().unwrap();

                    numbers.push(NumberGrid::new(number_begin.1, number_end.1, i, number));

                    start_pos = number_end.1 + 1;
                } else {
                    none_return = true;
                }
            }
        }

        numbers
    }

    fn find_all_gears(&self) -> Vec<(usize, usize)> {
        let mut gear_locations = Vec::<(usize, usize)>::new();

        for i in 0..X_SIZE {
            for j in 0..Y_SIZE {
                let z = self.matrix.get(i, j);
                if z == '*' {
                    gear_locations.push((i, j));
                }
            }
        }

        gear_locations
    }
}

#[derive(Debug, Clone)]
struct NumberGrid {
    left: usize,
    right: usize,
    y_loc: usize,
    value: u32,
}

impl NumberGrid {
    pub fn new(left: usize, right: usize, y_loc: usize, value: u32) -> Self {
        Self {
            left,
            right,
            y_loc,
            value,
        }
    }

    pub fn is_neighbor(&self, location: (usize, usize)) -> bool {
        let top_left = get_top_left_boundary((self.y_loc, self.left));
        let bottom_right = get_bottom_right_boundary((self.y_loc, self.right));

        for i in top_left.0..=bottom_right.0 {
            for j in top_left.1..=bottom_right.1 {
                let loc = (i, j);
                if loc == location {
                    return true;
                }
            }
        }

        false
    }

    pub fn get_value(&self) -> u32 {
        self.value
    }
}

fn get_top_left_boundary((x, y): (usize, usize)) -> (usize, usize) {
    let mut ret_x = x;
    let mut ret_y = y;

    if ret_x > 0 {
        ret_x -= 1;
    }

    if ret_y > 0 {
        ret_y -= 1;
    }

    (ret_x, ret_y)
}

fn get_bottom_right_boundary((x, y): (usize, usize)) -> (usize, usize) {
    let mut ret_x = x;
    let mut ret_y = y;

    if ret_x < X_SIZE - 1 {
        ret_x += 1;
    }

    if ret_y < Y_SIZE - 1 {
        ret_y += 1;
    }

    (ret_x, ret_y)
}
