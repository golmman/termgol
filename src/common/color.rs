// usage of termions 'Color' module is awkward
// see https://gitlab.redox-os.org/redox-os/termion/-/issues/123

// see
// https://stackoverflow.com/questions/4842424/list-of-ansi-color-escape-sequences
// https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797

use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub bg: Rgb,
    pub fg: Rgb,
}

impl Color {
    pub const RESET: &'static str = "\x1b[0m";

    pub const fn null() -> Self {
        Self {
            bg: Rgb { r: 0, g: 0, b: 0 },
            fg: Rgb { r: 0, g: 0, b: 0 },
        }
    }

    pub const fn text() -> Self {
        Self {
            bg: Rgb { r: 0, g: 0, b: 0 },
            fg: Rgb {
                r: 200,
                g: 200,
                b: 200,
            },
        }
    }
}

impl From<&Color> for String {
    fn from(color: &Color) -> Self {
        let Rgb {
            r: fg_r,
            g: fg_g,
            b: fg_b,
        } = color.fg;
        let Rgb {
            r: bg_r,
            g: bg_g,
            b: bg_b,
        } = color.bg;
        format!("\x1b[38;2;{fg_r};{fg_g};{fg_b};48;2;{bg_r};{bg_b};{bg_g}m")
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}
