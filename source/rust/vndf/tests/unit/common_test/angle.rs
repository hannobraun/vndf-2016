use std::f64;

use vndf::common::physics::{Radians, Vec2};


#[test]
fn it_should_convert_from_a_vector(){
	assert_eq!(
		Radians(0.0),
		Radians::from_vec(Vec2(1.0, 0.0)));
	assert_eq!(
		Radians(f64::consts::PI / 2.0),
		Radians::from_vec(Vec2(0.0, 1.0)));
}

#[test]
fn it_should_round_an_angle(){
	assert_eq!(Radians(0.25), Radians(0.25).round(2));
	assert_eq!(Radians(0.25), Radians(0.375).round(2));
}
