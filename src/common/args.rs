use crate::{
    common::color::Rgb,
    state::{cell_setup::CellSetup, rules::Rules},
};
use clap::Parser;

#[derive(Clone, Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Load a world with a predefined cell setup.
    /// Recognized values:
    ///   acorn
    ///   blank
    ///   r-pentonimo
    ///   termgol
    /// When the input does not match against the values above it is
    /// interpreted as a file path.
    /// -
    #[clap(short, long, value_parser = CellSetup::parse, default_value = "r-pentonimo", verbatim_doc_comment)]
    pub cell_setup: CellSetup,

    /// Set the initial background color for living cells
    #[clap(long, value_parser = Rgb::parse, default_value = "#EE8822")]
    pub color_bg_alive: Rgb,

    /// Set the initial background color for dead cells
    #[clap(long, value_parser = Rgb::parse, default_value = "#113011")]
    pub color_bg_dead: Rgb,

    /// Set the initial delay in milliseconds before the life starts evolving
    #[clap(short, long, value_parser, default_value_t = 1000)]
    pub delay: u64,

    /// Set the fading speed for dead cells:
    ///     1 => very slow,
    ///   255 => instant,
    ///     0 => cells appear as if they are not dying,
    ///    <0 => funny colors
    /// -
    #[clap(
        short = 'F',
        long,
        value_parser,
        default_value_t = 140,
        verbatim_doc_comment
    )]
    pub fading_speed: i32,

    /// Set the frames per second
    #[clap(short, long, value_parser, default_value_t = 10)]
    pub frames_per_second: u16,

    /// Set the birth and survival rules, defaults to conway's game of life
    /// rules. For the rule notation see:
    /// https://en.wikipedia.org/wiki/Life-like_cellular_automaton#Notation_for_rules
    /// -
    #[clap(short, long, value_parser = Rules::parse, default_value = "B3/S23", verbatim_doc_comment)]
    pub rules: Rules,

    /// Start paused so that you can edit the world
    #[clap(short, long, value_parser, default_value_t = false)]
    pub paused: bool,
}
