use crate::{
    common::color::Rgb,
    state::{cell_setup::CellSetup, rules::Rules},
};
use clap::Parser;

/// Simulates game of life like cellular automatons in your terminal.
/// Keyboard controls:
///   p           - pause time and enable drawing
///   space       - toggle cell life in pause/drawing mode
///   d           - show debug info
///   q or ctrl-c - quit
#[derive(Clone, Debug, Parser)]
#[clap(author, version, verbatim_doc_comment)]
pub struct Args {
    /// Load a world with a predefined cell setup.
    /// Recognized values:
    ///   acorn       - a classic long living minimal configuration
    ///   blank       - an empty world
    ///   r-pentonimo - a classic long living configuration with only 5 living cells
    ///   soupX       - a random square "soup" of cells, where X is the length of an edge
    ///   termgol     - TERMGOL letters
    /// When the input does not match against the values above it is
    /// interpreted as a file path. In a file the characters ' ' and '.' are
    /// interpreted as dead cells, all other characters as living cells.
    /// -
    #[clap(
        short,
        long,
        value_parser = CellSetup::parse,
        default_value = "r-pentonimo",
        verbatim_doc_comment
    )]
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

    /// Start paused so that you can edit the world
    #[clap(short, long, value_parser, default_value_t = false)]
    pub paused: bool,

    /// Start paused so that you can edit the world
    #[clap(long, value_parser, default_value_t = false)]
    pub rainbow: bool,

    /// Set the birth and survival rules, defaults to conway's game of life
    /// rules. For the rule notation see:
    /// https://en.wikipedia.org/wiki/Life-like_cellular_automaton#Notation_for_rules
    /// -
    #[clap(
        short,
        long,
        value_parser = Rules::parse,
        default_value = "B3/S23",
        verbatim_doc_comment
    )]
    pub rules: Rules,

    /// Start in screen saver mode: sets up a new random soup after the specified
    /// number of elapsed frames.
    #[clap(short, long, value_parser)]
    pub screen_saver: Option<u32>,
}

// Ideally we would set the Args default values in its Default impl,
// unfortunatly clap does not support this at the moment
// (https://github.com/clap-rs/clap/issues/3116) so we have to do it the other
// way around.
impl Default for Args {
    fn default() -> Self {
        Self::parse_from(Vec::<String>::new().into_iter())
    }
}
