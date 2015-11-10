use nalgebra::{
    cast,
    Mat4,
    Vec2,
};

use std::collections::HashSet;

use client::graphics::base::Graphics;
use client::graphics::draw::{
    GlyphDrawer,
    ShapeDrawer,
};
use client::graphics::transforms::Transforms;
use client::interface::Frame;
use shared::color;
use shared::game::data::{
    Body,
    EntityId,
};
use shared::physics::collision::{SphereCollider};


pub struct ShipDrawer {
    ship_size     : f32,
    line_height   : f32,

    symbol_drawer: ShapeDrawer,
    glyph_drawer : GlyphDrawer,
}

impl ShipDrawer {
    pub fn new(
        graphics      : &mut Graphics,
        ship_size     : f32,
        font_size     : f32,
    ) -> ShipDrawer {
        ShipDrawer {
            ship_size     : ship_size,
            line_height   : font_size,

            symbol_drawer: ShapeDrawer::ship(graphics),
            glyph_drawer : GlyphDrawer::new(graphics, font_size as u32),
        }
    }

    pub fn draw(
        &mut self,
        frame     : &Frame,
        cam_zoom  : f32,
        transforms: &Transforms,
        graphics  : &mut Graphics,
        ) {
        let mut grouped_ships: HashSet<EntityId> = HashSet::new();

        if cam_zoom > 1.0 { // TODO: group ships and exclude from frame iter below
            check_visual_collision(frame,
                                   &mut grouped_ships,
                                   cam_zoom);
        }
        
        for (ship_id, ship) in &frame.ships {
            //if grouped_ships.contains(&ship_id) { continue }
            
            let transform = transforms.symbol_to_screen(cast(ship.position));

            if frame.select_ids.contains(ship_id) {
                self.draw_selection(
                    transform,
                    graphics,
                );
            }

            self.draw_symbol(
                frame,
                *ship_id,
                transform,
                graphics,
            );

            self.draw_name(
                *ship_id,
                transform,
                graphics,
            );

            if let Some(broadcast) = frame.broadcasts.get(&ship_id) {
                self.draw_broadcast(
                    broadcast,
                    transform,
                    graphics,
                );
            }

            self.draw_info(
                ship,
                transform,
                graphics,
            );
        }
    }

    fn draw_selection(
        &mut self,
        transform: Mat4<f32>,
        graphics : &mut Graphics,
    ) {
        self.symbol_drawer.draw(
            self.ship_size * 1.25,
            color::Colors::white(),
            transform,
            graphics,
        );
    }

    fn draw_symbol(
        &mut self,
        frame    : &Frame,
        ship_id  : EntityId,
        transform: Mat4<f32>,
        graphics : &mut Graphics,
    ) {
        let mut color = color::Colors::blue();
        if let Some(sid) = frame.ship_id {
            if ship_id == sid  { color = color::Colors::green_spring(); }
        }

        self.symbol_drawer.draw(
            self.ship_size,
            color,
            transform,
            graphics,
        );
    }

    fn draw_name(
        &mut self,
        ship_id  : EntityId,
        transform: Mat4<f32>,
        graphics : &mut Graphics,
    ) {
        self.glyph_drawer.draw(
            &ship_id.to_string(),
            Vec2::new(0.0, self.ship_size * 0.6),
            color::Colors::white(),
            true,
            transform,
            graphics,
        );
    }

    fn draw_broadcast(
        &mut self,
        broadcast: &str,
        transform: Mat4<f32>,
        graphics : &mut Graphics,
    ) {
        self.glyph_drawer.draw(
            broadcast,
            -Vec2::new(0.0, self.ship_size),
            color::Colors::white(),
            true,
            transform,
            graphics,
        );
    }

    fn draw_info(
        &mut self,
        ship     : &Body,
        transform: Mat4<f32>,
        graphics : &mut Graphics,
    ) {
        let offset      = Vec2::new(0.7, 0.3) * self.ship_size;
        let line_offset = Vec2::new(0.0, -self.line_height);

        let position = format!("pos: ({:.2}, {:.2})",
            ship.position.x,
            ship.position.y,
        );
        let velocity = format!("vel: ({:.2}, {:.2})",
            ship.velocity.x,
            ship.velocity.y,
        );

        let mut advance = Vec2::new(0.0, 0.0);

        let mut draw = |text| {
            self.glyph_drawer.draw(
                text,
                offset + advance,
                color::Colors::white(),
                false,
                transform,
                graphics,
            );

            advance = advance + line_offset;
        };

        draw(&position);
        draw(&velocity);
    }
}

fn check_visual_collision(frame: &Frame,
                          set: &mut HashSet<EntityId>,
                          zoom: f32) {
    'ships: for (ship_id,ship_body) in frame.ships.iter() {
        let ship_coll = {
            if let Some (coll) = frame.colliders.get(&ship_id) { coll }
            else { warn!("No collider found for ship {}", ship_id);
                   continue 'ships }
        };

        // check ships colliding into eachother
        'other_ships: for (ship_id2,ship_body2) in frame.ships.iter() {
            if ship_id == ship_id2 { continue 'other_ships }
            
            let ship_coll2 = {
                if let Some (coll) = frame.colliders.get(&ship_id2) { coll }
                else { warn!("No collider found for ship {}", ship_id2);
                       continue 'other_ships }
            };

            if SphereCollider::check_collision_zoomed(
                (&ship_coll,&cast(ship_body.position)),
                (&ship_coll2,&cast(ship_body2.position)),
                zoom) {
                // visual collision made between *ship_id,*ship_id2
                set.insert(*ship_id);
                set.insert(*ship_id2);
            }
        }
    }
}
