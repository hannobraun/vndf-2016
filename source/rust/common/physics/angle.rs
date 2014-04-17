pub struct Radians(pub f64);

impl Add<Radians, Radians> for Radians {
	fn add(&self, &Radians(other): &Radians) -> Radians {
		let &Radians(this) = self;
		Radians(this + other)
	}
}

impl Eq for Radians {
	fn eq(&self, &Radians(other): &Radians) -> bool {
		let &Radians(this) = self;
		this == other
	}
}

impl Ord for Radians {
	fn lt(&self, &Radians(other): &Radians) -> bool {
		let &Radians(this) = self;
		this < other
	}
}
