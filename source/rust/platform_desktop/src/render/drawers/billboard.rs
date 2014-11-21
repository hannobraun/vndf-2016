use cgmath::{
	FixedArray,
	Vector2,
	Vector3,
};
use gfx::{
	mod,
	DrawState,
};

use render::{
	shaders,
	Graphics,
	Transform,
	Vertex,
};
use render::texture::Texture;

use super::{
	Draw,
	Drawer,
};


#[shader_param(Batch)]
struct Params {
	position   : [f32, ..3],
	transform  : [[f32, ..4], ..4],
	size       : [f32, ..2],
	offset     : [f32, ..2],
	screen_size: [f32, ..2],
	tex        : gfx::shade::TextureParam,
}


pub struct Billboard {
	pub position   : Vector3<f32>,
	pub offset     : Vector2<f32>,
	pub texture    : Texture,
	pub transform  : Transform,
	pub screen_size: Vector2<f32>,
}

impl Draw<Params> for Billboard {
	fn to_params(&self) -> Params {
		Params {
			position   : self.position.into_fixed(),
			transform  : self.transform.into_fixed(),
			size       : self.texture.size.into_fixed(),
			offset     : self.offset.into_fixed(),
			screen_size: self.screen_size.into_fixed(),
			tex        : self.texture.param,
		}
	}
}


pub type BillboardDrawer = Drawer<_ParamsLink, Params>;

pub fn new_drawer(
	graphics: &mut Graphics,
	draw_state: &DrawState
) -> BillboardDrawer {
	let vertices = [
		Vertex::new([ -1.0, -1.0, 0.0 ], [ 0.0, 1.0 ]),
		Vertex::new([  1.0, -1.0, 0.0 ], [ 1.0, 1.0 ]),
		Vertex::new([ -1.0,  1.0, 0.0 ], [ 0.0, 0.0 ]),
		Vertex::new([  1.0,  1.0, 0.0 ], [ 1.0, 0.0 ]),
	];

	Drawer::new(
		graphics,
		draw_state,
		vertices,
		gfx::TriangleStrip,
		shaders::vertex::FIXED_SIZE_BILLBOARD.clone(),
		shaders::fragment::TEXTURE.clone(),
	)
}
