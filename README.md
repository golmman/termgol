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

## Build, Usage, Installation

```
termgol 0.1.0
github.com/golmman
Simulates game of life like cellular automatons in your terminal.
Keyboard controls:
  p           - pause time and enable drawing
  space       - toggle cell life in pause/drawing mode
  d           - show debug info
  q or ctrl-c - quit

USAGE:
    termgol [OPTIONS]

OPTIONS:
    -c, --cell-setup <CELL_SETUP>
            Load a world with a predefined cell setup.
            Recognized values:
              acorn       - a classic long living minimal configuration
              blank       - an empty world
              r-pentonimo - a classic long living configuration with only 5 living cells
              soupX       - a random square "soup" of cells, where X is the length of an edge
              termgol     - TERMGOL letters
            When the input does not match against the values above it is
            interpreted as a file path.
            - [default: r-pentonimo]

        --color-bg-alive <COLOR_BG_ALIVE>
            Set the initial background color for living cells [default: #EE8822]

        --color-bg-dead <COLOR_BG_DEAD>
            Set the initial background color for dead cells [default: #113011]

    -d, --delay <DELAY>
            Set the initial delay in milliseconds before the life starts evolving [default: 1000]

    -f, --frames-per-second <FRAMES_PER_SECOND>
            Set the frames per second [default: 10]

    -F, --fading-speed <FADING_SPEED>
            Set the fading speed for dead cells:
                1 => very slow,
              255 => instant,
                0 => cells appear as if they are not dying,
               <0 => funny colors
            - [default: 140]

    -h, --help
            Print help information

    -p, --paused
            Start paused so that you can edit the world

    -r, --rules <RULES>
            Set the birth and survival rules, defaults to conway's game of life
            rules. For the rule notation see:
            https://en.wikipedia.org/wiki/Life-like_cellular_automaton#Notation_for_rules
            - [default: B3/S23]

        --rainbow
            Start paused so that you can edit the world

    -s, --screen-saver <SCREEN_SAVER>
            Start in screen saver mode: sets up a new random soup after the specified number of
            elapsed frames

    -V, --version
            Print version information
```
