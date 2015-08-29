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

use client::cli::Cli;
use client::config::Config;
use client::interface::{
    Frame,
    InputEvent,
};
use client::render::Renderer;
use client::window::Window;
use client::render::camera::CameraTrack;

pub trait Interface: Sized {
    fn new(config: Config) -> io::Result<Self>;
    fn update(&mut self, frame: &Frame, maybe_track: Option<CameraTrack>)
              -> io::Result<Drain<InputEvent>>;
}


pub struct Player {
    events  : Vec<InputEvent>,
    cli     : Cli,
    window  : Window,
    renderer: Renderer,
}

impl Interface for Player {
    fn new(config: Config) -> io::Result<Player> {
        let cli    = Cli::new();
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
        })
    }

    fn update(&mut self, frame: &Frame, maybe_track: Option<CameraTrack>)
              -> io::Result<Drain<InputEvent>> {
        self.cli.update(&mut self.events, frame, &self.window);
        
        if let Some(track) = maybe_track { self.renderer.camera.set(track); }
        
        self.renderer.render(
            self.cli.text(),
            (self.cli.input(),self.cli.get_prompt_idx()),
            frame,
            &self.window,
            );
        self.window.swap_buffers();

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

    fn update(&mut self, frame: &Frame, _: Option<CameraTrack>)
              -> io::Result<Drain<InputEvent>> {
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
