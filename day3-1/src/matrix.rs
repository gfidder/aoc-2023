pub const X_SIZE: usize = 140;
pub const Y_SIZE: usize = 140;

#[derive(Debug)]
pub struct Matrix {
    rows: [[char; X_SIZE]; Y_SIZE],
}

impl Matrix {
    pub fn new(file_input: Vec<String>) -> Self {
        let y_size = file_input.len();
        let x_size = file_input[0].len();

        println!("Y size: {}\nX size: {}", y_size, x_size);

        let mut rows = [[' '; X_SIZE]; Y_SIZE];

        for i in 0..file_input.len() {
            let line = &file_input[i];
            let line_length = line.len();

            for j in 0..line_length {
                rows[i][j] = line.as_bytes()[j] as char;
            }
        }

        Self { rows }
    }

    // pub fn print(&self) {
    //     for i in 0..Y_SIZE {
    //         for j in 0..X_SIZE {
    //             print!("{}", self.rows[i][j]);
    //         }

    //         print!("\n");
    //     }
    // }

    pub fn get(&self, x: usize, y: usize) -> char {
        self.rows[x][y]
    }

    pub fn get_num_coords(
        &self,
        row: usize,
        start_location: usize,
    ) -> Option<((usize, usize), (usize, usize))> {
        let mut found_begin = false;
        let mut start_x: usize = 0;
        let mut end_x: usize = 0;
        let mut set_end_x = false;

        for j in start_location..X_SIZE {
            if self.rows[row][j].is_numeric() {
                if found_begin {
                    // do nothing
                } else {
                    found_begin = true;
                    start_x = j
                }
            } else {
                if found_begin {
                    end_x = j - 1;
                    set_end_x = true;
                    break;
                } else {
                    // do nothing
                }
            }
        }

        if !found_begin {
            return None;
        }

        if !set_end_x {
            end_x = X_SIZE - 1;
        }

        Some(((row, start_x), (row, end_x)))
    }
}
