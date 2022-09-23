use crate::{
    common::{color::Color, point::Point},
    state::State,
};

use super::Renderer;

impl Renderer {
    pub fn draw_world(&mut self, state: &State) {
        let color = Color { bg: 1, fg: 1 };

        for y in 0..state.world.size.height() {
            for x in 0..state.world.size.width() {
                let i = (state.world.size.width() * y + x) as usize;
                if state.world.cells[i] != 0 {
                    self.screen.draw_pixel(Point::new(x, y), color);
                }
            }
        }
    }
}
