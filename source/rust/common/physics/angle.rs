use std::fmt;
use std::fmt::Formatter;


#[deriving(Eq, Show)]
pub struct Radians(pub f64);

impl Radians {
	pub fn degrees(&self) -> Degrees {
		let &Radians(this) = self;
		Degrees(this.to_degrees())
	}
}

impl Add<Radians, Radians> for Radians {
	fn add(&self, &Radians(other): &Radians) -> Radians {
		let &Radians(this) = self;
		Radians(this + other)
	}
}

impl Sub<Radians, Radians> for Radians {
	fn sub(&self, &Radians(other): &Radians) -> Radians {
		let &Radians(this) = self;
		Radians(this - other)
	}
}

impl Neg<Radians> for Radians {
	fn neg(&self) -> Radians {
		let &Radians(this) = self;
		Radians(-this)
	}
}

impl Ord for Radians {
	fn lt(&self, &Radians(other): &Radians) -> bool {
		let &Radians(this) = self;
		this < other
	}
}


pub struct Degrees(pub f64);

impl fmt::Signed for Degrees {
	fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
		let &Degrees(this) = self;
		(this as i64).fmt(formatter)
	}
}
