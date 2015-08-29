use glutin::ElementState::Pressed;
use glutin::Event;
use glutin::Event::{
    Closed,
    KeyboardInput,
    ReceivedCharacter,
};
use client::window::Window;

pub struct Mouse;

//pub impl Mouse {
    
pub fn mouse (window_events: &Vec<Event>) {
    for event in window_events.iter() {
        println!("{:?}",event);
    }
}
