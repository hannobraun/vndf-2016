use std::io::timer::sleep;
use std::time::Duration;


fn main() {
	let mut i = 0u8;

	loop {
		print!("{}\n", i);

		i += 1;
		sleep(Duration::milliseconds(200));
	}
}
