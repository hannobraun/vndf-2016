#![cfg_attr(test, allow(dead_code))]


extern crate env_logger;
#[macro_use]
extern crate log;
extern crate time;

extern crate vndf;


use std::collections::HashMap;
use std::env;
use std::thread::sleep_ms;
use time::precise_time_s;

use vndf::client::args::Args;
use vndf::client::config::Config;
use vndf::client::interface::{
    self,
    Frame,
    InputEvent,
    Interface,
    Message,
};
use vndf::client::network::Network;
use vndf::shared::protocol::client::schedule_maneuver;
use vndf::shared::protocol::client::Event as ClientEvent;
use vndf::shared::protocol::client::event as client_event;
use vndf::shared::protocol::server;

const MIN_TIME: f64 = 0.015; // 15ms minimum frame time

fn main() {
    env_logger::init().unwrap_or_else(|e|
        panic!("Error initializing logger: {}", e)
    );

    let args = Args::parse(env::args());

    if args.headless {
        run(args, init_interface::<interface::Headless>())
    }
    else {
        run(args, init_interface::<interface::Player>())
    }
}


fn init_interface<I: Interface>() -> I {
    match Interface::new(Config::load()) {
        Ok(interface) => interface,
        Err(error)    => panic!("Error initializing interface: {}", error),
    }
}

fn run<I: Interface>(args: Args, mut interface: I) {    
    let mut frame = Frame::new();
    
    let mut broadcasts = HashMap::new();
    let mut ships      = HashMap::new();
    
    
    let mut network = Network::new(args.server);

    let mut last_server_activity = precise_time_s();

    network.send(ClientEvent::Public(client_event::Public::Login));

    let mut frame_time = precise_time_s();
    
    'main: loop {
        trace!("Start client main loop iteration");

        let input_events = match interface.update(&mut frame) {
            Ok(events) => events,
            Err(error) => panic!("Error updating interface: {}", error),
        };

        frame.message = Message::None;

        for event in input_events {
            match event {
                InputEvent::StartBroadcast(message) =>
                    if message.len() == 0 {
                        frame.message = Message::Error(
                            "Broadcasts can not be empty".to_string()
                                );
                    }
                else if message.len() > 256 {
                    frame.message = Message::Error(
                        "Broadcast message too long".to_string()
                            );
                }
                else {
                    network.send(
                        ClientEvent::Privileged(client_event::Privileged::StartBroadcast(message.clone()))
                            );

                    frame.message = Message::Notice(
                        "Sending broadcast".to_string()
                            );
                },
                InputEvent::StopBroadcast => {
                    network.send(ClientEvent::Privileged(client_event::Privileged::StopBroadcast));

                    frame.message = Message::Notice(
                        "Stopped sending broadcast".to_string()
                            );
                },
                InputEvent::ScheduleManeuver(data) => {
                    network.send(schedule_maneuver(data));

                    frame.message = Message::Notice(
                        "Scheduling maneuver".to_string()
                            );
                },
                InputEvent::Track(track) => {
                    frame.camera_track = Some(track);
                },
                InputEvent::Select(ids) => {
                    frame.select_ids = ids;
                },
                InputEvent::Quit => {
                    break 'main;
                },
            }
        }

        for event in network.receive() {
            match event {
                server::Event::Heartbeat(game_time_s) => {
                    frame.game_time_s = Some(game_time_s);
                },
                server::Event::ShipId(ship_id) => {
                    frame.ship_id = Some(ship_id);
                },
                server::Event::UpdateEntity(id, (ship, broadcast)) => {
                    ships.insert(id, ship);

                    match broadcast {
                        Some(broadcast) => {
                            broadcasts.insert(id, broadcast.message);
                        },
                        None => {
                            broadcasts.remove(&id);
                        }
                    }
                },
                server::Event::RemoveEntity(id) => {
                    broadcasts.remove(&id);
                    ships.remove(&id);
                },
            }

            last_server_activity = precise_time_s();
        }

        if precise_time_s() - last_server_activity > args.net_timeout_s {
            frame.message = Message::Error(
                "Lost connection to server".to_string()
                    );
        }

        frame.broadcasts = broadcasts.clone();
        frame.ships = ships
            .iter()
            .map(|(id, ship)|
                 (*id, *ship)
                 )
            .collect();

        network.send(ClientEvent::Privileged(client_event::Privileged::Heartbeat));

        let dt = precise_time_s() - frame_time;
        if dt < MIN_TIME {
            sleep_ms(((MIN_TIME - dt)*1000.0) as u32);
        }
        frame_time = precise_time_s();
    }
}
