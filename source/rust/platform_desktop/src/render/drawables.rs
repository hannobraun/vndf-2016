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

	pub fn push_base(&mut self, base: Base) {
		self.bases.push(base);
	}

	pub fn push_billboard(&mut self, billboard: Billboard) {
		self.billboards.push(billboard);
	}

	pub fn push_line(&mut self, line: Line) {
		self.lines.push(line);
	}

	pub fn push_nav_disc(&mut self, nav_disc: NavDisc) {
		self.nav_discs.push(nav_disc);
	}

	pub fn push_planet(&mut self, planet: Planet) {
		self.planets.push(planet);
	}
}
