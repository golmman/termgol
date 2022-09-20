use crate::{common::{point::Point, color::Color}, state::State};

use super::Renderer;

impl Renderer {
    pub fn draw_cursor(&mut self, state: &State) {
        let color = Color {
            bg_color: Some(2),
            fg_color: Some(2),
        };

        self.screen
            .draw_pixel(Point::from(state.cursor_pos.clone()), color);
    }
}
