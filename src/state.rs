use crate::common::args::Args;
use crate::common::point::Point;
use crate::common::DEBUG_INFO_PAGE_TOTAL;

use self::world::World;

mod world;

pub struct State {
    pub args: Args,
    pub cursor_pos: Point,
    pub debug_info_page: i32,
    pub elapsed_time: u64,
    pub pause: bool,
    pub screen_size: Point,
    pub world: World,
}

impl From<Args> for State {
    fn from(args: Args) -> Self {
        Self {
            args,
            cursor_pos: Point::new(0, 0),
            debug_info_page: 0,
            elapsed_time: 0,
            pause: false,
            screen_size: Point::new(0, 0),
            world: World::new(),
        }
    }
}

impl State {
    pub fn resize(&mut self, screen_size: &Point) {
        self.screen_size = screen_size.clone();
        self.world.resize(screen_size);
    }

    pub fn elapse_time(&mut self) {
        if self.pause {
            return;
        }

        self.elapsed_time += 1;
        self.world.update();
    }

    pub fn toggle_pause(&mut self) {
        self.pause = !self.pause;
    }

    pub fn toggle_life_at_cursor(&mut self) {
        if !self.pause {
            return;
        }

        let i = (self.world.size.width() * self.cursor_pos.y + self.cursor_pos.x) as usize;
        self.world.cells[i] = 1 - self.world.cells[i];
    }

    pub fn debug_info_next_page(&mut self) {
        self.debug_info_page += 1;

        if self.debug_info_page > DEBUG_INFO_PAGE_TOTAL {
            self.debug_info_page = 0;
        }
    }

    pub fn move_cursor_left(&mut self) {
        if !self.pause {
            return;
        }

        self.cursor_pos.x -= 1;

        if self.cursor_pos.x <= 0 {
            self.cursor_pos.x = 0;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if !self.pause {
            return;
        }

        self.cursor_pos.x += 1;

        if self.cursor_pos.x >= self.screen_size.width() - 1 {
            self.cursor_pos.x = self.screen_size.width() - 1;
        }
    }

    pub fn move_cursor_up(&mut self) {
        if !self.pause {
            return;
        }

        self.cursor_pos.y -= 1;

        if self.cursor_pos.y <= 0 {
            self.cursor_pos.y = 0;
        }
    }

    pub fn move_cursor_down(&mut self) {
        if !self.pause {
            return;
        }

        self.cursor_pos.y += 1;

        if self.cursor_pos.y >= self.screen_size.height() - 1 {
            self.cursor_pos.y = self.screen_size.height() - 1;
        }
    }
}
