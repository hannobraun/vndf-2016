use std::collections::BTreeMap;

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

    fn interpolate(&self, _time_s: f64) -> Body {
        // TODO: Return interpolated ship
        self.current.1
    }
}
