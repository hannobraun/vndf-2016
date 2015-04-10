use gfx;
use gfx::render::mesh::VertexFormat;
use gfx::render::shade::ShaderParam;
use gfx::traits::*;
use gfx_device_gl as gl;

use render::base::Graphics;


pub struct Batch<P: ShaderParam> {
	pub batch: gfx::batch::CoreBatch<P>,
	pub slice: gfx::Slice<gl::Resources>,
}

impl<P: ShaderParam<Resources=gl::Resources>> Batch<P> {
	pub fn new<V: Copy + VertexFormat>(
		graphics    : &mut Graphics,
		vertex_src  : &[u8],
		fragment_src: &[u8],
		mesh        : &[V],
	) -> Batch<P> {
		let program = graphics.graphics.factory
			.link_program(vertex_src, fragment_src)
			.unwrap_or_else(|e| panic!("Error linking program: {:?}", e));

		let mesh = graphics.graphics.factory.create_mesh(mesh);

		let batch = graphics.graphics
			.make_core(
				&program,
				&mesh,
				&gfx::DrawState::new().blend(gfx::BlendPreset::Alpha),
			)
			.unwrap_or_else(|e| panic!("Error making batch: {:?}", e));

		let slice = mesh.to_slice(gfx::PrimitiveType::TriangleStrip);

		Batch {
			batch: batch,
			slice: slice,
		}
	}
}
