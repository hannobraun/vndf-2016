use collections::HashMap;
use std::ptr;
use std::slice;
use std::str;

use freetype::freetype::{
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
	FT_Set_Pixel_Sizes,
	struct_FT_GlyphSlotRec_};

use ui::Texture;


pub fn load() -> HashMap<~str, Texture> {
	let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

	let mut font = HashMap::new();
	for c in chars.chars() {
		font.insert(str::from_char(c), load_char(c));
	}

	font
}

fn load_char(c: char) -> Texture {
	unsafe {
		let freetype: FT_Library = ptr::null();
		let init_error = FT_Init_FreeType(&freetype);
		assert!(init_error == 0);

		let mut font_face: FT_Face = ptr::null();
		let face_error = FT_New_Face(
				freetype,
				"fonts/amble/Amble-Bold.ttf".as_ptr() as *i8,
				0,
				&mut font_face);
		assert!(face_error == 0);

		let pixel_error = FT_Set_Pixel_Sizes(
			font_face,
			0,
			16);
		assert!(pixel_error == 0);

		let glyph_index = FT_Get_Char_Index(font_face, c as u64);

		let glyph_error = FT_Load_Glyph(
			font_face,
			glyph_index,
			FT_LOAD_DEFAULT as i32);
		assert!(glyph_error == 0);

		let render_error = FT_Render_Glyph(
			(*font_face).glyph as FT_GlyphSlot,
			FT_RENDER_MODE_NORMAL);
		assert!(render_error == 0);

		let bitmap =
			(*(((*font_face).glyph) as *struct_FT_GlyphSlotRec_)).bitmap;

		Texture::new_alpha(
			slice::from_buf(
				bitmap.buffer,
				(bitmap.width * bitmap.rows) as uint),
			bitmap.width as uint,
			bitmap.rows as uint)
	}
}
