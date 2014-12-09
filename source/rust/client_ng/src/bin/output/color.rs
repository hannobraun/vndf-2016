#[deriving(Clone)]
pub enum Color {
	Normal(ColorName),
	Bright(ColorName),
}

#[deriving(Clone)]
pub enum ColorName {
	Black,
	Red,
	Green,
	Yellow,
	Blue,
	Magenta,
	Cyan,
	White,
}
