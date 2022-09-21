use std::io::stdin;
use std::sync::mpsc::sync_channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::SyncSender;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use termion::event::Key;
use termion::input::TermRead;

use crate::common::point::Point;
use crate::common::FRAMES_PER_SECOND;
use crate::renderer::Renderer;
use crate::state::State;

pub enum TerminalEvent {
    Key(Key),
    Resize,
    Elapse,
}

pub struct Controller {
    receiver: Receiver<TerminalEvent>,
    renderer: Renderer,
    sender: SyncSender<TerminalEvent>,
    state: State,
}

impl Controller {
    pub fn new() -> Self {
        let (sender, receiver) = sync_channel::<TerminalEvent>(1024);
        let renderer = Renderer::new();

        let state = State::new();

        Self {
            receiver,
            renderer,
            sender,
            state,
        }
    }

    pub fn run(&mut self) {
        self.resize();
        self.state.cursor_pos = Point::new(
            self.state.screen_size.width() / 2,
            self.state.screen_size.height() / 2,
        );

        let elapse_sender = self.sender.clone();
        let key_sender = self.sender.clone();
        let interrupt_sender = self.sender.clone();
        let resize_sender = self.sender.clone();

        thread::spawn(move || Controller::send_elapse_events(elapse_sender));
        thread::spawn(move || Controller::send_key_events(key_sender));
        thread::spawn(move || Controller::send_interrupt_events(interrupt_sender));
        thread::spawn(move || Controller::send_resize_events(resize_sender));

        self.renderer.display(&self.state);

        while self.receive_event() {}
    }

    fn send_elapse_events(sender: SyncSender<TerminalEvent>) {
        loop {
            sleep(Duration::from_millis(1000 / FRAMES_PER_SECOND as u64));
            let _ = sender.send(TerminalEvent::Elapse);
        }
    }

    fn send_key_events(sender: SyncSender<TerminalEvent>) {
        let stdin = stdin();

        for key in stdin.keys().flatten() {
            let _ = sender.send(TerminalEvent::Key(key));
        }
    }

    fn send_interrupt_events(sync_sender: SyncSender<TerminalEvent>) {
        let _ = unsafe {
            signal_hook::low_level::register(signal_hook::consts::SIGINT, move || {
                sync_sender
                    .send(TerminalEvent::Key(Key::Char('q')))
                    .unwrap();
            })
        };
    }

    fn send_resize_events(sync_sender: SyncSender<TerminalEvent>) {
        let _ = unsafe {
            signal_hook::low_level::register(signal_hook::consts::SIGWINCH, move || {
                sync_sender.send(TerminalEvent::Resize).unwrap();
            })
        };
    }

    fn receive_event(&mut self) -> bool {
        let event = self.receiver.recv().unwrap();

        match event {
            TerminalEvent::Key(key) => match key {
                Key::Char('q') => return false,
                Key::Ctrl('c') => return false,

                Key::Char('h') => self.state.move_cursor_left(),
                Key::Char('l') => self.state.move_cursor_right(),
                Key::Char('k') => self.state.move_cursor_up(),
                Key::Char('j') => self.state.move_cursor_down(),

                Key::Char('d') => self.state.debug_info_next_page(),

                _ => {}
            },
            TerminalEvent::Resize => self.resize(),
            TerminalEvent::Elapse => {
                self.state.elapse_time();
            }
        }

        self.renderer.display(&self.state);

        true
    }

    fn resize(&mut self) {
        let size = self.renderer.resize();
        self.state.resize(&size.into());
    }
}
