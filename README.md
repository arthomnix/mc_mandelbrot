# mc_mandelbrot
Mandelbrot set generator for vanilla Minecraft in Rust using [wasmcraft2](https://github.com/SuperTails/wasmcraft2) and my [binding crate](https://crates.io/crates/mcinterface).

Usage:
```sh
cargo build --release
wasm-opt -O2 target/wasm32-unknown-unknown/release/mc_mandelbrot.wasm -o target/wasm32_unknown_unknown/release/mc_mandelbrot.opt.wasm
cd /path/to/wasmcraft2
cargo run --release -- /path/to/mc_mandelbrot/target/wasm32-unknown-unknown/release/mc_mandelbrot.opt.wasm -O0 -o ./datapack
cp -r datapack /path/to/.minecraft/saves/<world name>/datapacks/
```
Enabling wasmcraft2's `-O1` will probably cause wasmcraft2 to panic. 

The maximum iterations per block, width, and position can be changed with the `MAX_ITERATIONS`, `WIDTH`, `POS_X`, `POS_Y` and `POS_Z` environment variables at compile time.

In Minecraft:
```
/reload
/gamerule maxCommandChainLength 100000000
/function wasmrunner:init
/function wasmrunner:_start
```
The gamerule only needs to be set once per world. `/reload` is only necessary if you copied/changed the datapack after loading the world.
The commands should be run close to (0, 0) as wasmcraft2 places a large amount of jukeboxes near (0, 0) to use as memory.
