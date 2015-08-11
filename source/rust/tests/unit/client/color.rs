use vndf::client::render::base::color::{Color, Colorable, Colors};

#[test]
fn conversion() {
	let hex_col = "452945";
	let byte_col = vec![69u8,41,69];
	
	let c = Color::from_bytes(&byte_col);
	assert_eq!(c.to_bytes().clone(),byte_col);
	
	let _c = Color::from_hex(hex_col).unwrap();
	assert_eq!(_c.to_hex(),hex_col.to_string());
	
	assert_eq!(c,_c);
}

#[test]
fn smoke() {
	let hex_col = "F5F5F5";
	Color::from_hex(hex_col).unwrap();
}

#[test]
fn adding() {
	let c =	Colors::red().add(Colors::green());
	assert_eq!(c,Colors::yellow());

	assert_eq!(c.add(Colors::blue()),Colors::white());
}
