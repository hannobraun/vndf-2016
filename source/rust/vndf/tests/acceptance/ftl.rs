use nalgebra::Vec2;

use vndf::client::interface::InputEvent;
use vndf::server::game::data::Spawner;
use vndf::server::game::initial_state::InitialState;
use vndf::testing::rc;


#[test]
fn players_should_be_able_to_perform_ftl_jumps() {
    let initial_state = InitialState::new()
        .with_spawner(Spawner {
            position: Vec2::new(0.0, 0.0),
            velocity: Vec2::new(1.0, 0.0),
        });

    let     server = rc::Server::start(initial_state);
    let mut client = rc::Client::start(server.port());

    let frame = client.wait_until(|frame| {
        frame.own_ship().is_some()
    });

    let jump_destination_s = frame.game_time_s.unwrap() + 1000000.0;
    client.input(InputEvent::FtlJump(jump_destination_s));

    client.wait_until(|frame| {
        frame.own_ship().unwrap().position.x >= 1000000.0
    });
}
