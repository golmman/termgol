use clap::Parser;
use common::args::Args;
use controller::Controller;
use state::State;

mod common;
mod controller;
mod renderer;
mod state;

//fn main() {
//    let args = Args::parse();
//    let state = State::from(args);
//
//    Controller::from(state).run();
//}

fn main() {
    let args = Args::parse();
    let state = State::from(args);
    let controller = Controller::from(state);

    term2d::run(controller);
}
