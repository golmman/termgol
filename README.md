# termgol

Visit [life wiki](https://conwaylife.com/) for cell setups

## Features

- pause and drawing
- predefined cell setups
- delay
- frames per second
- rules
- 24 bit colors
- customizable live and dead cell color
- fading cells
- cell setups from files

## Ideas

- cell setups from stdin
- customizable live and dead cell character
- screensaver mode
- toroidal or sheet world
- rainbow colors

## Nice Settings

```
cargo run --release -- -c acorn -r B2/S
cargo run --release -- -c termgol -r B3/S012345678
cargo run --release -- -c termgol -r B36/S125
cargo run --release -- -F 1 -f 40 -r 'B357/S245' -c termgol
```

## Generate and load text files with figlet

```
echo hello | figlet -f banner > examples/hello.gol
cargo run --release -- -c examples/hello.gol
```
