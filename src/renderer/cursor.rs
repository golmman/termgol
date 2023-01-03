use crate::state::State;
use term2d::model::point::Point;
use term2d::model::rgba::Rgba;
use term2d::view::canvas::Canvas;

use super::Renderer;

impl Renderer {
    pub fn draw_cursor(&mut self, state: &State) {
        if !state.pause {
            return;
        }

        let position = Point::from(state.cursor_pos.clone());

        self.canvas.draw_pixel(
            &position,
            &Rgba {
                r: 255,
                g: 255,
                b: 255,
                a: 255,
            },
        );
    }
}
