use std::char;
use std::collections::HashMap;
use std::ptr;
use std::vec;

use cgmath::Vector2;
use freetype::ffi::{
	FT_Face,
	FT_Get_Char_Index,
	FT_GlyphSlot,
	FT_Init_FreeType,
	FT_Library,
	FT_LOAD_DEFAULT,
	FT_Load_Glyph,
	FT_New_Face,
	FT_Render_Glyph,
	FT_RENDER_MODE_NORMAL,
	FT_Set_Pixel_Sizes
};


pub type Font = HashMap<char, Glyph>;

pub struct Glyph {
	pub data   : Vec<u8>,
	pub size   : Vector2<f32>,
	pub offset : Vector2<f32>,
	pub advance: Vector2<f32>,
}


pub fn load() -> Font {
	let     font_face  = init_font_face();
	let mut font       = HashMap::new();

	for n in range(32u, 127) {
		let c = char::from_u32(n as u32).unwrap();

		let glyph_slot = load_glyph_slot(font_face, c);
		let glyph      = make_glyph(glyph_slot);

		font.insert(c, glyph);
	}

	font
}

fn init_font_face() -> FT_Face {
	unsafe {
		let mut freetype: FT_Library = ptr::mut_null();
		let init_error = FT_Init_FreeType(&mut freetype);
		assert!(init_error == 0);

		let mut font_face: FT_Face = ptr::mut_null();
		let face_error = FT_New_Face(
				freetype,
				"fonts/amble/Amble-Regular.ttf".to_c_str().as_ptr(),
				0,
				&mut font_face
		);
		assert!(face_error == 0);

		let pixel_error = FT_Set_Pixel_Sizes(
			font_face,
			0,
			16
		);
		assert!(pixel_error == 0);

		font_face
	}
}

fn load_glyph_slot(font_face: FT_Face, c: char) -> FT_GlyphSlot {
	unsafe {
		let glyph_index = FT_Get_Char_Index(font_face, c as u64);

		let glyph_error = FT_Load_Glyph(
			font_face,
			glyph_index,
			FT_LOAD_DEFAULT as i32
		);
		assert!(glyph_error == 0);

		let render_error = FT_Render_Glyph(
			(*font_face).glyph as FT_GlyphSlot,
			FT_RENDER_MODE_NORMAL
		);
		assert!(render_error == 0);

		(*font_face).glyph as FT_GlyphSlot
	}
}

fn make_glyph(glyph_slot: FT_GlyphSlot) -> Glyph {
	unsafe {
		let bitmap = (*glyph_slot).bitmap;

		Glyph {
			data: vec::raw::from_buf(
				bitmap.buffer as *const u8,
				(bitmap.width * bitmap.rows) as uint
			),
			size: Vector2::new(
				bitmap.width as f32,
				bitmap.rows as f32,
			),
			offset: Vector2::new(
				(*glyph_slot).bitmap_left as f32,
				(*glyph_slot).bitmap_top as f32 - bitmap.rows as f32
			),
			advance: Vector2::new(
				(*glyph_slot).advance.x as f32 / 64.0,
				(*glyph_slot).advance.y as f32 / 64.0
			),
		}
	}
}
