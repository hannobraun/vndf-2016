use std::f64;
use std::rc::Rc;

use glfw;

use common::physics::Radians;

use components::Control;
use entities::Components;
use io;
use ui::Window;


pub struct Input {
	window: Rc<Window>
}

impl Input {
	pub fn new(window: Rc<Window>) -> Input {
		Input {
			window: window
		}
	}
}

impl io::Input<Window> for Input {
	fn apply(&self, _: &Window, controls: &mut Components<Control>) {
		let angular_velocity = 0.1;
		let mut attitude_change = 0.0;

		if self.window.key_pressed(glfw::KeyLeft) {
			attitude_change += angular_velocity;
		}
		if self.window.key_pressed(glfw::KeyRight) {
			attitude_change -= angular_velocity;
		}

		for (_, control) in controls.mut_iter() {
			control.attitude = control.attitude + Radians(attitude_change);
			while control.attitude > Radians(f64::consts::PI) {
				control.attitude = control.attitude - Radians(f64::consts::PI * 2.0)
			}
			while control.attitude < -Radians(f64::consts::PI) {
				control.attitude = control.attitude + Radians(f64::consts::PI * 2.0)
			}

			if self.window.key_pressed(glfw::KeyEnter) {
				control.send = true;
			}
		}
	}
}
