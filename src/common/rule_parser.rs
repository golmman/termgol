use clap::{Command, Error, ErrorKind};
use regex::Regex;

pub fn parse_rules(rules: &str) -> Result<(Vec<u32>, Vec<u32>), Error> {
    // return clap::Error?????
    let mut birth_rule = Vec::new();
    let mut survival_rule = Vec::new();

    let rules_regex = Regex::new(r"^B([0-8]{0,9})/S([0-8]{0,9})$").unwrap();

    let mut cmd = Command::new("myprog");
    let err = cmd.error(ErrorKind::InvalidValue, "Some failure case");
    let captures = rules_regex.captures_iter(rules).next().ok_or(err)?;

    for birth in captures[1].chars() {
        birth_rule.push(birth as u32 - 48);
    }
    for survival in captures[2].chars() {
        survival_rule.push(survival as u32 - 48);
    }

    Ok((birth_rule, survival_rule))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_parses_the_default_life_rule() {
        let (birth_rule, survival_rule) = parse_rules("B3/S23").unwrap();
        assert_eq!(birth_rule, [3]);
        assert_eq!(survival_rule, [2, 3]);
    }

    #[test]
    fn it_parses_the_maximal_rule() {
        let (birth_rule, survival_rule) = parse_rules("B012345678/S012345678").unwrap();
        assert_eq!(birth_rule, [0, 1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(survival_rule, [0, 1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn it_fails_when_the_regex_is_not_matched() {
        let err = parse_rules("nonsense");
        assert!(err.is_err());
        assert_eq!(err.unwrap_err().kind(), ErrorKind::InvalidValue);
    }
}
