use nalgebra::{
    Norm,
    Vec2,
};

use shared::game::data::{
    Body,
    Planet,
};


pub fn apply_gravity(planet: &Planet, body: &mut Body) {
    let g = 6.674e-11; // unit: N * m^2 / kg^2

    let body_to_planet = body.position - planet.position;
    let distance       = body_to_planet.norm();
    let direction      = body_to_planet / distance;

    let force =
        direction * -g * (planet.mass * body.mass) / (distance * distance);

    body.force = body.force + force;
}

pub fn integrate(body: &mut Body, delta_t_s: f64) {
    body.velocity = body.velocity + body.force    * delta_t_s;
    body.position = body.position + body.velocity * delta_t_s;

    body.force = Vec2::new(0.0, 0.0);
}
