use std::collections::HashMap;
use std::hash;

use super::Command;


pub trait CommandKind {
	fn name(&self) -> &'static str;
	fn help(&self) -> &'static str;
	fn parse(
		&self,
		args : Option<&str>,
		kinds: &CommandKinds
	) -> Result<Command, &'static str>;
}

impl PartialEq for CommandKind + 'static {
	fn eq(&self, other: &CommandKind) -> bool {
		self.name().eq(other.name())
	}
}

impl Eq for CommandKind + 'static {}

impl<H: hash::Writer> hash::Hash<H> for CommandKind + 'static {
	fn hash(&self, hasher: &mut H) {
		self.name().hash(hasher)
	}
}


pub struct CommandKinds {
	kinds: HashMap<&'static str, &'static CommandKind + 'static>,
}

impl CommandKinds {
	pub fn new() -> CommandKinds {
		let mut kinds = HashMap::new();

		kinds.insert(HELP.name()          , &HELP           as &CommandKind);
		kinds.insert(BROADCAST.name()     , &BROADCAST      as &CommandKind);
		kinds.insert(STOP_BROADCAST.name(), &STOP_BROADCAST as &CommandKind);

		CommandKinds {
			kinds: kinds,
		}
	}

	pub fn get(&self, name: &str) -> Option<&CommandKind> {
		self.kinds.get(name).map(|kind| *kind)
	}

	pub fn start_with(&self, partial_name: &str) -> Vec<&CommandKind> {
		self.kinds
			.iter()
			.filter(|&(&name, _)|
				name.starts_with(partial_name)
			)
			.map(|(_, &kind)|
				kind
			)
			.collect()
	}
}


static BROADCAST: Broadcast = Broadcast;

struct Broadcast;

impl CommandKind for Broadcast {
	fn name(&self) -> &'static str {
		"broadcast"
	}

	fn help(&self) -> &'static str {
		"broadcast <message> - Broadcast a message into the surrounding space \
		(Example: broadcast This is a broadcasted message.)"
	}

	fn parse(
		&self,
		args: Option<&str>,
		_   : &CommandKinds
	) -> Result<Command, &'static str> {
		let message = match args {
			Some(message) => message,
			None          => return Err("Broadcast message is missing"),
		};

		Ok(Command::Broadcast(message.to_string()))
	}
}


static STOP_BROADCAST: StopBroadcast = StopBroadcast;

struct StopBroadcast;

impl CommandKind for StopBroadcast {
	fn name(&self) -> &'static str {
		"stop-broadcast"
	}

	fn help(&self) -> &'static str {
		"stop-broadcast - Stop the current broadcast (Example: stop-broadcast)"
	}

	fn parse(
		&self,
		args: Option<&str>,
		_   : &CommandKinds
	) -> Result<Command, &'static str> {
		match args {
			Some(_) => return Err("stop-broadcast has no arguments"),
			None    => (),
		}

		Ok(Command::StopBroadcast)
	}
}


static HELP: Help = Help;

struct Help;

impl CommandKind for Help {
	fn name(&self) -> &'static str {
		"help"
	}

	fn help(&self) -> &'static str {
		"help <command> - Learn about a command (Example: help help)"
	}

	fn parse(
		&self,
		args : Option<&str>,
		kinds: &CommandKinds,
	) -> Result<Command, &'static str> {
		let kind = match args {
			Some(kind) => kind,
			None       => "help",
		};

		match kinds.get(kind) {
			Some(kind) => Ok(Command::Help(kind.help())),
			None       => Err("Unknown command"),
		}
	}
}
