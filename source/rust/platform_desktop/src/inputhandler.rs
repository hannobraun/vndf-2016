use std::f64;
use std::rc::Rc;

use glfw;

use physics::{
	Degrees,
	Radians,
};
use platform::Input;
use window::Window;


pub struct InputHandler {
	window   : Rc<Window>,
	attitude : Radians,
	camera   : (Radians, Radians),
	missile  : u64,
}

impl InputHandler {
	pub fn new(window: Rc<Window>) -> InputHandler {
		InputHandler {
			window  : window,
			attitude: Radians(0.0),
			camera  : (Degrees(45.0).to_radians(), Degrees(45.0).to_radians()),
			missile : 0,
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

		let camera_speed = 1.0;
		if self.window.key_pressed(glfw::KeyA) {
			*self.camera.mut0() = self.camera.val0() - Radians(camera_speed);
		}
		if self.window.key_pressed(glfw::KeyD) {
			*self.camera.mut0() = self.camera.val0() + Radians(camera_speed);
		}
		if self.window.key_pressed(glfw::KeyS) {
			*self.camera.mut1() = self.camera.val1() - Radians(camera_speed);
		}
		if self.window.key_pressed(glfw::KeyW) {
			*self.camera.mut1() = self.camera.val1() + Radians(camera_speed);
		}

		let mut input = Input::default();

		input.exit     = self.window.should_close();
		input.attitude = self.attitude;
		input.camera   = self.camera;
		input.missile  = self.missile;

		input
	}
}
