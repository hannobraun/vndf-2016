pub fn logOutput(message: &str) {
	let time = ::extra::time::now().rfc822();
	print(format!("{:s}  {:s}", time, message));
}
