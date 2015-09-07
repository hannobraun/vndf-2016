use rand::{random,thread_rng, sample};

// TODO: Clean up
use glutin::Event;
use glutin::Event::{KeyboardInput,Closed};
use glutin::VirtualKeyCode;
use glutin::ElementState::{Pressed,Released};
use shared::game::{ManeuverData};
use client::graphics::camera::Camera;
use client::interface::{
    Frame,
    InputEvent,
};

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
        frame: &Frame,
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

                    // debug key to send a random maneuver
                    if self.held_keys[VirtualKeyCode::Tab as usize] {
                        if key == VirtualKeyCode::F9 {
                            let mut rng = thread_rng();
                            let a = sample(&mut rng, 1..359, 1);
                            
                            let direction_rad = (a[0] as f64).to_radians();

                            let game_time_s = {
                                if let Some(game_time_s) = frame.game_time_s {
                                    game_time_s
                                }
                                else {return} };
                            let data = ManeuverData {
                                start_s   : game_time_s + 0.0,
                                duration_s: random::<u8>() as f64,
                                angle     : direction_rad,
                            };

                            events.push(InputEvent::ScheduleManeuver(data));
                        }
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
