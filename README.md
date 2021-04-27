# Arena Gungeon
Open Source PVP arena 2d game cross-platform written in [Rust](https://www.rust-lang.org/).  

## TODO Features

* Single Player
  - [ ] Choose class
  - [ ] Spawn enemies
  - [ ] Add Levels

* Multi Player
  - [ ] Connect to Server
  - [ ] Choose class
  - [ ] Spawn other Players

## Commands

```bash

# run
cargo run

# build
cargo build

# build release
cargo build --release

```

## Dependecies
Arena Gungeon is made with [Macroquad](https://github.com/not-fl3/macroquad), to run need install some libs

```bash
# Linux system dependencies
apt install libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev
```

## Buildin to WebAssembly

Add build target for webassembly

```bash
rustup target add wasm32-unknown-unknown
```

There is a script in utils/wasm/build.sh.

```bash
./build.sh
```

This script compiles the program with cargo, takes the generated arena_gungeon.wasm file, and the files in utils/wasm/ and moves them into a new folder called static/. To run it in the browser I'm, using [basic-http-server](https://crates.io/crates/basic-http-server).

```bash
cargo install basic-http-server
```

start the server by using the correct path

```bash
basic-http-server . # starts server based on current directory
basic-http-server static # start server in the folder /static
```
