use render::drawers::{
	Base,
	Billboard,
	Line,
	NavDisc,
	Planet,
};


pub struct Drawables {
	pub bases     : Vec<Base>,
	pub billboards: Vec<Billboard>,
	pub lines     : Vec<Line>,
	pub nav_discs : Vec<NavDisc>,
	pub planets   : Vec<Planet>,
}

impl Drawables {
	pub fn new() -> Drawables {
		Drawables {
			bases     : Vec::new(),
			billboards: Vec::new(),
			lines     : Vec::new(),
			nav_discs : Vec::new(),
			planets   : Vec::new(),
		}
	}

	pub fn clear(&mut self) {
		self.bases.clear();
		self.billboards.clear();
		self.lines.clear();
		self.nav_discs.clear();
		self.planets.clear();
	}
}
