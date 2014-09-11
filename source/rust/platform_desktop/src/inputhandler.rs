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
	window  : Rc<Window>,
	attitude: Radians,
	missile : u64,

	camera_angle   : (Radians, Radians),
	camera_distance: f64,
}

impl InputHandler {
	pub fn new(window: Rc<Window>) -> InputHandler {
		let angle = Degrees(45.0).to_radians();

		InputHandler {
			window  : window,
			attitude: Radians(0.0),
			missile : 0,

			camera_angle   : (angle, angle),
			camera_distance: 500.0,
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

		let camera_speed = 0.05;
		if self.window.key_pressed(glfw::KeyA) {
			*self.camera_angle.mut0() = self.camera_angle.val0() - Radians(camera_speed);
		}
		if self.window.key_pressed(glfw::KeyD) {
			*self.camera_angle.mut0() = self.camera_angle.val0() + Radians(camera_speed);
		}
		if self.window.key_pressed(glfw::KeyS) {
			*self.camera_angle.mut1() = self.camera_angle.val1() + Radians(camera_speed);
		}
		if self.window.key_pressed(glfw::KeyW) {
			*self.camera_angle.mut1() = self.camera_angle.val1() - Radians(camera_speed);
		}

		if self.camera_angle.val1() <= Degrees(0.0).to_radians() {
			*self.camera_angle.mut1() = Degrees(1.0).to_radians();
		}
		if self.camera_angle.val1() >= Degrees(180.0).to_radians() {
			*self.camera_angle.mut1() = Degrees(179.0).to_radians();
		}

		let camera_speed = 10.0;
		if self.window.key_pressed(glfw::KeyR) {
			self.camera_distance -= camera_speed;
		}
		if self.window.key_pressed(glfw::KeyF) {
			self.camera_distance += camera_speed;
		}

		if self.camera_distance < 10.0 {
			self.camera_distance = 10.0;
		}
		if self.camera_distance > 600.0 {
			self.camera_distance = 600.0;
		}

		let mut input = Input::default();

		input.exit     = self.window.should_close();
		input.attitude = self.attitude;
		input.missile  = self.missile;

		input.camera_angle    = self.camera_angle;
		input.camera_distance = self.camera_distance;

		input
	}
}
