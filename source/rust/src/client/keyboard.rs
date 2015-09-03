use glutin::Event;
use glutin::Event::{KeyboardInput,Closed};
use glutin::VirtualKeyCode;
use glutin::ElementState::Pressed;

use client::interface::{
    Frame,
    InputEvent,
};

/// Keyboard Input Controller
pub struct Keyboard;// {
    //held_keys: [u8;u8], //keys currently being pressed
//}

impl Keyboard {
    pub fn new () -> Keyboard {
        Keyboard //{ held_keys: , }
    }

    pub fn update(
        &mut self,
        events: &mut Vec<InputEvent>,
        frame : &Frame,
        window_events: &Vec<Event>,)
    {
        for event in window_events.iter() {
            match *event {
                KeyboardInput(Pressed, _, Some(VirtualKeyCode::Escape)) => {
                    events.push(InputEvent::Quit);
                },
                KeyboardInput(Pressed, _, Some(key)) => {
                    println!("{:?}",key as usize);
                },
                Closed => events.push(InputEvent::Quit),
                _ => {},
            }
        }
    }
    
}
