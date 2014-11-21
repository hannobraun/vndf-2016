use gfx::{
	mod,
	DeviceHelper,
	DrawState,
	Frame,
	ToSlice,
};

use render::{
	Graphics,
	Vertex,
};

use self::base::BaseDrawer;
use self::billboard::BillboardDrawer;
use self::line::LineDrawer;
use self::nav_disc::NavDiscDrawer;
use self::planet::PlanetDrawer;
use self::projected_course::ProjectedCourseDrawer;


pub use self::base::Base;
pub use self::billboard::Billboard;
pub use self::line::Line;
pub use self::nav_disc::NavDisc;
pub use self::planet::Planet;
pub use self::projected_course::ProjectedCourse;


mod base;
mod billboard;
mod line;
mod nav_disc;
mod planet;
mod projected_course;


pub trait Draw<T> {
	fn to_params(&self) -> T;
}


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


struct Drawer<L, T> {
	batch: gfx::batch::RefBatch<L, T>,
}

impl<L, T: gfx::shade::ShaderParam<L>, D: Draw<T>, > Drawer<L, T> {
	pub fn new(
		graphics       : &mut Graphics,
		draw_state     : &DrawState,
		vertices       : &[Vertex],
		primitive      : gfx::PrimitiveType,
		vertex_shader  : gfx::ShaderSource,
		fragment_shader: gfx::ShaderSource,
	) -> Drawer<L, T> {
		let mesh  = graphics.device.create_mesh(vertices);
		let slice = mesh.to_slice(primitive);

		let program = graphics.device
			.link_program(
				vertex_shader,
				fragment_shader,
			)
			.unwrap_or_else(
				|error|
					panic!("error linking program: {}", error)
			);

		let batch = graphics
			.make_batch(
				&program,
				&mesh,
				slice,
				draw_state,
			)
			.unwrap();

		Drawer {
			batch: batch,
		}
	}

	pub fn draw(
		&self,
		graphics: &mut Graphics,
		frame   : &Frame,
		drawable: &D,
	) {
		graphics.draw(
			&self.batch,
			&drawable.to_params(),
			frame,
		);
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
			projected_course::new_drawer(graphics, draw_state);

		Drawers {
			base_drawer     : BaseDrawer::new(graphics, draw_state),
			billboard_drawer: BillboardDrawer::new(graphics, draw_state),
			line_drawer     : line::new_drawer(graphics, draw_state),
			nav_disc_drawer : nav_disc::new_drawer(graphics, draw_state),
			planet_drawer   : planet::new_drawer(graphics, draw_state),

			projected_course_drawer: projected_course_drawer,
		}
	}
}
