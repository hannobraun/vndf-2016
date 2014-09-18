use std::f64;
use std::rc::Rc;

use cgmath::{
	deg,
	rad,
	Rad,
	ToRad,
};
use glfw;

use platform::Input;
use window::Window;


pub struct InputHandler {
	window  : Rc<Window>,
	attitude: Rad<f64>,
	missile : u64,

	camera_angle   : (Rad<f64>, Rad<f64>),
	camera_distance: f64,
}

impl InputHandler {
	pub fn new(window: Rc<Window>) -> InputHandler {
		let angle = deg(45.0).to_rad();

		InputHandler {
			window  : window,
			attitude: Rad::zero(),
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

		self.attitude = self.attitude + rad(attitude_change);
		while self.attitude > rad(f64::consts::PI) {
			self.attitude = self.attitude - rad(f64::consts::PI * 2.0)
		}
		while self.attitude < -rad(f64::consts::PI) {
			self.attitude = self.attitude + rad(f64::consts::PI * 2.0)
		}

		if self.window.key_pressed(glfw::KeyEnter) {
			self.missile += 1;
		}

		let camera_speed = 0.05;
		if self.window.key_pressed(glfw::KeyA) {
			*self.camera_angle.mut0() = self.camera_angle.val0() - rad(camera_speed);
		}
		if self.window.key_pressed(glfw::KeyD) {
			*self.camera_angle.mut0() = self.camera_angle.val0() + rad(camera_speed);
		}
		if self.window.key_pressed(glfw::KeyS) {
			*self.camera_angle.mut1() = self.camera_angle.val1() + rad(camera_speed);
		}
		if self.window.key_pressed(glfw::KeyW) {
			*self.camera_angle.mut1() = self.camera_angle.val1() - rad(camera_speed);
		}

		if self.camera_angle.val1() <= deg(0.0).to_rad() {
			*self.camera_angle.mut1() = deg(1.0).to_rad();
		}
		if self.camera_angle.val1() >= deg(180.0).to_rad() {
			*self.camera_angle.mut1() = deg(179.0).to_rad();
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
