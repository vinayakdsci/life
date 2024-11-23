pub mod grid;
pub mod simulation;
// use raylib::prelude::*;
use clap::Parser;
use simulation::Simulation;

// const WHITE: [i32; 4] = [255, 255, 255, 255];

const _FPS: u32 = 20;

const GRID_H: i32 = 800;
const GRID_W: i32 = 1200;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, action = clap::ArgAction::Set)]
    preset: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let (mut rl, thread) = raylib::init()
        .size(GRID_H, GRID_W)
        .title("Welcome to the Game Of Life")
        .build();

    rl.set_target_fps(_FPS);
    let mut simulation = Simulation::new(GRID_H, GRID_W, 2, &mut rl);
    simulation.randomize(cli.preset);
    while !simulation.rl.window_should_close() {
        simulation.update();
        simulation.draw(&thread);
    }
}
