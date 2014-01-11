pub fn log(message: &str) {
	let time = ::extra::time::now().rfc822();
	print(format!("{:s}  {:s}\n", time, message));
}
