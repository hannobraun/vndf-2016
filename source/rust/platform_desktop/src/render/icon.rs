use cgmath::{
	FixedArray,
	Matrix4,
	Vector,
	Vector2,
};
use gfx::{
	mod,
	Device,
	DeviceHelper,
	Frame,
	ToSlice,
};

use font::Glyph;
use images::Image;

use super::{
	Graphics,
	Transform,
	Vertex,
};


static ICON_VERTEX_SHADER: gfx::ShaderSource = shaders! {
	GLSL_150: b"
		#version 150 core

		uniform mat4 transform;

		in vec3 position;
		in vec2 tex_coord;

		out vec2 tex_coord_f;

		void main()
		{
			gl_Position = transform * vec4(position, 1.0);
			tex_coord_f = tex_coord;
		}
	"
};

static ICON_FRAGMENT_SHADER: gfx::ShaderSource = shaders! {
	GLSL_150: b"
		#version 150 core

		uniform sampler2D tex;

		in vec2 tex_coord_f;

		out vec4 out_color;

		void main()
		{
			out_color = texture(tex, tex_coord_f);
		}
	"
};


#[shader_param(IconBatch)]
struct IconParams {
	transform: [[f32, ..4], ..4],
	tex      : gfx::shade::TextureParam,
}


pub struct Icon {
	pub batch : IconBatch,
	pub param : gfx::shade::TextureParam,
	pub size  : Vector2<f32>,
	pub offset: Vector2<f32>,
}

impl Icon {
	pub fn from_glyph(graphics: &mut Graphics, glyph: &Glyph) -> Icon {
		let data = Vec::from_fn(
			glyph.data.len() * 4,
			|i| {
				if (i + 1) % 4 == 0 {
					glyph.data[i / 4]
				}
				else {
					255
				}
			}
		);

		Icon::new(
			graphics,
			glyph.size[0],
			glyph.size[1],
			data.as_slice(),
			false,
		)
	}

	pub fn from_image(graphics: &mut Graphics, image: Image) -> Icon {
		Icon::new(
			graphics,
			image.width as f32,
			image.height as f32,
			image.data.as_slice(),
			true,
		)
	}

	fn new(
		graphics: &mut Graphics,
		width   : f32,
		height  : f32,
		data    : &[u8],
		center  : bool,
	) -> Icon {
		let vertices = [
			Vertex::for_icon([   0.0,    0.0, 0.0 ], [ 0.0, 1.0 ]),
			Vertex::for_icon([ width,    0.0, 0.0 ], [ 1.0, 1.0 ]),
			Vertex::for_icon([   0.0, height, 0.0 ], [ 0.0, 0.0 ]),
			Vertex::for_icon([ width, height, 0.0 ], [ 1.0, 0.0 ]),
		];

		let mesh  = graphics.device.create_mesh(vertices);
		let slice = mesh.to_slice(gfx::TriangleStrip);

		let program = graphics.device
			.link_program(
				ICON_VERTEX_SHADER.clone(),
				ICON_FRAGMENT_SHADER.clone()
			)
			.unwrap_or_else(|error| fail!("error linking program: {}", error));

		let texture_info = gfx::tex::TextureInfo {
			width : width as u16,
			height: height as u16,
			depth : 1,
			levels: -1,
			kind  : gfx::tex::Texture2D,
			format: gfx::tex::RGBA8,
		};

		let texture = graphics.device.create_texture(texture_info).unwrap();
		graphics.device.update_texture(
			&texture,
			&texture_info.to_image_info(),
			data
		)
		.unwrap();

		let sampler = graphics.device.create_sampler(
			gfx::tex::SamplerInfo::new(
				gfx::tex::Bilinear,
				gfx::tex::Clamp
			)
		);

		let batch = graphics
			.make_batch(
				&program,
				&mesh,
				slice,
				&gfx::DrawState::new().blend(gfx::BlendAlpha)
			)
			.unwrap();

		let size   = Vector2::new(width as f32, height as f32);
		let offset = if center { size.mul_s(-0.5) } else { Vector2::zero() };

		Icon {
			batch : batch,
			param : (texture, Some(sampler)),
			size  : size,
			offset: offset,
		}
	}

	pub fn draw(
		&self,
		graphics : &mut Graphics,
		frame    : &Frame,
		transform: &Transform,
	) {
		let params = IconParams {
			transform: transform.mul(&Matrix4::from_translation(&self.offset.extend(0.0))).into_fixed(),
			tex      : self.param,
		};

		graphics.draw(
			&self.batch,
			&params,
			frame
		);
	}
}
