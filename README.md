# Game Of Life

#### This is a Rust implementation of John Conway's Game of Life that uses Raylib to render the society.

The Game of Life is a cellular automaton whose subsequent states are determined by its initial state, thus requiring no manual input (0-player).
More information can be found [here](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life).

##### Usage

To build and run:
```bash
$ cargo run --release -- <flags>
```

The following flags are supported:
- `--preset`
  - Defines the fill pattern of the GUI window.
  - Can be one of `pentomino` (R-Pentomino), `spaceship` (Light Spaceship), `random`, or `glider`.
- `--height`
  - Defines the height of the GUI window.
- `--width`
  - Defines the width of the GUI window.
- `--cellsize`
  - Defines the size of one cell.
  - Requires height and width flags.
- `--fps`
  - Defines the frame rate/speed of the animation.
  - Must be 32-bits wide unsigned int.
 
Example:
```bash
$ cargo run --release -- --preset glider --height 800 --width 1200 --cellsize 4 --fps 20
```

##### Interactivity
The simulation allows the player to interact with it:

- Pressing \<SPACE\> will toggle the running state of the simulation.
- Pressing \<F\> will make the simulation faster (increase fps) up till 30 fps.
- Pressing \<S\> will make the simulation slower (decrease fps) down till 5 fps.
- Pressing \<TAB\> when the simulation is stopped will clear the grid and fill it with R-Pentomino.
- Pressing \<C\> will clear the grid when the simulation is stopped.
- \<LEFT_CLICK\> will toggle the cell when the simulation is stopped. If the cell is live, it will die, and be reborn if it is dead.

By default, the simulation will start with the R-Pentomino pattern, but will not be running &ndash; to begin the simulation, press \<SPACE\>.

##### TODO
I would like to still implement a number of things:
1. Optimizations: Currently the state update logic uses a temporary grid that is copied back into the original grid. This should be changed.
2. Aesthetics: Make the GUI more pleasing, possibly by implementing better color combinations.
3. ~~Interaction: Make the GUI more interactive. Allow the player to toggle cells by mouse-clicks, and to stop and start the animation at will.~~
