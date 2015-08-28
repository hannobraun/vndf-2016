use glutin::Event;
use glutin::Event::{KeyboardInput,Closed};
use glutin::VirtualKeyCode;
use glutin::ElementState::{Pressed,Released};

use client::interface::{
    Frame,
    InputEvent,
};
use client::render::camera::Camera;

/// Keyboard Input Controller
pub struct Keyboard {
    held_keys: [bool;256], //keys currently being pressed
}

impl Keyboard {
    pub fn new () -> Keyboard {
        Keyboard { held_keys: [false;256] }
    }

    pub fn update(
        &mut self,
        events: &mut Vec<InputEvent>,
        _     : &Frame,
        window_events: &Vec<Event>,
        camera: &mut Camera)
    {
        for event in window_events.iter() {
            match *event {
                KeyboardInput(Pressed, _, Some(VirtualKeyCode::Escape)) => {
                    events.push(InputEvent::Quit);
                },
                KeyboardInput(Pressed, _, Some(key)) => {
                    self.held_keys[key as usize] = true;
                },
                KeyboardInput(Released, _, Some(key)) => {
                    let nkey = key as usize;
                    self.held_keys[nkey] = false;

                    //special case for caps lock
                    if key == VirtualKeyCode::Capital {
                        self.held_keys[nkey] != self.held_keys[nkey];
                    }
                },
                // NOTE: this should probably be in Interface's update
                Closed => events.push(InputEvent::Quit), 
                _ => {},
            }
        }

        if self.held_keys[VirtualKeyCode::Tab as usize] {
            if self.held_keys[VirtualKeyCode::PageUp as usize] {
                camera.zoom(0.25);
            }
            else if self.held_keys[VirtualKeyCode::PageDown as usize] {
                camera.zoom(-0.25);
            }
        }
    }
    
}
