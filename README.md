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

##### TODO
I would like to still implement a number of things:
1. Optimizations: Currently the state update logic uses a temporary grid that is copied back into the original grid. This should be changed.
2. Interaction: Make the GUI more interactive. Allow the player to toggle cells by mouse-clicks, and to stop and start the animation at will.
3. Aesthetics: Make the GUI more pleasing, possibly by implementing better color combinations.
