use crate::color::Color;
use crate::common::ScreenPoint;
use crate::screen::DefaultScreen;
use crate::screen::Pixel;
use crate::state::State;

pub mod draw_debug_info;

pub struct Renderer {
    screen: DefaultScreen,

    debug_line_y: i32,
}

impl Renderer {
    pub fn new() -> Self {
        let screen = DefaultScreen::new();

        Self {
            screen,
            debug_line_y: 0,
        }
    }

    pub fn resize(&mut self) -> ScreenPoint {
        self.screen.resize()
    }

    pub fn display(&mut self, state: &State) {
        self.screen.clear();

        self.draw_debug_info(state);
        self.draw_cursor(state);

        self.screen.display();
    }

    fn draw_cursor(&mut self, state: &State) {
        let color = Color {
            bg_color: Some(2),
            fg_color: Some(2),
        };

        self.screen.draw_pixel(ScreenPoint::from(state.cursor_pos.clone()), color);
    }
}
