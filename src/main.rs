pub mod grid;
pub mod simulation;
// use raylib::prelude::*;
use simulation::Simulation;

// const WHITE: [i32; 4] = [255, 255, 255, 255];

const _FPS: u32 = 20;

const GRID_H: i32 = 750;
const GRID_W: i32 = 750;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(GRID_H, GRID_W)
        .title("Welcome to the Game Of Life")
        .build();

    rl.set_target_fps(_FPS);
    let mut simulation = Simulation::new(GRID_H, GRID_W, 10, &mut rl);
    simulation.randomize();
    while !simulation.rl.window_should_close() {
        simulation.update();
        simulation.draw(&thread);
    }
}
