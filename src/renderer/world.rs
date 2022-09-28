use crate::{
    common::{
        color::{Color, Rgb},
        point::Point,
    },
    state::State,
};

use super::Renderer;

impl Renderer {
    pub fn draw_world(&mut self, state: &State) {
        let color = Color {
            bg: Rgb {
                r: 200,
                g: 110,
                b: 0,
            },
            fg: Rgb { r: 0, g: 0, b: 0 },
        };

        for y in 0..state.world.size.height() {
            for x in 0..state.world.size.width() {
                let i = (state.world.size.width() * y + x) as usize;
                if state.world.cells[i].alive {
                    self.screen.draw_pixel(Point::new(x, y), color);
                }
            }
        }
    }
}
