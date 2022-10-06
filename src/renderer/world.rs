use crate::{common::point::Point, state::State};

use super::Renderer;

impl Renderer {
    pub fn draw_world(&mut self, state: &State) {
        for y in 0..state.world.size.height() {
            for x in 0..state.world.size.width() {
                let i = (state.world.size.width() * y + x) as usize;
                let mut cell_color = state.world.cells[i].color.clone();
                cell_color.bg = cell_color
                    .bg
                    .blend(&state.world.color_bg_dead, state.world.color_alpha);
                self.screen.draw_pixel(Point::new(x, y), cell_color);
            }
        }
    }
}
