use std::collections::BTreeMap;

use nalgebra::Vec2;

use shared::game::data::{
    Body,
    EntityId,
};


pub struct Interpolator {
    ships: BTreeMap<EntityId, Interpolated>,
}

impl Interpolator {
    pub fn new() -> Self {
        Interpolator {
            ships: BTreeMap::new(),
        }
    }

    pub fn update_ship(&mut self, time_s: f64, id: EntityId, ship: Body) {
        let must_insert = if let Some(interpolated) = self.ships.get_mut(&id) {
            interpolated.update(time_s, ship);
            false
        }
        else {
            true
        };

        // An ugly hack, because lifetimes are lexical.
        if must_insert {
            self.ships.insert(id, Interpolated::new(time_s, ship));
        }
    }

    pub fn remove_ship(&mut self, id: &EntityId) {
        self.ships.remove(id);
    }

    pub fn interpolate(&self, time_s: f64, target: &mut BTreeMap<EntityId, Body>) {
        for (&id, interpolated) in &self.ships {
            target.insert(id, interpolated.interpolate(time_s));
        }
    }
}


struct Interpolated {
    current : (f64, Body),
    previous: Option<(f64, Body)>,
}

impl Interpolated {
    fn new(time_s: f64, current: Body) -> Self {
        Interpolated {
            current : (time_s, current),
            previous: None,
        }
    }

    fn update(&mut self, time_s: f64, ship: Body) {
        self.previous = Some(self.current);
        self.current  = (time_s, ship);
    }

    fn interpolate(&self, time_s: f64) -> Body {
        match self.previous {
            Some((time_1_s, ship_1)) => {
                let (time_2_s, ship_2) = self.current;

                let s = (time_s - time_2_s) / (time_2_s - time_1_s);

                let mut ship = ship_1;
                ship.position = interpolate(
                    ship_1.position,
                    ship_2.position,
                    s,
                );

                ship
            },
            None => {
                self.current.1
            },
        }
    }
}

fn interpolate(v1: Vec2<f64>, v2: Vec2<f64>, s: f64) -> Vec2<f64> {
    v1 + (v2 - v1) * s
}
