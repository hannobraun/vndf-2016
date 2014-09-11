use std::fmt;
use std::fmt::Formatter;

use super::Vec2;


#[deriving(Clone, Decodable, Encodable, PartialEq, PartialOrd, Show)]
pub struct Radians(pub f64);

impl Radians {
	pub fn from_vec(Vec2(x, y): Vec2) -> Radians {
		Radians(y.atan2(x))
	}

	pub fn to_vec(&self) -> Vec2 {
		let &Radians(this) = self;
		Vec2(
			this.cos(),
			this.sin(),
		)
	}

	pub fn round(&self, precision_in_bits: uint) -> Radians {
		let &Radians(this) = self;
		let factor = (1u << precision_in_bits) as f64;
		Radians((this * factor).floor() / factor)
	}

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


#[deriving(Show)]
pub struct Degrees(pub f64);

impl Degrees {
	pub fn to_radians(&self) -> Radians {
		let &Degrees(this) = self;
		Radians(this.to_radians())
	}
}

impl fmt::Signed for Degrees {
	fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
		let &Degrees(this) = self;
		(this as i64).fmt(formatter)
	}
}
