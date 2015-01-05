use std::io::{
	self,
	fs,
	File,
};
use std::os;
use std::rand::random;


pub struct Tree {
	root: Path,
}

impl Tree {
	pub fn root(&self) -> &Path {
		&self.root
	}
}


pub struct TreeBuilder {
	root: Path,
}

impl TreeBuilder {
	pub fn new() -> TreeBuilder {
		let root = os::tmpdir()
			.join(
				format!("acceptance-rs-tree-{}", random::<u16>()).as_slice()
			);

		TreeBuilder {
			root: root
		}
	}

	pub fn with_file(self, path: &str, contents: &str) -> TreeBuilder {
		let path = self.root.join(path);

		fs::mkdir_recursive(&path.dir_path(), io::USER_RWX)
			.unwrap_or_else(|e| panic!("failed to create tree root: {}", e));

		File::create(&path)
			.write_str(contents)
			.unwrap_or_else(|e| panic!("failed to write file: {}", e));

		self
	}

	pub fn build(self) -> Tree {
		Tree {
			root: self.root
		}
	}
}
