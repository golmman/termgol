use clap::{builder::ValueParser, Error, Parser};

use super::{rule_parser::parse_rules, rules::Rules};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Load a world with a cell setup
    #[clap(short, long, value_enum, default_value = "r-pentonimo")]
    pub cell_setup: CellSetup,

    /// Set the initial delay in milliseconds before the life starts evolving
    #[clap(short, long, value_parser, default_value_t = 1000)]
    pub delay: u64,

    /// Set the frames per second
    #[clap(short, long, value_parser, default_value_t = 10)]
    pub frames_per_second: u16,

    /// Set the birth and survival rules, defaults to conway's game of life
    /// rules. For the rule notation see:
    /// https://en.wikipedia.org/wiki/Life-like_cellular_automaton#Notation_for_rules
    #[clap(short, long, value_parser = ttt, default_value = "B3/S23")]
    pub rules: u64,

    #[clap(short, long, value_parser = parse_rules, default_value = "b3/s23")]
    pub rules2: (Vec<u32>, Vec<u32>),

    #[clap(short, long, value_parser = Rules::parse, default_value = "b3/s23")]
    pub rules3: Rules,

    /// Start paused so that you can edit the world
    #[clap(short, long, value_parser, default_value_t = false)]
    pub paused: bool,
}

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub enum CellSetup {
    Acorn,
    Blank,
    RPentonimo,
    Termgol,
}

fn ttt(s: &str) -> Result<u64, Error> {
    Ok(1)
}
