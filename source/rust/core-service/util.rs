#[no_mangle]
pub extern fn logOutput(c_chars: *::std::libc::c_char) {
	unsafe {
		let c_str = ::std::c_str::CString::new(c_chars, false);
		match c_str.as_str() {
			Some(s) => {
				let t = ::extra::time::now().rfc822();
				print(format!("{:s}  {:s}", t, s));
			},

			None =>
				print("logOutput: Can't print string\n")
		}
	}
}
