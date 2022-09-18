// usage of termions 'Color' module is awkward
// see https://gitlab.redox-os.org/redox-os/termion/-/issues/123

// see
// https://stackoverflow.com/questions/4842424/list-of-ansi-color-escape-sequences
// https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797

use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub bg_color: Option<u8>,
    pub fg_color: Option<u8>,
}

impl Color {
    pub const RESET: &'static str = "\x1b[0m";

    pub const fn new(bg_color: u8, fg_color: u8) -> Self {
        Self {
            bg_color: Some(bg_color),
            fg_color: Some(fg_color),
        }
    }

    pub const fn none() -> Self {
        Self {
            bg_color: None,
            fg_color: None,
        }
    }

    pub const fn null() -> Self {
        Self {
            bg_color: Some(0),
            fg_color: Some(0),
        }
    }

    pub const fn text() -> Self {
        Self {
            bg_color: Some(0),
            fg_color: Some(7),
        }
    }
}

impl From<&Color> for String {
    fn from(color: &Color) -> Self {
        if let Some(bg_color) = color.bg_color {
            if let Some(fg_color) = color.fg_color {
                return format!("\x1b[38;5;{fg_color};48;5;{bg_color}m");
            } else {
                return format!("\x1b[48;5;{bg_color}m");
            }
        } else if let Some(fg_color) = color.fg_color {
            return format!("\x1b[38;5;{fg_color}m");
        }

        String::new()
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}
