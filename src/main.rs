use clap::Parser;
use common::args::Args;
use controller::Controller;
use state::State;
use term2d::model::ansiesc::CLEAR_ALL;
use term2d::model::ansiesc::COLOR_RESET;
use term2d::model::ansiesc::CURSOR_GOTO_1_1;
use term2d::model::ansiesc::CURSOR_SHOW;
use term2d::model::config::Config;

mod common;
mod controller;
mod renderer;
mod state;

fn main() {
    let args = Args::parse();
    let fps = args.frames_per_second;
    let state = State::from(args);
    let controller = Controller::from(state);

    term2d::run_with_config(
        controller,
        Config {
            fps,
            screen_drop_strings: vec![
                COLOR_RESET.to_string(),
                CLEAR_ALL.to_string(),
                CURSOR_GOTO_1_1.to_string(),
                CURSOR_SHOW.to_string(),
            ],
        },
    );
}
