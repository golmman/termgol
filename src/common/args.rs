use clap::{Command, Error, ErrorKind, Parser};
use regex::Regex;

#[derive(Clone, Debug, Parser)]
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
    #[clap(short, long, value_parser = Rules::parse, default_value = "B3/S23")]
    pub rules: Rules,

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

#[derive(Clone, Debug, Parser)]
pub struct Rules {
    pub birth: Vec<u32>,
    pub survival: Vec<u32>,
}

impl Rules {
    pub fn parse(rules: &str) -> Result<Rules, Error> {
        let mut birth = Vec::new();
        let mut survival = Vec::new();

        let rules_regex = Regex::new(r"^B([0-8]{0,9})/S([0-8]{0,9})$").unwrap();

        let captures = rules_regex.captures_iter(rules).next().ok_or(
            Command::new("set argument to e.g. 'B3/S23' for conway's game of life rules")
                .error(ErrorKind::InvalidValue, "invalid rules pattern"),
        )?;

        for d in captures[1].chars() {
            birth.push(d as u32 - 48);
        }
        for d in captures[2].chars() {
            survival.push(d as u32 - 48);
        }

        Ok(Rules { birth, survival })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_parses_the_default_life_rule() {
        let Rules { birth, survival } = Rules::parse("B3/S23").unwrap();
        assert_eq!(birth, [3]);
        assert_eq!(survival, [2, 3]);
    }

    #[test]
    fn it_parses_the_maximal_rule() {
        let Rules { birth, survival } = Rules::parse("B012345678/S012345678").unwrap();
        assert_eq!(birth, [0, 1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(survival, [0, 1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn it_fails_when_the_regex_is_not_matched() {
        let err = Rules::parse("nonsense");
        assert!(err.is_err());
        assert_eq!(err.unwrap_err().kind(), ErrorKind::InvalidValue);
    }
}
