use std::fmt::Display;

use clap::Parser;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum DemoWorld {
    RPentonimo,
    Blank,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Load a demo world
    #[clap(short, long, value_enum, default_value = "r-pentonimo")]
    pub demo_world: DemoWorld,

    /// Set the frames per second
    #[clap(short, long, value_parser, default_value_t = 10)]
    pub frames_per_second: u16,

    /// Start paused
    #[clap(short, long, value_parser, default_value_t = false)]
    pub paused: bool,
}
