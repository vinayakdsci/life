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
        simulation.update();
        simulation.draw(&thread);
    }
}
