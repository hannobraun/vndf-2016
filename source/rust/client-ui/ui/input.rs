use std::f64;

use glfw;

use common::physics::Body;

use ui::Window;


pub fn apply_input(window: &Window, player: Option<&mut Body>) {
	let angular_velocity = 0.1;
	let mut attitude_change = 0.0;

	if window.key_pressed(glfw::KeyLeft) {
		attitude_change += angular_velocity;
	}
	if window.key_pressed(glfw::KeyRight) {
		attitude_change -= angular_velocity;
	}

	match player {
		Some(ship) => {
			ship.attitude += attitude_change;
			while ship.attitude > f64::consts::PI {
				ship.attitude -= f64::consts::PI * 2.0
			}
			while ship.attitude < -f64::consts::PI {
				ship.attitude += f64::consts::PI * 2.0
			}
		},
		None => ()
	}
}
