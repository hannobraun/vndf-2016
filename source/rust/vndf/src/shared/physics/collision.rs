pub struct Collision {
    points: Vec4<f64>, // bounding box
    kind: CollideKind,
}

use nalgebra::Pnt2;
use client::graphics::SHIP_SIZE;

impl Collison {
    pub fn new (points: [Pnt2;4]) -> Collision {
	Collision { points: points,
	}
    }

    /// builds based on current ship mesh layout (from equilateral triangle)
    pub fn new_from_ship () -> Collision {
	let size = SHIP_SIZE/2.0;
	let p = [Pnt2::new(-0.5,-0.5) * size,
		 Pnt2::new(0.5,-0.5) * size,
		 Pnt2::new(0.5,0.5) * size,
		 Pnt2::new(-0.5,0.5) * size,];
	
	Collision::new(p,CollideKind::Rect)
    }

    pub check_collision (&self, other: &Collision) -> bool {
	false
    }
}

pub enum CollideKind {
    Sphere,
    Rect,
}
