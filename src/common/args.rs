use crate::common::{cell_setup::CellSetup, color::Rgb, rules::Rules};
use clap::Parser;

#[derive(Clone, Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Load a world with a cell setup
    #[clap(short, long, value_enum, default_value = "r-pentonimo")]
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
    /// 1 => very slow,
    /// 255 => instant,
    /// 0 => cells appear as if they are not dying,
    /// negative values => funny colors
    #[clap(short = 'F', long, value_parser, default_value_t = 140)]
    pub fading_speed: i32,

    /// Set the frames per second
    #[clap(short, long, value_parser, default_value_t = 10)]
    pub frames_per_second: u16,

    /// Set the birth and survival rules, defaults to conway's game of life
    /// rules. For the rule notation see:
    /// https://en.wikipedia.org/wiki/Life-like_cellular_automaton#Notation_for_rules
    #[clap(short, long, value_parser = Rules::parse, default_value = "B3/S23")]
    pub rules: Rules,

    /// Start paused so that you can edit the world
    #[clap(short, long, value_parser, default_value_t = false)]
    pub paused: bool,
}
