use crate::{common::point::Point, state::State};

use super::Renderer;

impl Renderer {
    pub fn draw_world(&mut self, state: &State) {
        for y in 0..state.world.size.height() {
            for x in 0..state.world.size.width() {
                let i = (state.world.size.width() * y + x) as usize;
                self.screen
                    .draw_pixel(Point::new(x, y), state.world.cells[i].color);
            }
        }
    }
}
