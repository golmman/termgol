# termgol

Application which brings Conway's game of life and other cellular automata to your terminal!

![example](https://github.com/golmman/termgol/blob/master/gifs/ex1.gif "example")

## Features

- pause and draw cells
- predefined cell setups
- cell setups from files, supports plain text files from the [life wiki](https://conwaylife.com/)
- initial delay, so you can see the inital setup for some time
- configurable frames per second
- configurable cellular automata rules
- 24 bit colors
- customizable live and dead cell color
- fading dead cells
- screensaver mode, which resets the cell setup after a given number of frames
- rainbow colors

See [termgol -h](./HELP) for all options a arguments.

### Ideas (not implemented)

- ~~cell setups from stdin~~ - no raw mode after EOF `:(`
- customizable live and dead cell character
- toroidal or sheet world
- hash life

## Build, Install

### Prerequisites

In order to build and use this project you need the compiler for the [rust programming language](https://www.rust-lang.org/tools/install)
and a terminal emulator which knows how to handle 24 bit colors (e.g. iTerm2, Windows Terminal, xfce-terminal, alacritty, ...).

### Build

Clone, build with

```
git clone https://github.com/golmman/termgol.git
cd termgol
cargo build --release
```

You can then find the executable at `./target/release/termgol`.

### Run

If you just want to try it out use `cargo run --release`.

### Install

Use
```
cargo install --path .
```
to install the application to the rust/cargo bin directory (`$HOME/.cargo/bin`)
which should be in you `PATH` after you installed rust.

## Usage

Display the [help page](./HELP) with `termgol -h` for all options and command
line arguments.

Some examples:

```
termgol -F 255 -c termgol --rainbow -r B0/S01234567
termgol -c examples/gliders.cells --rainbow
termgol -c acorn -r B2/S
termgol -c termgol -r B3/S012345678
termgol -c termgol -r B36/S125
termgol -F 1 -f 40 -r 'B357/S245' -c termgol
termgol -F 10 -c examples/konze.cells -r B345/S46
```

If you just want to try it out replace `termgol` with `cargo run --release --` in the root directory, e.g.

```
cargo run --release -- -F 255 -c termgol --rainbow -r B0/S01234567
```

### Generate and load text files with figlet

```
echo hello | figlet -f banner > examples/hello.cells
cargo run --release -- -c examples/hello.cells
```
