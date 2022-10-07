# termgol

Visit [life wiki](https://conwaylife.com/) for cell setups

## Features

- pause and drawing
- predefined cell setups
- cell setups from files, supports plain text files from the life wiki
- initial delay
- frames per second
- cellular automata rules
- 24 bit colors
- customizable live and dead cell color
- fading dead cells
- screensaver mode
- rainbow colors

## Ideas

- ~~cell setups from stdin~~ (no raw mode after EOF `:(`)
- customizable live and dead cell character
- toroidal or sheet world

## Nice Settings

```
cargo run --release -- -c acorn -r B2/S
cargo run --release -- -c termgol -r B3/S012345678
cargo run --release -- -c termgol -r B36/S125
cargo run --release -- -F 1 -f 40 -r 'B357/S245' -c termgol
cargo run --release -- -F 10 -c examples/konze.cells -r B345/S46
cargo run --release -- -F 255 -c termgol --rainbow -r B0/S01234567
cargo run --release -- -c examples/gliders.cells --rainbow
```

## Generate and load text files with figlet

```
echo hello | figlet -f banner > examples/hello.cells
cargo run --release -- -c examples/hello.cells
```
