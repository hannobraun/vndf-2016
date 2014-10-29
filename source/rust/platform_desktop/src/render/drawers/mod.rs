use gfx::{
	DrawState,
	Frame,
};

use render::Graphics;

use self::base_drawer::BaseDrawer;
use self::billboard_drawer::BillboardDrawer;
use self::line_drawer::LineDrawer;
use self::nav_disc_drawer::NavDiscDrawer;
use self::planet_drawer::PlanetDrawer;
use self::projected_course::ProjectedCourseDrawer;


pub use self::base_drawer::Base;
pub use self::billboard_drawer::Billboard;
pub use self::line_drawer::Line;
pub use self::nav_disc_drawer::NavDisc;
pub use self::planet_drawer::Planet;
pub use self::projected_course::ProjectedCourse;


mod base_drawer;
mod billboard_drawer;
mod line_drawer;
mod nav_disc_drawer;
mod planet_drawer;
mod projected_course;


pub enum Drawable {
	IsBase(Base),
	IsBillboard(Billboard),
	IsLine(Line),
	IsNavDisc(NavDisc),
	IsPlanet(Planet),
	IsProjectedCourse(ProjectedCourse),
}


pub struct Drawables {
	pub drawers  : Drawers,
	pub drawables: Vec<Drawable>,
}

impl Drawables {
	pub fn new(graphics: &mut Graphics, draw_state: &DrawState) -> Drawables {
		Drawables {
			drawers  : Drawers::new(graphics, draw_state),
			drawables: Vec::new(),
		}
	}

	pub fn clear(&mut self) {
		self.drawables.clear();
	}

	pub fn push_base(&mut self, base: Base) {
		self.drawables.push(IsBase(base));
	}
	pub fn push_billboard(&mut self, billboard: Billboard) {
		self.drawables.push(IsBillboard(billboard));
	}
	pub fn push_line(&mut self, line: Line) {
		self.drawables.push(IsLine(line));
	}
	pub fn push_nav_disc(&mut self, nav_disc: NavDisc) {
		self.drawables.push(IsNavDisc(nav_disc));
	}
	pub fn push_planet(&mut self, planet: Planet) {
		self.drawables.push(IsPlanet(planet));
	}
	pub fn push_projected_course(&mut self, projected_course: ProjectedCourse) {
		self.drawables.push(IsProjectedCourse(projected_course));
	}

	pub fn draw(&self, graphics: &mut Graphics, frame: &Frame) {
		for drawable in self.drawables.iter() {
			match *drawable {
				IsBase(base) =>
					self.drawers.base_drawer.draw(
						graphics,
						frame,
						&base,
					),
				IsBillboard(billboard) =>
					self.drawers.billboard_drawer.draw(
						graphics,
						frame,
						&billboard,
					),
				IsLine(line) =>
					self.drawers.line_drawer.draw(
						graphics,
						frame,
						&line,
					),
				IsNavDisc(nav_disc) =>
					self.drawers.nav_disc_drawer.draw(
						graphics,
						frame,
						&nav_disc,
					),
				IsPlanet(planet) =>
					self.drawers.planet_drawer.draw(
						graphics,
						frame,
						&planet,
					),
				IsProjectedCourse(projected_course) =>
					self.drawers.projected_course_drawer.draw(
						graphics,
						frame,
						&projected_course,
					),
			}
		}
	}
}


pub struct Drawers {
	pub base_drawer            : BaseDrawer,
	pub billboard_drawer       : BillboardDrawer,
	pub line_drawer            : LineDrawer,
	pub nav_disc_drawer        : NavDiscDrawer,
	pub planet_drawer          : PlanetDrawer,
	pub projected_course_drawer: ProjectedCourseDrawer,
}

impl Drawers {
	pub fn new(graphics: &mut Graphics, draw_state: &DrawState) -> Drawers {
		let projected_course_drawer =
			ProjectedCourseDrawer::new(graphics, draw_state);

		Drawers {
			base_drawer     : BaseDrawer::new(graphics, draw_state),
			billboard_drawer: BillboardDrawer::new(graphics, draw_state),
			line_drawer     : LineDrawer::new(graphics, draw_state),
			nav_disc_drawer : NavDiscDrawer::new(graphics, draw_state),
			planet_drawer   : PlanetDrawer::new(graphics, draw_state),
			projected_course_drawer: projected_course_drawer,
		}
	}
}
