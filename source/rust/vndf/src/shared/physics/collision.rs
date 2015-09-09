use nalgebra::Pnt2;
use ncollide::shape::{Ball,Convex};
//use ncollide::bounding_volume::HasBoundingSphere;

use client::graphics::SHIP_SIZE;


pub enum CollideKind {
    Ship(Convex<Pnt2<f32>>),
    Planet(Ball<f32>),
}

pub struct Collider {
    kind: CollideKind,
}

impl Collider {
    pub fn new (kind: CollideKind) -> Collider {
	Collider { kind: kind, }
    }

    /// builds based on current ship mesh layout (from equilateral triangle)
    // TODO: make ship mesh points as public in shapes module
    pub fn new_from_ship (scaling_factor: f32) -> Collider {
	let size = SHIP_SIZE/2.0 * scaling_factor;
	let p = vec![Pnt2::new(-0.5, -0.5) * size,
		     Pnt2::new( 0.5, -0.5) * size,
		     Pnt2::new( 0.0,  0.5) * size,];
	let c = Convex::new(p);
	
	Collider::new(CollideKind::Ship(c))
    }

    pub fn new_from_planet (planet_size: f32, scaling_factor: f32) -> Collider {
	let size = planet_size/2.0 * scaling_factor;
	let b = Ball::new(size);
	
	Collider::new(CollideKind::Planet(b))
    }

    pub fn check_collision (&self, other: &Collider) -> bool {
	let mut is_collide = false;
	match (&self.kind,&other.kind) {
	    (&CollideKind::Ship(ref c1),&CollideKind::Ship(ref c2)) => {

	    },
	    (&CollideKind::Ship(ref c),&CollideKind::Planet(ref b)) => {

	    },
	    _ => { warn!("Unsupported collision types"); }
	}

	is_collide
    }
}
