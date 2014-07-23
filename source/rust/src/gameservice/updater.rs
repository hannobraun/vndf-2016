use std::io::timer::sleep;

use gameservice::events::{
	GameEvent,
	Update
};


pub fn init(frame_time_in_ms: u64, game: Sender<GameEvent>) {
	spawn(proc() {
		loop {
			game.send(Update(frame_time_in_ms as f64 / 1000.0));
			sleep(frame_time_in_ms);
		}
	});
}
