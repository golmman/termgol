// usage of termions 'Color' module is awkward
// see https://gitlab.redox-os.org/redox-os/termion/-/issues/123

// see
// https://stackoverflow.com/questions/4842424/list-of-ansi-color-escape-sequences
// https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797

use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub bg: u8,
    pub fg: u8,
}

impl Color {
    pub const RESET: &'static str = "\x1b[0m";

    pub const fn new(bg_color: u8, fg_color: u8) -> Self {
        Self {
            bg: bg_color,
            fg: fg_color,
        }
    }

    pub const fn null() -> Self {
        Self { bg: 0, fg: 0 }
    }

    pub const fn text() -> Self {
        Self { bg: 0, fg: 7 }
    }
}

impl From<&Color> for String {
    fn from(color: &Color) -> Self {
        let fg_color = color.fg;
        let bg_color = color.bg;
        format!("\x1b[38;5;{fg_color};48;5;{bg_color}m")
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}
