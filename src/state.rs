use crate::common::MapPoint;
use std::collections::HashSet;
use std::rc::Rc;

use crate::renderer::draw_debug_info::DEBUG_INFO_PAGE_TOTAL;

pub struct State {
    pub debug_info_page: i32,
    pub cursor_pos: MapPoint,
    pub elapsed_time: u64,
    pub screen_size: MapPoint,
}

impl State {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let elapsed_time = 0;

        Self {
            debug_info_page: 1,
            cursor_pos: MapPoint::new(0, 0),
            elapsed_time,
            screen_size: MapPoint::new(0, 0),
        }
    }

    pub fn resize(&mut self, screen_size: &MapPoint) {
        self.screen_size = screen_size.clone();
    }

    pub fn elapse_time(&mut self) {
        self.elapsed_time += 1;
    }

    pub fn debug_info_next_page(&mut self) {
        self.debug_info_page += 1;

        if self.debug_info_page > DEBUG_INFO_PAGE_TOTAL {
            self.debug_info_page = 0;
        }
    }

    pub fn move_cursor_left(&mut self) {
        self.cursor_pos.x -= 1;

        if self.cursor_pos.x <= 0 {
            self.cursor_pos.x = 0;
        }
    }

    pub fn move_cursor_right(&mut self) {
        self.cursor_pos.x += 1;

        if self.cursor_pos.x >= self.screen_size.width() - 1 {
            self.cursor_pos.x = self.screen_size.width() - 1;
        }
    }

    pub fn move_cursor_up(&mut self) {
        self.cursor_pos.y -= 1;

        if self.cursor_pos.y <= 0 {
            self.cursor_pos.y = 0;
        }
    }

    pub fn move_cursor_down(&mut self) {
        self.cursor_pos.y += 1;

        if self.cursor_pos.y >= self.screen_size.height() - 1 {
            self.cursor_pos.y = self.screen_size.height() - 1;
        }
    }
}
