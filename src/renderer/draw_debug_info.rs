use super::Renderer;
use crate::color::Color;
use crate::common::ScreenPoint;
use crate::state::State;

pub const DEBUG_INFO_PAGE_TOTAL: i32 = 2;

impl Renderer {
    pub fn draw_debug_info(&mut self, state: &State) {
        match state.debug_info_page {
            0 => (),
            1 => self.draw_debug_info_general(state),
            _ => panic!("debug info page {} out of bounds", state.debug_info_page),
        }
    }

    fn draw_next_line(&mut self, formatted_string: String) {
        let color = Color {
            bg_color: None,
            fg_color: Some(7),
        };

        self.screen.draw_text(ScreenPoint::new(0, self.debug_line_y), color, formatted_string);
        self.debug_line_y += 1;
    }

    fn draw_page_info(&mut self, state: &State, text: &str) {
        self.debug_line_y = 0;

        self.draw_next_line(format!(
            //"\x1b[1m{}/{} {}\x1b[22m",
            "{}/{} {}",
            state.debug_info_page, DEBUG_INFO_PAGE_TOTAL, text,
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
