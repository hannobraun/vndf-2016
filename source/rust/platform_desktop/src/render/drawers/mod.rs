use gfx::{
	DrawState,
	Frame,
};

use render::Graphics;


pub use self::base_drawer::{
	Base,
	BaseDrawer,
};
pub use self::billboard_drawer::{
	Billboard,
	BillboardDrawer,
};
pub use self::line_drawer::{
	Line,
	LineDrawer,
};
pub use self::nav_disc_drawer::{
	NavDisc,
	NavDiscDrawer,
};
pub use self::planet_drawer::{
	Planet,
	PlanetDrawer,
};


mod base_drawer;
mod billboard_drawer;
mod line_drawer;
mod nav_disc_drawer;
mod planet_drawer;


pub trait Drawer<T> {
	fn new(graphics: &mut Graphics, draw_state: &DrawState) -> Self;
	fn draw(&self, graphics: &mut Graphics, frame: &Frame, drawable: &T);
}


pub struct Drawables {
	pub drawers: Drawers,

	pub bases     : Vec<Base>,
	pub billboards: Vec<Billboard>,
	pub lines     : Vec<Line>,
	pub nav_discs : Vec<NavDisc>,
	pub planets   : Vec<Planet>,
}

impl Drawables {
	pub fn new(graphics: &mut Graphics, draw_state: &DrawState) -> Drawables {
		Drawables {
			drawers: Drawers::new(graphics, draw_state),

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


pub struct Drawers {
	pub base_drawer     : BaseDrawer,
	pub billboard_drawer: BillboardDrawer,
	pub line_drawer     : LineDrawer,
	pub planet_drawer   : PlanetDrawer,
	pub nav_disc_drawer : NavDiscDrawer,
}

impl Drawers {
	pub fn new(graphics: &mut Graphics, draw_state: &DrawState) -> Drawers {
		Drawers {
			base_drawer     : Drawer::new(graphics, draw_state),
			billboard_drawer: Drawer::new(graphics, draw_state),
			line_drawer     : Drawer::new(graphics, draw_state),
			planet_drawer   : Drawer::new(graphics, draw_state),
			nav_disc_drawer : Drawer::new(graphics, draw_state),
		}
	}
}
