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
        let mut part_numbers: Vec<u32> = vec![];
        let mut sum: u32 = 0;

        for i in 0..Y_SIZE {
            let mut none_return = false;
            let mut start_pos: usize = 0;

            while !none_return {
                if let Some(found_num) = self.matrix.get_num_coords(i, start_pos) {
                    let (number_begin, number_end) = found_num;

                    let top_left = self.get_top_left_boundary(number_begin);
                    let bottom_right = self.get_bottom_right_boundary(number_end);

                    if self.find_gear(top_left, bottom_right) {
                        let mut number_vec: Vec<char> = vec![];

                        for j in number_begin.1..=number_end.1 {
                            number_vec.push(self.matrix.get(i, j));
                        }

                        let number_str: String = number_vec.iter().collect();
                        let number = number_str.parse::<u32>().unwrap();

                        part_numbers.push(number);
                    }

                    start_pos = number_end.1 + 1;
                } else {
                    none_return = true;
                }
            }
        }

        for part in part_numbers {
            sum += part;
        }

        sum
    }

    fn find_gear(&self, top_left: (usize, usize), bottom_right: (usize, usize)) -> bool {
        let symbol_chars = vec!['*'];

        for i in top_left.0..=bottom_right.0 {
            for j in top_left.1..=bottom_right.1 {
                let c = self.matrix.get(i, j);
                if symbol_chars.contains(&c) {
                    // println!("Found a character");
                    return true;
                }
            }
        }

        false
    }

    fn get_top_left_boundary(&self, (x, y): (usize, usize)) -> (usize, usize) {
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

    fn get_bottom_right_boundary(&self, (x, y): (usize, usize)) -> (usize, usize) {
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
}
