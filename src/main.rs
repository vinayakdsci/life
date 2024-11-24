pub mod grid;
pub mod simulation;
use clap::Parser;
use simulation::Simulation;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, action = clap::ArgAction::Set)]
    preset: Option<String>,
    #[arg(short, long, action = clap::ArgAction::Set)]
    height: Option<i32>,
    #[arg(short, long, action = clap::ArgAction::Set)]
    width: Option<i32>,
    #[arg(short, long, action = clap::ArgAction::Set)]
    fps: Option<u32>,
    #[arg(short, long, action = clap::ArgAction::Set, requires = "height", requires = "width")]
    cellsize: Option<i32>,
}

fn main() {
    let cli = Cli::parse();

    let height = if let Some(h) = cli.height { h } else { 1000 };
    let width = if let Some(w) = cli.width { w } else { 1000 };
    let fps = if let Some(f) = cli.fps { f } else { 18 };
    let cellsize = if let Some(cs) = cli.cellsize { cs } else { 10 };

    let (mut rl, thread) = raylib::init()
        .size(height, width)
        .title("Welcome to the Game Of Life")
        .build();

    rl.set_target_fps(fps);
    let mut simulation = Simulation::new(height, width, cellsize, &mut rl);
    simulation.randomize(cli.preset);
    while !simulation.rl.window_should_close() {
        // Interaction logic. Allows the player to:
        // 1. Stop and start the simulation.
        // 2. Change the frame rate/speed of the simulation.
        // 3. Toggle cells from live to dead and vice versa by mouse press.
        // 4. Clear the grid,
        // 5. Fill and empty grid with the default R-Pentomino pattern.
        if simulation
            .rl
            .is_key_pressed(raylib::ffi::KeyboardKey::KEY_SPACE)
        {
            simulation.toggle_running_state();
        }

        if simulation
            .rl
            .is_key_pressed(raylib::ffi::KeyboardKey::KEY_C)
        {
            simulation.clear_grid();
        }

        if simulation
            .rl
            .is_mouse_button_pressed(raylib::ffi::MouseButton::MOUSE_BUTTON_LEFT)
        {
            let m_position = simulation.rl.get_mouse_position();
            let row: f32 = m_position.y / cellsize as f32;
            let col: f32 = m_position.x / cellsize as f32;
            simulation.toggle_cell(row as i32, col as i32);
        }

        if simulation.rl.is_key_down(raylib::ffi::KeyboardKey::KEY_TAB) {
            if simulation.is_running() {
                simulation.stop();
            }

            simulation.clear_grid();
            simulation.grid.fill_pentomino();
        }

        if simulation.rl.is_key_down(raylib::ffi::KeyboardKey::KEY_S) {
            // The lowest fps we allow is 5.
            let curr_fps = simulation.rl.get_fps();
            if curr_fps >= 10 {
                simulation.rl.set_target_fps(curr_fps - 5);
            }
        }

        if simulation.rl.is_key_down(raylib::ffi::KeyboardKey::KEY_F) {
            // The highest fps we allow is 30.
            let curr_fps = simulation.rl.get_fps();
            if curr_fps <= 25 {
                simulation.rl.set_target_fps(curr_fps + 5);
            }
        }

        // Update the simulation, and render it.
        simulation.update();
        simulation.draw(&thread);
    }
}
