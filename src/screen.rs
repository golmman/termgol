use std::io::stdout;
use std::io::Stdout;
use std::io::Write;

use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;

use crate::common::color::Color;
use crate::common::color::Rgb;
use crate::common::point::Point;

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
    pub size: Point,
}

impl DefaultScreen {
    pub fn new() -> Self {
        Screen::from(stdout().into_raw_mode().unwrap())
    }

    pub fn resize(&mut self) -> Point {
        let (cols, rows) = termion::terminal_size().unwrap();

        self.size = Point::new(cols as i32, rows as i32);

        self.size.clone()
    }

    pub fn clear(&mut self) {
        let buffer_size = (self.size.width() * self.size.height()) as usize;
        self.prelude_buffer = String::new();
        self.pixel_buffer = vec![Pixel::from(' '); buffer_size];
    }

    pub fn draw_pixel(&mut self, p: Point, color: Color) {
        let index = (self.size.width() * p.y + p.x) as usize;
        self.pixel_buffer[index] = Pixel { ch: ' ', color };
    }

    pub fn draw_text(&mut self, p: Point, color: Color, text: String) {
        let index = (self.size.width() * p.y + p.x) as usize;

        for (i, ch) in text.chars().enumerate() {
            self.pixel_buffer[index + i] = Pixel { ch, color };
        }
    }

    pub fn draw_text_transparent(&mut self, p: Point, fg_color: Rgb, text: String) {
        let index = (self.size.width() * p.y + p.x) as usize;
        for (i, ch) in text.chars().enumerate() {
            let color = Color {
                bg: self.pixel_buffer[index + i].color.bg,
                fg: fg_color,
            };
            self.pixel_buffer[index + i] = Pixel { ch, color };
        }
    }

    pub fn display(&mut self) {
        let mut s = String::new();

        s.push_str(&self.prelude_buffer);

        for y in 0..self.size.height() {
            let row = y + 1;
            s.push_str(&format!("\x1b[{row};1H")); // goto (row, 1)

            let mut x = 0;

            while x < self.size.width() {
                let mut i = (self.size.width() * y + x) as usize;
                let mut last_color = self.pixel_buffer[i].color;
                s.push_str(&format!("{last_color}"));

                while x < self.size.width() && last_color == self.pixel_buffer[i].color {
                    last_color = self.pixel_buffer[i].color;
                    s.push(self.pixel_buffer[i].ch);

                    x += 1;
                    i = (self.size.width() * y + x) as usize;
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
            size: Point::new(cols as i32, rows as i32),
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
