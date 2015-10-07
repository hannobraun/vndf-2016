use std::env;

use rand::random;


pub fn random_port(min: u16, max: u16) -> u16 {
	let r: u16 = random();
	r % (max - min) + min
}

pub fn random_path() -> String {
	let mut temp_file = env::temp_dir();

	let r: u16 = random();
	temp_file.push(format!("vndf-testing-{}", r));

	temp_file.display().to_string()
}
