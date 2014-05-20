use std::f64;
use std::rc::Rc;

use glfw;

use common::io;
use common::io::Input;
use common::physics::Radians;

use ui::Window;


pub struct InputHandler {
	window   : Rc<Window>,
	attitude : Radians
}

impl InputHandler {
	pub fn new(window: Rc<Window>) -> InputHandler {
		InputHandler {
			window  : window,
			attitude: Radians(0.0)
		}
	}
}

impl io::InputHandler for InputHandler {
	fn input(&mut self) -> Input {
		self.window.poll_events();

		let angular_velocity = 0.01;
		let mut attitude_change = 0.0;

		if self.window.key_pressed(glfw::KeyLeft) {
			attitude_change += angular_velocity;
		}
		if self.window.key_pressed(glfw::KeyRight) {
			attitude_change -= angular_velocity;
		}

		self.attitude = self.attitude + Radians(attitude_change);
		while self.attitude > Radians(f64::consts::PI) {
			self.attitude = self.attitude - Radians(f64::consts::PI * 2.0)
		}
		while self.attitude < -Radians(f64::consts::PI) {
			self.attitude = self.attitude + Radians(f64::consts::PI * 2.0)
		}

		Input {
			exit    : self.window.should_close(),
			attitude: self.attitude,
			missile : false
		}
	}
}
