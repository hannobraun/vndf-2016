use vndf::client::render::base::ui::{Color,Colorable};

#[test]
fn test_basic() {
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
