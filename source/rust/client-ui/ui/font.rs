use collections::HashMap;
use std::char;
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
	FT_Set_Pixel_Sizes};

use ui::{Texture, Textures};


pub struct Font {
	glyphs: HashMap<char, Glyph>
}

pub struct Glyph {
	texture_id: ~str
}


impl Font {
	pub fn load(textures: &mut Textures) -> Font {
		let font_face  = init_font_face();
		let mut glyphs = HashMap::new();

		for n in range(32, 127) {
			let c = char::from_u32(n as u32).unwrap();

			let texture_id = "char:" + str::from_char(c);
			let glyph_slot = load_glyph_slot(font_face, c);
			let texture    = make_texture(glyph_slot);
			let glyph      = Glyph { texture_id: texture_id.clone() };

			textures.add(texture_id, texture);
			glyphs.insert(c, glyph);
		}

		Font {
			glyphs: glyphs
		}
	}

	pub fn get<'a>(&'a self, c: char) -> &'a Glyph {
		self.glyphs.get(&c)
	}
}


fn init_font_face() -> FT_Face {
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

		font_face
	}
}

fn load_glyph_slot(font_face: FT_Face, c: char) -> FT_GlyphSlot {
	unsafe {
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

		(*font_face).glyph as FT_GlyphSlot
	}
}

fn make_texture(glyph_slot: FT_GlyphSlot) -> Texture {
	unsafe {
		let bitmap = (*glyph_slot).bitmap;

		Texture::new_alpha(
			slice::from_buf(
				bitmap.buffer,
				(bitmap.width * bitmap.rows) as uint),
			bitmap.width as uint,
			bitmap.rows as uint)
	}
}
