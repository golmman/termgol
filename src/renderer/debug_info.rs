use super::Renderer;
use crate::common::color::Color;
use crate::common::point::Point;
use crate::common::DEBUG_INFO_PAGE_TOTAL;
use crate::state::State;

impl Renderer {
    pub fn draw_debug_info(&mut self, state: &State) {
        match state.debug_info_page {
            0 => (),
            1 => self.draw_debug_info_general(state),
            _ => panic!("debug info page {} out of bounds", state.debug_info_page),
        }
    }

    fn draw_next_line(&mut self, formatted_string: String) {
        let color = Color::text();

        self.screen
            .draw_text(Point::new(0, self.debug_line_y), color, formatted_string);
        self.debug_line_y += 1;
    }

    fn draw_page_info(&mut self, state: &State, text: &str) {
        self.debug_line_y = 0;

        self.draw_next_line(format!(
            //"\x1b[1m{}/{} {}\x1b[22m",
            "{}/{} {} {}",
            state.debug_info_page,
            DEBUG_INFO_PAGE_TOTAL,
            text,
            -12 % 7
        ));
    }

    fn draw_debug_info_general(&mut self, state: &State) {
        self.draw_page_info(state, "General");

        self.draw_next_line(format!(
            "cols: {}, rows: {}, tiles_x: {}, tiles_y: {}, time: {}",
            self.screen.size.width(),
            self.screen.size.height(),
            state.screen_size.width(),
            state.screen_size.height(),
            state.elapsed_time,
        ));

        self.draw_next_line(format!(
            "cursor_x: {}, cursor_y: {}",
            state.cursor_pos.x, state.cursor_pos.y,
        ));
    }
}
