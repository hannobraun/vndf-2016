use std::f64;
use std::rc::Rc;

use glfw;

use physics::Radians;
use platform::Input;
use window::Window;


pub struct InputHandler {
	window   : Rc<Window>,
	attitude : Radians,
	missile  : u64
}

impl InputHandler {
	pub fn new(window: Rc<Window>) -> InputHandler {
		InputHandler {
			window  : window,
			attitude: Radians(0.0),
			missile : 0
		}
	}

	pub fn input(&mut self) -> Input {
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

		if self.window.key_pressed(glfw::KeyEnter) {
			self.missile += 1;
		}

		let mut input = Input::default();

		input.exit     = self.window.should_close();
		input.attitude = self.attitude;
		input.missile  = self.missile;

		input
	}
}
