use gfx;
use gfx::render::mesh::VertexFormat;
use gfx::render::shade::ShaderParam;
use gfx::traits::*;
use gfx_device_gl as gl;

use client::render::base::Graphics;


pub struct Batch<P: ShaderParam> {
	pub batch: gfx::batch::Core<P>,
	pub slice: gfx::Slice<gl::Resources>,
}

impl<P: ShaderParam<Resources=gl::Resources>> Batch<P> {
	pub fn new<V: VertexFormat>(
		graphics    : &mut Graphics,
		vertex_src  : &[u8],
		fragment_src: &[u8],
		mesh        : &[V],
	) -> Batch<P> {
		let program = graphics.factory
			.link_program(vertex_src, fragment_src)
			.unwrap_or_else(|e| panic!("Error linking program: {:?}", e));

		let mesh  = graphics.factory.create_mesh(mesh);
		let slice = mesh.to_slice(gfx::PrimitiveType::TriangleStrip);

		let batch = gfx::batch::Core::new(mesh, program)
			.unwrap_or_else(|e| panic!("Error making batch: {:?}", e));

		Batch {
			batch: batch,
			slice: slice,
		}
	}
}
