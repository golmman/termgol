use std::cmp::{max, min};

use crate::common::args::Args;
use crate::common::point::Point;
use crate::common::DEBUG_INFO_PAGE_TOTAL;

use self::world::World;

mod cell;
mod world;

pub mod cell_image;
pub mod cell_setup;
pub mod rules;

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
        let pause = args.paused;
        let args_clone = args.clone();

        Self {
            args,
            cursor_pos: Point::new(0, 0),
            debug_info_page: 0,
            elapsed_time: 0,
            pause,
            screen_size: Point::new(0, 0),
            world: World::from(args_clone),
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

        self.handle_screen_saver();
        self.elapsed_time += 1;
        self.world.update();
    }

    fn handle_screen_saver(&mut self) {
        if let Some(screen_saver) = self.args.screen_saver {
            if self.elapsed_time < 30 {
                self.world.color_alpha = min(255, self.world.color_alpha as i32 + 10) as u8;
            }

            if self.elapsed_time > (screen_saver + 30) as u64 {
                self.world.color_alpha = max(0, self.world.color_alpha as i32 - 10) as u8;
            }

            if self.elapsed_time >= (screen_saver + 60) as u64 {
                self.world = World::from(self.args.clone());
                self.world.resize(&self.screen_size);
                self.elapsed_time = 0;
            }
        }
    }

    pub fn toggle_pause(&mut self) {
        self.pause = !self.pause;
    }

    pub fn toggle_life_at_cursor(&mut self) {
        if !self.pause {
            return;
        }

        let i = (self.world.size.width() * self.cursor_pos.y + self.cursor_pos.x) as usize;
        if self.world.cells[i].alive {
            self.world.set_dead(i);
        } else {
            self.world.set_alive(i);
        }
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
