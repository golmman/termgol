use crate::state::State;
use term2d::model::point::Point;
use term2d::view::canvas::halfblock::HalfblockCanvas;
use term2d::view::canvas::Canvas;

mod cursor;
mod debug_info;
mod world;

pub struct Renderer {
    pub canvas: HalfblockCanvas,
    debug_line_y: i32,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            canvas: HalfblockCanvas::new(),
            debug_line_y: 0,
        }
    }

    pub fn resize(&mut self) -> Point {
        self.canvas.resize()
    }

    pub fn display(&mut self, state: &State) {
        self.canvas.clear();

        self.draw_world(state);
        self.draw_debug_info(state);
        self.draw_cursor(state);

        self.canvas.display();
    }
}
