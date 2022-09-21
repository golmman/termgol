use crate::{
    common::{color::Color, point::Point},
    state::State,
};

use super::Renderer;

impl Renderer {
    pub fn draw_cursor(&mut self, state: &State) {
        let color = Color { bg: 2, fg: 2 };

        self.screen
            .draw_pixel(Point::from(state.cursor_pos.clone()), color);
    }
}
