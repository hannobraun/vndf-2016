use std::collections::HashMap;

use font::Glyph;
use render::Texture;


pub struct GlyphDrawer {
	pub textures: HashMap<char, (Glyph, Texture)>,
}
