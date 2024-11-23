use crate::grid::*;
use raylib::prelude::*;

pub struct Simulation<'a> {
    grid: Grid,
    tmp_grid: Grid,
    pub rl: &'a mut RaylibHandle,
}

impl<'a> Simulation<'a> {
    pub fn new(h: i32, w: i32, cs: i32, rl: &'a mut RaylibHandle) -> Self {
        Simulation {
            grid: Grid::new(h, w, cs),
            tmp_grid: Grid::new(h, w, cs),
            rl,
        }
    }

    pub fn set_cell_value(&mut self, row: i32, col: i32, value: i32) {
        self.grid.set_cell_value(row, col, value)
    }

    pub fn draw(&mut self, thread: &RaylibThread) {
        let mut d = self.rl.begin_drawing(thread);
        d.clear_background(Color::BLACK);
        self.grid.draw(&mut d);
    }

    pub fn count_live_neighbors(&self, row: i32, col: i32) -> i32 {
        let mut live_neighbors: i32 = 0;

        for i in row - 1..=row + 1 {
            for j in col - 1..=col + 1 {
                if i != row || j != col {
                    let n_row = (i + self.grid.get_row_count()) % self.grid.get_row_count();
                    let n_col = (j + self.grid.get_col_count()) % self.grid.get_col_count();
                    live_neighbors += self.grid.get_value(n_row, n_col);
                }
            }
        }

        live_neighbors
    }

    pub fn randomize(&mut self, preset: Option<String>) {
        if let Some(pre) = preset {
            match &pre[..] {
                "pentomino" => {
                    self.grid.fill_pentomino();
                    return;
                }
                "glider" => {
                    self.grid.fill_glider();
                    return;
                }
                "spaceship" => {
                    self.grid.fill_light_spaceship();
                    return;
                }
                _ => {
                    self.rl.trace_log(
                        TraceLogLevel::LOG_ERROR,
                        "Unsupported preset. Availaible presets are { glider, spaceship, pentomino }."
                    );
                    self.rl
                        .trace_log(TraceLogLevel::LOG_WARNING, "Defaulting to random fill.");
                }
            }
        }
        self.grid.fill_randomly(self.rl);
    }

    pub fn update(&mut self) {
        for row in 0..self.grid.get_row_count() {
            for col in 0..self.grid.get_col_count() {
                let live_neighbors: i32 = self.count_live_neighbors(row, col);
                let cell_value = self.grid.get_value(row, col);

                if cell_value == 1 {
                    if live_neighbors > 3 || live_neighbors < 2 {
                        self.tmp_grid.set_cell_value(row, col, 0);
                    } else {
                        self.tmp_grid.set_cell_value(row, col, 1);
                    }
                } else {
                    if live_neighbors == 3 {
                        self.tmp_grid.set_cell_value(row, col, 1);
                    } else {
                        self.tmp_grid.set_cell_value(row, col, 0);
                    }
                }
            }
        }
        // TODO (vinayakdsci): Try to remove this clone
        self.grid = self.tmp_grid.to_owned();
    }
}
