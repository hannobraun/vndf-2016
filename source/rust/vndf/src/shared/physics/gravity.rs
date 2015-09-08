pub struct Gravity {
    constant: f32, // -9.8 m/s^2 as units/s.squared()
}

impl Gravity {
    fn new (g: f32) -> Gravity {
	Gravity { constant: g }
    }

    /// assumes Frame's ships have a mass equivalent
    //pub fn apply_gravity (&mut Frame);
    
    //pub fn get_gravity (&self, other: &Ship) -> Vec2<f32>;
}

// vector form of gravity
// https://en.wikipedia.org/wiki/Newton%27s_law_of_universal_gravitation#Vector_form
// force is to be applied to object 2
//let f = G * ((m1 * m2)/
//             (r2 - r1).squared()) //* (unit-vector between o1 o2)

// NOTE: might want to look around in nphysics source
