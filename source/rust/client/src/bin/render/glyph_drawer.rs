use std::collections::HashMap;

use font::Glyph;
use texture::Texture;


pub struct GlyphDrawer {
	pub textures: HashMap<char, (Glyph, Texture)>,
}
