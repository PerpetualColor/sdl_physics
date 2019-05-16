# sdl_physics
A physics simulator, using OpenGL and SDL2 for display, written in Rust.

## Running
### Linux
You may need to install your OpenGL libraries and SDL2 libraries. Then just run 
` cargo run`
### Windows x64
The required libraries (freetype.lib, freetype.dll, and sdl2.dll) are included in the binary. Just run 
` cargo run`
### Windows x86_64
You will need to download or build freetype and sdl2 libraries. Build using ` cargo build` and run with `cargo run`.

## Usage
It's mostly self-explanatory. Play to play, pause to pause, grid to make a grid, sine to make a sine wave of particles, and a list of functions to play around with. The little lines all over the graph are acceleration vectors, the strengths of which are represented by the redness. Click and drag to add a new particle.

### Screenshots
Gravity Simulation
![alt text](https://raw.githubusercontent.com/PerpetualColor/sdl_physics/master/screenshots/gravity.gif "Gravity Simulation")

Butterfly-Looking Force
![alt text](https://raw.githubusercontent.com/PerpetualColor/sdl_physics/master/screenshots/butterfly.gif "Butterfly Simulation")

Harmonic Sine-Wave
![alt text](https://raw.githubusercontent.com/PerpetualColor/sdl_physics/master/screenshots/harmonic_sine.gif "Harmonic Sine Wave Simulation")
