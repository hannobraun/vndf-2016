use std::f64;

use glfw;

use components::Control;
use entities::Components;
use ui::Window;


pub fn apply_input(window: &Window, controls: &mut Components<Control>) {
	let angular_velocity = 0.1;
	let mut attitude_change = 0.0;

	if window.key_pressed(glfw::KeyLeft) {
		attitude_change += angular_velocity;
	}
	if window.key_pressed(glfw::KeyRight) {
		attitude_change -= angular_velocity;
	}

	for (_, control) in controls.mut_iter() {
		control.attitude += attitude_change;
		while control.attitude > f64::consts::PI {
			control.attitude -= f64::consts::PI * 2.0
		}
		while control.attitude < -f64::consts::PI {
			control.attitude += f64::consts::PI * 2.0
		}
	}
}
