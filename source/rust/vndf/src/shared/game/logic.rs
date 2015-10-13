use nalgebra::Vec2;

use shared::game::data::Body;


pub fn integrate(body: &mut Body, delta_t_s: f64) {
    body.velocity = body.velocity + body.force    * delta_t_s;
    body.position = body.position + body.velocity * delta_t_s;

    body.force = Vec2::new(0.0, 0.0);
}
