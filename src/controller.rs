use term2d::model::event::Event;
use term2d::model::key::Key;
use term2d::view::canvas::halfblock::HalfblockCanvas;

use crate::renderer::Renderer;
use crate::state::State;

pub struct Controller {
    renderer: Renderer,
    state: State,
}

impl From<State> for Controller {
    fn from(state: State) -> Self {
        let renderer = Renderer::new();

        Self { renderer, state }
    }
}

impl term2d::controller::Controller<HalfblockCanvas> for Controller {
    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::Key(key) => match key {
                Key::Char('q') => return false,
                Key::Ctrl('c') => return false,

                Key::Char('h') => self.state.move_cursor_left(),
                Key::Char('l') => self.state.move_cursor_right(),
                Key::Char('k') => self.state.move_cursor_up(),
                Key::Char('j') => self.state.move_cursor_down(),

                Key::Char('p') => self.state.toggle_pause(),
                Key::Char('d') => self.state.debug_info_next_page(),

                Key::Char(' ') => self.state.toggle_life_at_cursor(),

                _ => {}
            },
            Event::Resize => {
                let size = self.renderer.resize();
                self.state.resize(&size.into());
            }
            Event::Elapse => self.state.elapse_time(),
        }

        self.renderer.display(&self.state);

        true
    }

    fn get_canvas(&mut self) -> &mut HalfblockCanvas {
        &mut self.renderer.canvas
    }
}
