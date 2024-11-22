use raylib::prelude::*;

#[derive(Clone)]
pub struct Grid {
    rows: i32,
    cols: i32,
    cell_size: i32,
    cells: Vec<Vec<i32>>,
}

impl Grid {
    pub fn new(h: i32, w: i32, cs: i32) -> Self {
        return Grid {
            rows: h / cs,
            cols: w / cs,
            cell_size: cs,
            cells: vec![vec![0; h as usize / cs as usize]; w as usize / cs as usize],
        };
    }

    pub fn set_cell_value(&mut self, row: i32, col: i32, value: i32) {
        if self.within_bounds(row, col) {
            self.cells[row as usize][col as usize] = value;
        }
    }

    // Allow dead code for this function, might need it later.
    #[allow(dead_code)]
    fn quadrant(&self, row: i32, col: i32) -> i32 {
        if row < self.rows && col < self.cols {
            return 0;
        } else if row > self.rows && col < self.cols {
            return 3;
        } else if row < self.rows && col > self.cols {
            return 2;
        } else {
            return 4;
        }
    }

    pub fn draw(&self, rd: &mut RaylibDrawHandle) {
        // If the cell is dead, let it be white.
        // Else, draw it as black.
        for row in 0..self.rows {
            for col in 0..self.cols {
                let color = if self.cells[row as usize][col as usize] == 1 {
                    raylib::color::Color::BLACK
                } else {
                    // raylib::color::rcolor(50, 50, 50, 255)
                    raylib::color::Color::WHITE
                };

                rd.draw_rectangle(
                    col * self.cell_size,
                    row * self.cell_size,
                    self.cell_size - 1,
                    self.cell_size - 1,
                    color,
                );
            }
        }
    }

    pub fn get_value(&self, row: i32, col: i32) -> i32 {
        self.cells[row as usize][col as usize]
    }

    fn within_bounds(&self, row: i32, col: i32) -> bool {
        row <= self.rows && col < self.cols
    }

    pub fn get_row_count(&self) -> i32 {
        self.rows
    }

    pub fn get_col_count(&self) -> i32 {
        self.cols
    }

    pub fn fill_randomly(&mut self, rl: &RaylibHandle) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let random: i32 = rl.get_random_value(0..2);
                if random == 2 {
                    self.cells[row as usize][col as usize] = 1;
                }
            }
        }
    }
}