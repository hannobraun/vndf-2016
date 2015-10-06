use rand::random;


pub fn random_port(min: u16, max: u16) -> u16 {
	let r: u16 = random();
	r % (max - min) + min
}

// TODO: Make Windows-compatible
pub fn random_path() -> String {
	let r: u16 = random();
	format!("/tmp/vndf-test-{}", r)
}
