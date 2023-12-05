pub const X_SIZE: usize = 10;
pub const Y_SIZE: usize = 10;

#[derive(Debug)]
pub struct Matrix {
    rows: [[char; X_SIZE]; Y_SIZE],
}

impl Matrix {
    pub fn new(file_input: Vec<String>) -> Self {
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
                    break;
                } else {
                    // do nothing
                }
            }
        }

        if !found_begin {
            return None;
        }

        Some(((row, start_x), (row, end_x)))
    }
}
