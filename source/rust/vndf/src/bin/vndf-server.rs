#![cfg_attr(test, allow(dead_code))]


extern crate env_logger;
#[macro_use]
extern crate log;
extern crate time;
extern crate nalgebra;

extern crate vndf;


use std::env;
use std::thread::sleep_ms;

use time::precise_time_s;

use nalgebra::cast;

use vndf::server::args::Args;
use vndf::server::clients::Clients;
use vndf::server::game::state::GameState;
use vndf::server::incoming_events::IncomingEvents;
use vndf::server::network::Network;
use vndf::server::outgoing_events::{
    OutgoingEvents,
    Recipients,
};
use vndf::shared::protocol::server::Event as ServerEvent;

use vndf::shared::physics::SphereCollider;

fn main() {
    env_logger::init().unwrap_or_else(|e|
                                      panic!("Error initializing logger: {}", e)
                                      );

    let args = Args::parse(env::args());

    let mut game_state = GameState::new();
    let mut clients    = Clients::new();
    let mut network    = Network::new(args.port);
    
    let planets = game_state.load_state();

    info!("Listening on port {}", args.port);

    let mut incoming_events = IncomingEvents::new();
    let mut outgoing_events = OutgoingEvents::new();

    
    
    loop {
        trace!("Start server main loop iteration");

        let now_s = precise_time_s();

        incoming_events.receive(network.receive());
        incoming_events.handle(
            now_s,
            &mut clients,
            &mut game_state,
            &mut outgoing_events,
            );

        clients.remove_inactive(now_s, args.client_timeout_s, |client| {
            outgoing_events.push(
                ServerEvent::RemoveEntity(client.ship_id),
                Recipients::All,
                );
            game_state.on_leave(&client.ship_id);
        });

        game_state.on_update(now_s);

        for ent in game_state.export_entities() {
            outgoing_events.push(
                ServerEvent::UpdateEntity(ent),
                Recipients::All,
                )
        }

        // check collisions
        let entities = game_state.get_entities();
        'ships: for (ship_id,ship_body) in entities.bodies.iter() {
            // check only from the perspective of a ship
            if entities.ships.get(&ship_id).is_none() {
                continue 'ships
            }

            let ship_coll = {
                if let Some (coll) = entities.colliders.get(&ship_id) {
                    coll
                }
                else {
                    warn!("No collider found for ship {}", ship_id);
                    continue 'ships
                }
            };

            // check ship collisions with planets
            'planets: for planet_id in planets.iter() {
                let planet_coll = {
                    if let Some (coll) = entities.colliders.get(&planet_id) {
                        coll
                    }
                    else {
                        warn!("No collider found for planet {}", planet_id);
                        continue 'planets
                    }
                };
                let planet_body = {
                    if let Some (body) = entities.bodies.get(&planet_id) {
                        body
                    }
                    else {
                        warn!("No body found for planet {}", planet_id);
                        continue 'planets
                    }
                };

                if SphereCollider::check_collision((ship_coll,&cast(ship_body.position)),
                                                   (planet_coll,&cast(planet_body.position))) {
                    outgoing_events.push(
                        ServerEvent::Collision(*ship_id,*planet_id),
                        Recipients::All);
                }
            }

            // check ship collisions with eachother
            'other_ships: for (ship_id2,ship_body2) in entities.bodies.iter() {
                if ship_id == ship_id2 {
                    continue 'other_ships
                }

                if entities.ships.get(&ship_id).is_none() {
                    continue 'other_ships
                }

                let ship_coll2 = {
                    if let Some (coll) = entities.colliders.get(&ship_id2) {
                        coll
                    }
                    else {
                        warn!("No collider found for ship {}", ship_id2);
                        continue 'other_ships
                    }
                };
                if SphereCollider::check_collision((ship_coll,&cast(ship_body.position)),
                                                   (ship_coll2,&cast(ship_body2.position))) {
                    outgoing_events.push(
                        ServerEvent::Collision(*ship_id,*ship_id2),
                        Recipients::All);
                }
            }
        }

        outgoing_events.push(ServerEvent::Heartbeat(now_s), Recipients::All);
        outgoing_events.send(&mut clients, &mut network);

        // TODO(1oL33ljB): While physics will generally need to happen on a
        //                 fixed interval, there's not really a reason to delay
        //                 other kinds of logic by sleeping. For example,
        //                 broadcasts can be handled immediately.
        sleep_ms(args.sleep_ms);
    }
}
