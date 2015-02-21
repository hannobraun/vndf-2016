use rand::random;


pub fn random_port(min: u16, max: u16) -> u16 {
	random() % (max - min) + min
}
