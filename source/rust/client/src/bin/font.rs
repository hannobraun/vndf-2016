use std::ffi::CString;
use std::ptr;

use nalgebra::Vec2;
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


pub struct Font {
	pub font_face: FT_Face,
	pub size     : u32,
}

impl Font {
	pub fn load(size: u32) -> Font {
		Font {
			font_face: init_font_face(),
			size     : size,
		}
	}

	pub fn glyph(&self, c: char) -> Glyph {
		make_glyph(load_glyph_slot(self.font_face, c, self.size))
	}
}


pub struct Glyph {
	pub data   : Vec<u8>,
	pub size   : Vec2<f32>,
	pub offset : Vec2<f32>,
	pub advance: Vec2<f32>,
}


fn init_font_face() -> FT_Face {
	unsafe {
		let mut freetype: FT_Library = ptr::null_mut();
		let init_error = FT_Init_FreeType(&mut freetype);
		assert!(init_error == 0);

		let mut font_face: FT_Face = ptr::null_mut();
		let face_error = FT_New_Face(
				freetype,
				CString::new(b"source/assets/UbuntuMono-B.ttf")
					.unwrap_or_else(|e| panic!("Error creating CString: {}", e))
					.as_ptr(),
				0,
				&mut font_face
		);
		assert!(face_error == 0);

		font_face
	}
}

fn load_glyph_slot(font_face: FT_Face, c: char, size: u32) -> FT_GlyphSlot {
	unsafe {
		let pixel_error = FT_Set_Pixel_Sizes(
			font_face,
			0,
			size,
		);
		assert!(pixel_error == 0);

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
		let ref bitmap = (*glyph_slot).bitmap;

		Glyph {
			data: Vec::from_raw_buf(
				bitmap.buffer as *mut u8,
				(bitmap.width * bitmap.rows) as usize,
			),
			size: Vec2::new(
				bitmap.width as f32,
				bitmap.rows as f32,
			),
			offset: Vec2::new(
				(*glyph_slot).bitmap_left as f32,
				(*glyph_slot).bitmap_top as f32 - bitmap.rows as f32
			),
			advance: Vec2::new(
				(*glyph_slot).advance.x as f32 / 64.0,
				(*glyph_slot).advance.y as f32 / 64.0
			),
		}
	}
}
