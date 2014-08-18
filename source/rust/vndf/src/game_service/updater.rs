use std::io::timer::sleep;
use std::time::Duration;

use game_service::events::{
	GameEvent,
	Update
};


pub fn init(frame_time_in_ms: u64, game: Sender<GameEvent>) {
	spawn(proc() {
		loop {
			game.send(Update(frame_time_in_ms as f64 / 1000.0));
			sleep(Duration::milliseconds(frame_time_in_ms as i32));
		}
	});
}
