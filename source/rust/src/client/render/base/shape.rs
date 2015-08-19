use std::f64::consts::PI;

pub struct Shape {
	points: Vec<(f32,f32)>,

	//possible ideas
	//weight: f32, 
	//time: u16,
}

impl Shape {
	pub fn get_points(&self) -> &[(f32,f32)] {
		&self.points
	}
	pub fn poly(points: Vec<(f32, f32)>) -> Shape {
		Shape { points: points }
	}
	pub fn tri(x: f32) -> Shape {
		let a = (0.0-x, 0.0-x);
		let b = (x, 0.0-x);
		let c = (0.0, x);
		let points = vec![a,b,c];
		Shape { points: points }
	}
	pub fn rect(w: f32, h: f32) -> Shape {
		let hw = w / 2.0;
		let hh = h / 2.0;
		let points = vec![(0.0-hw, hh),
						  (0.0-hw, 0.0-hh),
						  (hw, hh),
						  (hw, 0.0-hh)];
		Shape { points: points }
	}
	//FIXME: needs to have a different algorithm for a filled oval
	//pub fn oval(w: f32, h: f32, n: u8) -> Shape {
		/*let t = 2.0 * (PI as f32)  / n as f32;
		let hw = w / 2.0;
		let hh = h / 2.0;
		let f = |i: f32| (hw * (t*i).cos(),
						  hh * (t*i).sin());
		let points = (0..n-1).map(|i| f(i as f32)).collect();
		Shape { points: points }*/
	//}
}
