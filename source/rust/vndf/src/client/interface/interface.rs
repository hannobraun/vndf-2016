use std::io::{
    self,
    stdin,
};
use std::sync::mpsc::{
    channel,
    Receiver,
    TryRecvError,
};
use std::thread::spawn;
use std::vec::Drain;

use client::console;
use client::mouse::Mouse;
use client::keyboard::Keyboard;
use client::config::Config;
use client::interface::{
    Frame,
    InputEvent,
};
use client::render::Renderer;
use client::window::Window;

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
        let window_events = self.window.poll_events().collect();
        self.keyboard.update(&mut self.events,
                             frame,
                             &window_events,
                             &mut self.renderer.camera);
        
        self.mouse.update(&mut self.events,
                          frame,
                          &window_events,
                          self.window.get_size(),
                          &mut self.renderer.camera);
        self.cli.update(&mut self.events, frame, &window_events);
        
        if let Some(track) = frame.camera_track.clone() {
            self.renderer.camera.set_track(track);
            frame.camera_track = None; //we should clear this out
        }

        // interpolate ship position
        for (_,ship) in frame.ships.iter_mut() {
            let pos = ship.position+(ship.velocity*frame.deltatime*1.99);
            ship.position = pos;
        }
        
        self.renderer.render(
            frame,
            &self.cli.console,
            &self.window,
        );
        self.window.swap_buffers();

        // frame delay notifier
        if frame.deltatime > MAX_FRAME_TIME {
            // notify of frame delays
            // TODO: add event type to push (FrameDelay(dt:f64))
        }
        
        Ok(self.events.drain(..))
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

        print!("{}\n", frame.to_json());

        Ok(self.events.drain(..))
    }
}
