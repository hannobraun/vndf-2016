#![cfg_attr(test, allow(dead_code))]


extern crate env_logger;
#[macro_use]
extern crate log;

extern crate vndf;

use std::env;

use vndf::client::args::Args;
use vndf::client::config::Config;
use vndf::client::interface::{
    self,
    Frame,
    InputEvent,
    Interface,
    Message,
};
use vndf::client::interpolator::Interpolator;
use vndf::client::network::Network;
use vndf::client::times::Times;
use vndf::shared::protocol::client::{
    cancel_maneuver,
    ftl_jump,
    schedule_maneuver,
};
use vndf::shared::protocol::client::Event as ClientEvent;
use vndf::shared::protocol::client::event as client_event;
use vndf::shared::protocol::server;
use vndf::shared::physics::SphereCollider;
use vndf::client::graphics::camera::CameraTrack;

use vndf::client::graphics::SHIP_SIZE;

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
    let mut times = Times::new();

    let mut frame        = Frame::new();
    let mut interpolator = Interpolator::new();

    let mut network = Network::new(args.server);
    let mut last_server_activity = times.client_now_s();

    let mut frame_time = times.client_now_s();

    network.send(ClientEvent::Public(client_event::Public::Login));

    'main: loop {
        let now = times.client_now_s();
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

                InputEvent::CancelManeuver(id) => {
                    network.send(cancel_maneuver(id));

                    frame.message = Message::Notice(
                        "Cancelling maneuver".to_string()
                    );
                },

                InputEvent::FtlJump(destination_time_s) => {
                    network.send(ftl_jump(destination_time_s));
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
                    times.update_server_s(game_time_s);
                    frame.game_time_s = Some(game_time_s);
                },
                server::Event::ShipId(ship_id) => {
                    frame.ship_id = Some(ship_id);
                },
                server::Event::UpdateEntity(entity) => {
                    if let Some(body) = entity.body {
                        if let Some(_) = entity.ship {
                            interpolator.update_ship(
                                times.server_last_known_s(),
                                entity.id,
                                body,
                            );

                            if !frame.colliders.contains_key(&entity.id) {
                                frame.colliders.insert(
                                    entity.id,
                                    SphereCollider::new_from_oval(SHIP_SIZE));
                            }
                        }
                    }

                    if let Some(planet) = entity.planet {
                        frame.planets.insert(entity.id, planet);

                        if !frame.colliders.contains_key(&entity.id) {
                            frame.colliders.insert(
                                entity.id,
                                SphereCollider::new_from_oval(planet.radius as f32));
                        }
                    }

                    if let Some(maneuver) = entity.maneuver {
                        frame.maneuvers.insert(entity.id, maneuver.data);
                    }

                    match entity.broadcast {
                        Some(broadcast) => {
                            frame.broadcasts.insert(entity.id, broadcast.message);
                        },
                        None => {
                            frame.broadcasts.remove(&entity.id);
                        }
                    }
                },
                server::Event::RemoveEntity(id) => {
                    frame.broadcasts.remove(&id);
                    frame.maneuvers.remove(&id);

                    interpolator.remove_ship(&id);
                },
            }

            last_server_activity = now;
        }

        frame.ships.clear();
        interpolator.interpolate(
            times.server_interpolated_s(),
            &mut frame.ships,
        );

        if now - last_server_activity > args.net_timeout_s {
            frame.message = Message::Error(
                "Lost connection to server".to_string()
                    );
        }

        network.send(ClientEvent::Privileged(client_event::Privileged::Heartbeat));
    }
}
