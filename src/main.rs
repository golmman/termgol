use args::Args;
use clap::Parser;
use state::State;

use crate::controller::Controller;

mod args;
mod common;
mod controller;
mod renderer;
mod screen;
mod state;

fn main() {
    let args = Args::parse();
    let state = State::from(args);
    let mut controller = Controller::from(state);
    controller.run();
}
