use clap::Parser;
use common::args::Args;
use controller::Controller;
use state::State;

mod common;
mod controller;
mod renderer;
mod screen;
mod state;

fn main() {
    let args = Args::parse();
    let state = State::from(args);

    Controller::from(state).run();
}
