use crate::{
    common::{color::Color, point::Point},
    state::State,
};

use super::Renderer;

impl Renderer {
    pub fn draw_cursor(&mut self, state: &State) {
        if !state.cursor_active {
            return;
        }

        let color = Color { bg: 2, fg: 2 };
        let position = Point::from(state.cursor_pos.clone());

        self.screen.draw_pixel(position, color);
    }
}
