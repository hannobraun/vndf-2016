pub struct Args {
	pub headless: bool,
}

impl Args {
	pub fn parse(args: &[String]) -> Args {
		Args {
			headless: args.len() > 1 && args[1] == "--headless".to_string(),
		}
	}
}
