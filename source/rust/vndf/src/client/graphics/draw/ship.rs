use nalgebra::{
    cast,
    Mat4,
    Vec2,
    zero,
};

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

    ships_drawer: ShapeDrawer,
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

            ships_drawer: ShapeDrawer::ship_group(graphics),
        }
    }

    pub fn draw(
        &mut self,
        frame     : &Frame,
        cam_zoom  : f32,
        transforms: &Transforms,
        graphics  : &mut Graphics,
        ) {
        let mut grouped_ships: Vec<Vec<EntityId>> = vec!();

        if cam_zoom > 1.0 { // TODO: group ships and exclude from frame iter below
            check_visual_collision(frame,
                                   &mut grouped_ships,
                                   cam_zoom);
        }
        
        for (ship_id, ship) in &frame.ships {
            if set_contains (&grouped_ships,&ship_id)
                .is_some() { continue }
            
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

        // draw ship groups
        for (i,group) in grouped_ships.iter().enumerate() {
            let first_ship = frame.ships.get(&group[0]).unwrap();
            
            let avg_ship = group_avg(group,frame);
            
            let transform = transforms.symbol_to_screen(cast(first_ship.position));
            
            self.draw_symbol_group(
                frame,
                &grouped_ships,
                transform,
                graphics,
                );

            self.draw_name(
                i as EntityId, // TODO: consider some naming scheme for groups
                transform,
                graphics,
                );

            self.draw_info(
                &avg_ship,
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

    // TODO: draw as a group of three ships
    fn draw_symbol_group(
        &mut self,
        frame    : &Frame,
        set: &Vec<Vec<EntityId>>,
        transform: Mat4<f32>,
        graphics : &mut Graphics,
    ) {
        let mut color = color::Colors::blue_sky();
        if let Some(sid) = frame.ship_id {
            if set_contains(set,&sid).is_some()  {
                color = color::Colors::green_spring();
            }
        }

        self.ships_drawer.draw(
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
                          set: &mut Vec<Vec<EntityId>>,
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
                set_insert_pair(set,(ship_id,ship_id2));
            }
        }
    }
}


/// insert two entities that relate into same group in set
fn set_insert_pair (set: &mut Vec<Vec<EntityId>>,
                    ents: (&EntityId,&EntityId)) {
    if let Some(r) = set_contains(set,ents.0) {
        set[r.0].push(*ents.1);
    }
    else if let Some(r) = set_contains(set,ents.1) {
        set[r.0].push(*ents.0);
    }
    else { // new group
        set.push(vec!(*ents.0,*ents.1));
    }
    
}

/// determines if ship is in grouped sets, returns first group and index
// NOTE: this might be inefficient
fn set_contains (set: &Vec<Vec<EntityId>>,
                 ent: &EntityId) -> Option<(usize,usize)> {
    for (i,g) in set.iter().enumerate() {
        for (j,e) in g.iter().enumerate() {
            if e == ent {
                return Some((i,j))
            }
        }
    }

    None
}

fn group_avg (group: &Vec<EntityId>,
              frame: &Frame) -> Body {
    let mut avg_ship = Body { // TODO: as a Body default creation method
        position:zero(),
        velocity:zero(),
        force:zero(),
        mass:zero(),
    };
    
    for ship in group {
        let body = frame.ships.get(&ship).unwrap();
        avg_ship.position = avg_ship.position + body.position;
        avg_ship.velocity = avg_ship.velocity + body.velocity;
        avg_ship.force = avg_ship.force + body.force;
        avg_ship.mass = avg_ship.mass + body.mass;
    }

    avg_ship.position = avg_ship.position / group.len() as f64;
    avg_ship.velocity = avg_ship.velocity / group.len() as f64;
    avg_ship.force = avg_ship.force / group.len() as f64;
    avg_ship.mass = avg_ship.mass / group.len() as f64;


    avg_ship
}
