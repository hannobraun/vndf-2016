use extra::time;

pub fn log(message: &str) {
	let time = time::now().rfc822();
	print!("{:s}  {:s}\n", time, message);
}
