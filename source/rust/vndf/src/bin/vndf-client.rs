#![cfg_attr(test, allow(dead_code))]


extern crate env_logger;
#[macro_use]
extern crate log;
extern crate time;

extern crate vndf;

use std::env;
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
use vndf::shared::game::Attributes;
use vndf::shared::planet::Planet;
use vndf::shared::physics::collision::Collider;
use vndf::client::graphics::camera::CameraTrack;

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

    let mut network = Network::new(args.server);
    let mut last_server_activity = precise_time_s();
    
    let mut frame_time = precise_time_s();

    network.send(ClientEvent::Public(client_event::Public::Login));

    let config = interface.get_config();

    let mut scale = 1.0;
    if let Some(conf) = config { scale = conf.scaling_factor; }
    
    'main: loop {
        let now = precise_time_s();
        frame.deltatime = now-frame_time;
        frame_time = now;
        
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
                    // TODO: consider moving this logic to Interface
                    for id in ids {
                        frame.select_ids.insert(id);
                    }

                    frame.camera_track = Some(CameraTrack::Entity
                                              (frame.select_ids.clone()));
                },
                InputEvent::Deselect(ids) => {
                    if ids.is_empty() { frame.select_ids.clear(); }
                    else {
                        for id in ids {
                            frame.select_ids.remove(&id);
                        }
                    }
                    // update camera tracking
                    if frame.select_ids.is_empty() {
                        frame.camera_track = Some(CameraTrack::Default);
                    }
                    else {
                        frame.camera_track = Some(CameraTrack::Entity
                                                  (frame.select_ids.clone()));
                    }
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
                server::Event::UpdateEntity(ent) => {
		    let (id, (body,broadcast,attr)) = ent;

		    // for now match against attr, later we should cache this
		    match attr {
			Some(Attributes::Ship) => {
                            frame.ships.insert(id, body);
                            if !frame.colliders.contains_key(&id) {
                                frame.colliders.insert(
                                    id,
                                    Collider::new_from_ship(scale));
                            }
                        },
			Some(Attributes::Planet(attr)) => {
			    let planet = Planet { body: body,
						  attr: attr };
			    frame.planets.insert(id,planet);

                            if !frame.colliders.contains_key(&id) {
                                frame.colliders.insert(
                                    id,
                                    Collider::new_from_planet(scale,
                                                              attr.size));
                            }
			},
			_ =>  { //default to ships
                            frame.ships.insert(id, body);
                            if !frame.colliders.contains_key(&id) {
                                frame.colliders.insert(
                                    id,
                                    Collider::new_from_ship(scale));
                            }
                        },
		    }

                    match broadcast {
                        Some(broadcast) => {
                            frame.broadcasts.insert(id, broadcast.message);
                        },
                        None => {
                            frame.broadcasts.remove(&id);
                        }
                    }
                },
                server::Event::RemoveEntity(id) => {
                    frame.broadcasts.remove(&id);
                    frame.ships.remove(&id);
                },
            }

            last_server_activity = precise_time_s();
        }

        if precise_time_s() - last_server_activity > args.net_timeout_s {
            frame.message = Message::Error(
                "Lost connection to server".to_string()
                    );
        }

        network.send(ClientEvent::Privileged(client_event::Privileged::Heartbeat));
    }
}
