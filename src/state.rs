use crate::common::point::Point;
use crate::common::DEBUG_INFO_PAGE_TOTAL;

use self::world::World;

mod world;

pub struct State {
    pub debug_info_page: i32,
    pub cursor_pos: Point,
    pub cursor_active: bool,
    pub elapsed_time: u64,
    pub screen_size: Point,

    pub world: World,
}

impl State {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let elapsed_time = 0;

        Self {
            debug_info_page: 1,
            cursor_pos: Point::new(0, 0),
            cursor_active: true,
            elapsed_time,
            screen_size: Point::new(0, 0),
            world: World::new(),
        }
    }

    pub fn resize(&mut self, screen_size: &Point) {
        self.screen_size = screen_size.clone();
        self.world.resize(screen_size);
    }

    pub fn elapse_time(&mut self) {
        self.elapsed_time += 1;
        self.world.update();
    }

    pub fn toggle_cursor_active(&mut self) {
        self.cursor_active = !self.cursor_active;
    }

    pub fn debug_info_next_page(&mut self) {
        self.debug_info_page += 1;

        if self.debug_info_page > DEBUG_INFO_PAGE_TOTAL {
            self.debug_info_page = 0;
        }
    }

    pub fn move_cursor_left(&mut self) {
        if !self.cursor_active {
            return;
        }

        self.cursor_pos.x -= 1;

        if self.cursor_pos.x <= 0 {
            self.cursor_pos.x = 0;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if !self.cursor_active {
            return;
        }

        self.cursor_pos.x += 1;

        if self.cursor_pos.x >= self.screen_size.width() - 1 {
            self.cursor_pos.x = self.screen_size.width() - 1;
        }
    }

    pub fn move_cursor_up(&mut self) {
        if !self.cursor_active {
            return;
        }

        self.cursor_pos.y -= 1;

        if self.cursor_pos.y <= 0 {
            self.cursor_pos.y = 0;
        }
    }

    pub fn move_cursor_down(&mut self) {
        if !self.cursor_active {
            return;
        }

        self.cursor_pos.y += 1;

        if self.cursor_pos.y >= self.screen_size.height() - 1 {
            self.cursor_pos.y = self.screen_size.height() - 1;
        }
    }
}
