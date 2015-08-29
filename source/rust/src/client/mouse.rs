use glutin::ElementState::Pressed;
use glutin::Event;
use glutin::Event::{
    MouseMoved
};
use client::window::Window;

pub struct Mouse {
    pos: (i32,i32),
}

impl Mouse {
    pub fn new() -> Mouse {
        Mouse { pos: (0,0) }
    }
    pub fn update (&mut self, window_events: &Vec<Event>) {
        for event in window_events.iter() {
            match *event {
                MouseMoved(pos) => { self.pos = pos; },
                _ => { },
            }
        }
    }

}
