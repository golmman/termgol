use crate::{common::point::Point, state::State};

use super::Renderer;

impl Renderer {
    pub fn draw_cursor(&mut self, state: &State) {
        if !state.pause {
            return;
        }

        let position = Point::from(state.cursor_pos.clone());

        self.screen
            .draw_text_transparent(position, 7, String::from("X"));
    }
}
