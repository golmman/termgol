use clap::Command;
use clap::Error;
use clap::ErrorKind;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RgbaParser {}

impl RgbaParser {
    pub fn parse(hex_code: &str) -> Result<term2d::model::rgba::Rgba, Error> {
        let error_map = || {
            Command::new("set argument to e.g. '#ff0000' for a bright red color")
                .error(ErrorKind::InvalidValue, "invalid rgb hex code")
        };

        if hex_code.len() != 7 {
            return Err(error_map());
        }

        let r = u8::from_str_radix(&hex_code[1..3], 16).map_err(|_| error_map())?;
        let g = u8::from_str_radix(&hex_code[3..5], 16).map_err(|_| error_map())?;
        let b = u8::from_str_radix(&hex_code[5..7], 16).map_err(|_| error_map())?;

        Ok(term2d::model::rgba::Rgba { r, g, b, a: 255 })
    }
}

#[cfg(test)]
mod test {
    use term2d::model::rgba::Rgba;

    use super::*;

    #[test]
    fn it_parses_bright_red_color_from_hex_code() {
        let Rgba { r, g, b, a } = RgbaParser::parse("#ff0000").unwrap();
        assert_eq!(r, 255);
        assert_eq!(g, 0);
        assert_eq!(b, 0);
        assert_eq!(a, 255);
    }

    #[test]
    fn it_parses_test_color_from_hex_code() {
        let Rgba { r, g, b, a } = RgbaParser::parse("#010203").unwrap();
        assert_eq!(r, 1);
        assert_eq!(g, 2);
        assert_eq!(b, 3);
        assert_eq!(a, 255);
    }

    #[test]
    fn it_fails_when_the_rgb_hex_code_is_invalid() {
        let err = RgbaParser::parse("nonsense");
        assert!(err.is_err());
        assert_eq!(err.unwrap_err().kind(), ErrorKind::InvalidValue);
    }
}
