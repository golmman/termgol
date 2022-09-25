use regex::Regex;

pub fn parse_rules(rules: &str) -> (Vec<u32>, Vec<u32>) {
    let mut birth_rule = Vec::new();
    let mut survival_rule = Vec::new();

    let rules_regex = Regex::new(r"^B([0-8]{0,9})/S([0-8]{0,9})$").unwrap();
    assert!(rules_regex.is_match(rules));

    let captures = rules_regex.captures_iter(rules).next().unwrap();

    for birth in captures[1].chars() {
        birth_rule.push(birth as u32 - 48);
    }
    for survival in captures[2].chars() {
        survival_rule.push(survival as u32 - 48);
    }

    (birth_rule, survival_rule)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_parses_the_default_life_rule() {
        let (birth_rule, survival_rule) = parse_rules("B3/S23");
        assert_eq!(birth_rule, [3]);
        assert_eq!(survival_rule, [2, 3]);
    }

    #[test]
    fn it_parses_the_maximal_rule() {
        let (birth_rule, survival_rule) = parse_rules("B012345678/S012345678");
        assert_eq!(birth_rule, [0, 1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(survival_rule, [0, 1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    #[should_panic]
    fn it_fails_when_the_regex_is_not_matched() {
        parse_rules("nonsense");
    }
}
