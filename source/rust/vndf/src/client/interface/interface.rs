use std::io::{
    self,
    stdin,
};
use std::sync::mpsc::{
    channel,
    Receiver,
    TryRecvError,
};
use std::thread::{
    sleep,
    spawn,
};
use std::time::Duration;
use std::vec::Drain;

use glutin::Event;
use glutin::Event::{Closed};

use nalgebra::cast;

use shared::physics::{SphereCollider};

use client::config::Config;
use client::console;
use client::graphics::Renderer;
use client::interface::{
    Frame,
    InputEvent,
};
use client::keyboard::Keyboard;
use client::mouse::Mouse;
use client::window::Window;

use client::graphics::SHIP_SIZE;

const MAX_FRAME_TIME: f64 = 0.020; // 15ms minimum frame time

pub trait Interface: Sized {
    fn new(config: Config) -> io::Result<Self>;
    fn update(&mut self, frame: &mut Frame) -> io::Result<Drain<InputEvent>>;
}


pub struct Player {
    events  : Vec<InputEvent>,
    cli     : console::Controller,
    window  : Window,
    renderer: Renderer,
    mouse   : Mouse, // NOTE: this might be renamed to selector or controller
    keyboard: Keyboard,
}

impl Interface for Player {
    fn new(config: Config) -> io::Result<Player> {
        let cli    = console::Controller::new();
        let window = Window::new(
            (800.0 * config.scaling_factor) as u32,
            (600.0 * config.scaling_factor) as u32,
            );

        let renderer = Renderer::new(&window, config.scaling_factor);

        Ok(Player {
            events  : Vec::new(),
            cli     : cli,
            window  : window,
            renderer: renderer,
            mouse   : Mouse::new(),
            keyboard: Keyboard::new(),
        })
    }

    fn update(&mut self, frame: &mut Frame)
              -> io::Result<Drain<InputEvent>> {
        let window_events: Vec<Event> = self.window.poll_events().collect();

        // handle a closed-window event
        for event in window_events.iter() {
            match *event {
                Closed => self.events.push(InputEvent::Quit), 
                _ => {},
            }
        }
        
        self.keyboard.update(&mut self.events,
                             frame,
                             &window_events,
                             &mut self.renderer.camera);

        if let Some(size) = self.window.get_size().ok() {
            self.mouse.update(&mut self.events,
                              frame,
                              &window_events,
                              size,
                              &mut self.renderer.camera);
        }
        
        self.cli.update(&mut self.events, frame, &window_events);
        
        if let Some(track) = frame.camera_track.clone() {
            self.renderer.camera.set_track(track);
            frame.camera_track = None; //we should clear this out
        }
        
        self.renderer.render(
            frame,
            &self.cli.console,
            &self.window,
            );
        self.window.swap_buffers();

        check_collisions(frame,self.renderer.camera.zoom);

        // frame delay notifier
        if frame.deltatime > MAX_FRAME_TIME {
            // notify of frame delays
            // TODO: add event type to push (FrameDelay(dt:f64))
        }
        
        Ok(self.events.drain(..))
    }
}

fn check_collisions(frame: &mut Frame,
                    zoom: f32) {
    // TODO: needs some notion of space-partitioning for efficiency
    'ships: for (ship_id,ship_body) in frame.ships.iter() {
        if let Some (_) = frame.colliders.get(&ship_id) {
            // collision
        }
        else {
            warn!("No collider found for ship {}", ship_id);
            continue 'ships
        }

        // check ships colliding into eachother
        'other_ships: for (ship_id2,ship_body2) in frame.ships.iter() {
            if ship_id == ship_id2 { continue 'other_ships }
            
            if let Some (_) = frame.colliders.get(&ship_id2) {
                // collision
            }
            else {
                warn!("No collider found for ship {}", ship_id2);
                continue 'other_ships
            }

            
            // NOTE: previous logic denotes the requirement for colliders
            // even though below function does not require it

            let b = SphereCollider::new_from_oval(SHIP_SIZE * zoom);
            if SphereCollider::check_collision((&b,&cast(ship_body.position)),
                                               (&b,&cast(ship_body2.position)),) {
                // visual collision made between *ship_id,*ship_id2
            }
        }
    }
}






pub struct Headless {
    events  : Vec<InputEvent>,
    receiver: Receiver<InputEvent>,
}

impl Interface for Headless {
    fn new(_: Config) -> io::Result<Headless> {
        let (sender, receiver) = channel();

        spawn(move || -> () {
            let stdin = stdin();

            loop {
                let mut line = String::new();
                match stdin.read_line(&mut line) {
                    Ok(_) => match InputEvent::from_json(line.as_ref()) {
                        Ok(event) =>
                            match sender.send(event) {
                                Ok(()) =>
                                    (),
                                Err(error) =>
                                    panic!("Error sending input: {:?}", error),
                            },
                        Err(error) =>
                            panic!("Error decoding input: {:?}", error),
                    },
                    Err(error) =>
                        panic!("Error reading from stdin: {}", error),
                }
            }
        });

        Ok(Headless {
            events  : Vec::new(),
            receiver: receiver,
        })
    }

    fn update(&mut self, frame: &mut Frame) -> io::Result<Drain<InputEvent>> {
        loop {
            match self.receiver.try_recv() {
                Ok(event) =>
                    self.events.push(event),
                Err(error) => match error {
                    TryRecvError::Empty        => break,
                    TryRecvError::Disconnected => panic!("Channel disconnected"),
                }
            }
        }

        sleep(Duration::from_millis(5));

        print!("{}\n", frame.to_json());

        Ok(self.events.drain(..))
    }
}
