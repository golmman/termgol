use std::io::stdout;
use std::io::Stdout;
use std::io::Write;

use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;

use crate::color::Color;
use crate::common::intersect;
use crate::common::RectAbsolute;
use crate::common::ScreenPoint;

pub type DefaultScreen = Screen<RawTerminal<Stdout>>;

#[derive(Debug, Clone)]
pub struct Pixel {
    pub ch: char,
    pub color: Color,
}

impl From<char> for Pixel {
    fn from(ch: char) -> Self {
        Self {
            color: Color::text(),
            ch,
        }
    }
}

pub struct Screen<W: Write> {
    main_display: W,
    prelude_buffer: String,
    pixel_buffer: Vec<Pixel>,
    pub size: ScreenPoint,
}

impl DefaultScreen {
    pub fn new() -> Self {
        Screen::from(stdout().into_raw_mode().unwrap())
    }

    pub fn resize(&mut self) -> ScreenPoint {
        let (cols, rows) = termion::terminal_size().unwrap();

        self.size = ScreenPoint::new(cols as i32, rows as i32);

        self.size.clone()
    }

    pub fn clear(&mut self) {
        let buffer_size = (self.size.width() * self.size.height()) as usize;
        self.prelude_buffer = String::new();
        self.pixel_buffer = vec![Pixel::from(' '); buffer_size];

        //self.prelude_buffer.push_str("\x1b[2J"); // clear screen
        //self.prelude_buffer.push_str("\x1b[H"); // goto to (1, 1)
    }

    pub fn draw_color(&mut self, pos: ScreenPoint, size: ScreenPoint, color: Color) {
        let screen_rect = RectAbsolute {
            x1: 0,
            y1: 0,
            x2: self.size.width(),
            y2: self.size.height(),
        };

        let sprite_rect = RectAbsolute {
            x1: pos.x,
            y1: pos.y,
            x2: pos.x + size.width(),
            y2: pos.y + size.height(),
        };

        let intersection = intersect(&screen_rect, &sprite_rect);

        for sprite_y in intersection.y1..intersection.y2 {
            for sprite_x in intersection.x1..intersection.x2 {
                let screen_i = (self.size.width() * sprite_y + sprite_x) as usize;
                let ch = self.pixel_buffer[screen_i].ch;

                self.pixel_buffer[screen_i] = Pixel { ch, color };
            }
        }
    }

    pub fn draw_inversion(&mut self, pos: ScreenPoint, size: ScreenPoint) {
        let screen_rect = RectAbsolute {
            x1: 0,
            y1: 0,
            x2: self.size.width(),
            y2: self.size.height(),
        };

        let sprite_rect = RectAbsolute {
            x1: pos.x,
            y1: pos.y,
            x2: pos.x + size.width(),
            y2: pos.y + size.height(),
        };

        let intersection = intersect(&screen_rect, &sprite_rect);

        for sprite_y in intersection.y1..intersection.y2 {
            for sprite_x in intersection.x1..intersection.x2 {
                let screen_i = (self.size.width() * sprite_y + sprite_x) as usize;

                let fg_color = self.pixel_buffer[screen_i].color.bg_color;

                let bg_color = self.pixel_buffer[screen_i].color.fg_color;

                let ch = self.pixel_buffer[screen_i].ch;

                self.pixel_buffer[screen_i] = Pixel {
                    ch,
                    color: Color { fg_color, bg_color },
                };
            }
        }
    }

    pub fn draw_pixel(&mut self, p: ScreenPoint, color: Color) {
        let index = (self.size.width() * p.y + p.x) as usize;
        self.pixel_buffer[index] = Pixel { ch: ' ', color };
    }

    pub fn draw_text(&mut self, p: ScreenPoint, color: Color, text: String) {
        let index = (self.size.width() * p.y + p.x) as usize;

        for (i, ch) in text.chars().enumerate() {
            self.pixel_buffer[index + i] = Pixel { ch, color };
        }
    }

    pub fn display(&mut self) {
        let mut s = String::new();

        s.push_str(&self.prelude_buffer);

        for y in 0..self.size.height() {
            let row = y + 1;
            s.push_str(&format!("\x1b[{row};1H")); // goto (row, 1)

            let mut last_color = Color::none();

            // TODO: background color is not properly set, when e.g. transparent npc is next to another color
            for x in 0..self.size.width() {
                let i = (self.size.width() * y + x) as usize;
                let ch = self.pixel_buffer[i].ch;
                let color = self.pixel_buffer[i].color;

                let mut change_color = Color::none();

                if color.bg_color.is_some() && color.bg_color != last_color.bg_color {
                    change_color.bg_color = color.bg_color;
                }

                if color.fg_color.is_some() && color.fg_color != last_color.fg_color {
                    change_color.fg_color = color.fg_color;
                }

                s.push_str(&format!("{change_color}{ch}"));

                if change_color.bg_color.is_some() {
                    last_color.bg_color = change_color.bg_color;
                }

                if change_color.fg_color.is_some() {
                    last_color.fg_color = change_color.fg_color;
                }
            }
        }

        self.main_display.write_all(s.as_bytes()).unwrap();
        self.main_display.flush().unwrap();
    }
}

impl<W: Write> From<W> for Screen<W> {
    fn from(mut buffer: W) -> Self {
        write!(
            buffer,
            "{}{}{}",
            termion::cursor::Hide,
            termion::cursor::Goto(1, 1),
            termion::clear::All,
        )
        .unwrap();

        buffer.flush().unwrap();

        let (cols, rows) = termion::terminal_size().unwrap();
        let buffer_size = (cols * rows) as usize;

        let prelude_buffer = String::new();
        let pixel_buffer = vec![Pixel::from(' '); buffer_size];

        Self {
            main_display: buffer,
            prelude_buffer,
            pixel_buffer,
            size: ScreenPoint::new(cols as i32, rows as i32),
        }
    }
}

impl<W: Write> Drop for Screen<W> {
    fn drop(&mut self) {
        write!(
            self.main_display,
            "{}{}{}{}",
            Color::RESET,
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::cursor::Show,
        )
        .unwrap();

        self.main_display.flush().unwrap();
    }
}
