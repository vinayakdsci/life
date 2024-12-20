use crate::grid::*;
use raylib::prelude::*;

pub struct Simulation<'a> {
    pub grid: Grid,
    tmp_grid: Grid,
    pub rl: &'a mut RaylibHandle,
    running: bool,
}

impl<'a> Simulation<'a> {
    pub fn new(h: i32, w: i32, cs: i32, rl: &'a mut RaylibHandle) -> Self {
        Simulation {
            grid: Grid::new(h, w, cs),
            tmp_grid: Grid::new(h, w, cs),
            rl,
            running: false,
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
                "random" => {
                    self.grid.fill_randomly(self.rl);
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
                "pentomino" => 'pentomino: {
                    break 'pentomino;
                }
                _ => {
                    self.rl.trace_log(
                        TraceLogLevel::LOG_ERROR,
                        "Unsupported preset. Availaible presets are { pentomino (default), glider, spaceship, random }."
                    );
                    self.rl
                        .trace_log(TraceLogLevel::LOG_WARNING, "Defaulting to R-Pentomino fill");
                }
            }
        }
        self.grid.fill_pentomino();
    }

    pub fn update(&mut self) {
        // Only update when the simulation is running.
        if !self.running {
            return;
        }

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
        // TODO (vinayakdsci): Try to remove this clone operation.
        self.grid = self.tmp_grid.to_owned();
    }

    pub fn clear_grid(&mut self) {
        if !self.running {
            self.grid.clear()
        }
    }

    pub fn is_running(&self) -> bool {
        self.running == true
    }

    pub fn toggle_cell(&mut self, row: i32, col: i32) {
        // Only toggle when the simulation is not running.
        if !self.running {
            let current_value = if self.grid.get_value(row, col) == 1 {
                0
            } else {
                1
            };
            self.grid.set_cell_value(row, col, current_value);
        }
    }

    pub fn start(&mut self) {
        self.running = true;
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn toggle_running_state(&mut self) {
        self.running = !self.running;
    }
}
