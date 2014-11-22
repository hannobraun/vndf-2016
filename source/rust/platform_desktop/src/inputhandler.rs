use std::rc::Rc;

use cgmath::{
	deg,
	rad,
	Quaternion,
	Rad,
	Rotation3,
	ToRad,
};
use glfw::Key;

use platform::Input;
use window::Window;


pub struct InputHandler {
	window  : Rc<Window>,
	attitude: Quaternion<f64>,
	missile : u64,

	thrust    : bool,
	thrust_key: bool,

	camera_angle   : (Rad<f64>, Rad<f64>),
	camera_distance: f64,
}

impl InputHandler {
	pub fn new(window: Rc<Window>) -> InputHandler {
		let angle = deg(45.0).to_rad();

		InputHandler {
			window  : window,
			attitude: Quaternion::identity(),
			missile : 0,

			thrust    : false,
			thrust_key: false,

			camera_angle   : (-angle, angle),
			camera_distance: 10000.0,
		}
	}

	pub fn input(&mut self) -> Input {
		self.window.poll_events();

		let angular_velocity = 0.01;

		let mut attitude_change_z = 0.0;
		if self.window.key_pressed(Key::Left) {
			attitude_change_z += angular_velocity;
		}
		if self.window.key_pressed(Key::Right) {
			attitude_change_z -= angular_velocity;
		}

		let mut attitude_change_y = 0.0;
		if self.window.key_pressed(Key::Up) {
			attitude_change_y += angular_velocity;
		}
		if self.window.key_pressed(Key::Down) {
			attitude_change_y -= angular_velocity;
		}

		let attitude_change_q = Quaternion::identity()
			.mul_q(&Rotation3::from_angle_z(rad(attitude_change_z)))
			.mul_q(&Rotation3::from_angle_y(rad(attitude_change_y)));
		self.attitude = self.attitude.mul_q(&attitude_change_q);


		let thrust_key = self.window.key_pressed(Key::Space);
		if thrust_key && !self.thrust_key {
			self.thrust = !self.thrust;
		}
		self.thrust_key = thrust_key;


		if self.window.key_pressed(Key::Enter) {
			self.missile += 1;
		}

		let camera_speed = 0.05;
		if self.window.key_pressed(Key::A) {
			*self.camera_angle.mut0() = self.camera_angle.val0() - rad(camera_speed);
		}
		if self.window.key_pressed(Key::D) {
			*self.camera_angle.mut0() = self.camera_angle.val0() + rad(camera_speed);
		}
		if self.window.key_pressed(Key::S) {
			*self.camera_angle.mut1() = self.camera_angle.val1() + rad(camera_speed);
		}
		if self.window.key_pressed(Key::W) {
			*self.camera_angle.mut1() = self.camera_angle.val1() - rad(camera_speed);
		}

		if self.camera_angle.val1() <= deg(0.0).to_rad() {
			*self.camera_angle.mut1() = deg(1.0).to_rad();
		}
		if self.camera_angle.val1() >= deg(180.0).to_rad() {
			*self.camera_angle.mut1() = deg(179.0).to_rad();
		}

		let camera_speed = 100.0;
		if self.window.key_pressed(Key::R) {
			self.camera_distance -= camera_speed;
		}
		if self.window.key_pressed(Key::F) {
			self.camera_distance += camera_speed;
		}

		if self.camera_distance < 10.0 {
			self.camera_distance = 10.0;
		}

		let mut input = Input::default();

		input.exit     = self.window.should_close();
		input.attitude = self.attitude;
		input.thrust   = self.thrust;
		input.missile  = self.missile;

		input.camera_angle    = self.camera_angle;
		input.camera_distance = self.camera_distance;

		input
	}
}
