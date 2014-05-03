use std::f64;
use std::rc::Rc;

use glfw;

use common::io::Input;
use common::physics::Radians;

use components::Control;
use entities::Components;
use io;
use ui::Window;


pub struct InputHandler {
	window    : Rc<Window>,
	last_input: Option<Input>
}

impl InputHandler {
	pub fn new(window: Rc<Window>) -> InputHandler {
		InputHandler {
			window    : window,
			last_input: None
		}
	}
}

impl io::InputHandler for InputHandler {
	fn apply(&mut self, controls: &mut Components<Control>) -> Input {
		self.window.poll_events();

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

		let mut input = match self.last_input {
			Some(input) => input,

			None =>
				Input {
					exit    : false,
					attitude: Radians(0.0),
					send    : false
				}
		};

		input.exit = self.window.should_close();

		input.attitude = input.attitude + Radians(attitude_change);
		while input.attitude > Radians(f64::consts::PI) {
			input.attitude = input.attitude - Radians(f64::consts::PI * 2.0)
		}
		while input.attitude < -Radians(f64::consts::PI) {
			input.attitude = input.attitude + Radians(f64::consts::PI * 2.0)
		}

		input.send = self.window.key_pressed(glfw::KeyEnter);

		self.last_input = Some(input);
		input
	}
}
