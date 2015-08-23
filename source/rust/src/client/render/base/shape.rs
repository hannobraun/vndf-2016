use std::f64::consts::PI;

#[derive(Debug)]
pub struct Shape {
	points: Vec<(f32,f32)>,

	//TODO: impl shape kind enum
	//kind: ShapeKind,
	
	//possible ideas
	//weight: f32, 
	//time: u16,
}

impl Shape {
	pub fn get_points(&self) -> &[(f32,f32)] {
		&self.points
	}
	pub fn get_mut_points(&mut self) -> &mut [(f32,f32)] {
		&mut self.points
	}
	
	pub fn new(points: Vec<(f32, f32)>) -> Shape {
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
	pub fn oval(w: f32, h: f32, n: u8) -> Shape {
		let t = 2.0 * (PI as f32) / n as f32;
		let hw = w / 2.0;
		let hh = h / 2.0;
		
		let mut points: Vec<(f32,f32)> = vec!();
		for i in (0..n+1) {
			points.push((0.0,0.0));
			points.push((hw * (t*i as f32).cos(),
						 hh * (t*i as f32).sin()));
		}
		
		Shape { points: points }
	}
	pub fn line(s:[f32;2], e: [f32;2], w: f32) -> Shape {
		let dx = e[0]-s[0];
		let dy = e[1]-s[1];
		let length = (dx*dx + dy*dy).sqrt();
		let px = 1.0 * w * (dy/length);
		let py = 1.0 * w * (dx/length);
		
		Shape::new(vec!(
			(e[0]-px,e[1]+py),
			(s[0]-px,s[1]+py),
			(e[0]+px,e[1]-py),				
			(s[0]+px,s[1]-py),
			))
	}
	pub fn lines(v: Vec<([f32;2])>, w: f32, formed: bool) -> Vec<Shape> {
		let len = v.len();
		let mut lines = vec!();

		lines.push(Shape::line(v[0],v[1],w));

		for n in (1..len) {
			let mut line = Shape::line(v[n-1],v[n],w);
			
			if formed {
				let p = line.get_mut_points();
				p[1] = lines[n-1].get_points()[2]; // set vertex to match
				p[3] = lines[n-1].get_points()[0];
			}

			lines.push(line);
		}

		lines
	}
}
