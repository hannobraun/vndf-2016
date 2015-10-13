use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;


fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let path    = Path::new(&out_dir).join("entities.rs");

    let mut file = File::create(&path).unwrap();

    Entities::new()
        .generate(&mut file)
        .unwrap();
}


struct Entities;

impl Entities {
    fn new() -> Entities {
        Entities
    }

    fn generate<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        try!(writer.write_all(
b"use std::collections::{
    HashMap,
    HashSet,
};


pub type Components<T> = HashMap<EntityId, T>;


#[derive(Debug)]
pub struct Entities {
    next_id: u64,

    pub entities: HashSet<EntityId>,

"
        ));

        try!(writer.write_all(
b"    pub bodies    : Components<Body>,
    pub broadcasts: Components<Broadcast>,
    pub maneuvers : Components<Maneuver>,
    pub planets   : Components<Planet>,
    pub ships     : Components<Ship>,"
        ));

        try!(writer.write_all(
b"
}

impl Entities {
    pub fn new() -> Entities {
        Entities {
            next_id: 0,

            entities: HashSet::new(),

"
        ));

        try!(writer.write_all(
b"            bodies    : HashMap::new(),
            broadcasts: HashMap::new(),
            maneuvers : HashMap::new(),
            planets   : HashMap::new(),
            ships     : HashMap::new(),"

        ));

        try!(writer.write_all(
b"
        }
    }

    pub fn create_entity(&mut self) -> EntityBuilder {
        let id = self.next_id;
        self.next_id += 1;

        self.entities.insert(id);

        EntityBuilder {
            id: id,

"
        ));

        try!(writer.write_all(
b"            bodies    : &mut self.bodies,
            broadcasts: &mut self.broadcasts,
            maneuvers : &mut self.maneuvers,
            planets   : &mut self.planets,
            ships     : &mut self.ships,"
        ));

        try!(writer.write_all(
b"
        }
    }

    pub fn update_entity(&mut self, id: EntityId) -> EntityUpdater {
        EntityUpdater {
            id: id,
"
        ));

        try!(writer.write_all(

b"            bodies    : &mut self.bodies,
            broadcasts: &mut self.broadcasts,
            maneuvers : &mut self.maneuvers,
            planets   : &mut self.planets,
            ships     : &mut self.ships,"
        ));

        try!(writer.write_all(
b"
        }
    }

    pub fn destroy_entity(&mut self, id: &EntityId) {
"
        ));

        try!(writer.write_all(
b"        self.bodies.remove(id);
        self.broadcasts.remove(id);
        self.maneuvers.remove(id);
        self.planets.remove(id);
        self.ships.remove(id);
"
        ));

        try!(writer.write_all(
b"
        self.entities.remove(id);
    }
}


pub struct EntityBuilder<'c> {
    id: EntityId,

"
        ));

        try!(writer.write_all(
b"    bodies    : &'c mut Components<Body>,
    broadcasts: &'c mut Components<Broadcast>,
    maneuvers : &'c mut Components<Maneuver>,
    planets   : &'c mut Components<Planet>,
    ships     : &'c mut Components<Ship>,"
        ));

        try!(writer.write_all(
b"
}

impl<'c> EntityBuilder<'c> {
"
        ));

        try!(writer.write_all(
b"    pub fn with_body(mut self, component: Body) -> EntityBuilder<'c> {
        self.bodies.insert(self.id, component);
        self
    }

    pub fn with_broadcast(mut self, component: Broadcast) -> EntityBuilder<'c> {
        self.broadcasts.insert(self.id, component);
        self
    }

    pub fn with_maneuver(mut self, component: Maneuver) -> EntityBuilder<'c> {
        self.maneuvers.insert(self.id, component);
        self
    }

    pub fn with_planet(mut self, component: Planet) -> EntityBuilder<'c> {
        self.planets.insert(self.id, component);
        self
    }

    pub fn with_ship(mut self, component: Ship) -> EntityBuilder<'c> {
        self.ships.insert(self.id, component);
        self
    }
"
        ));

        try!(writer.write_all(
b"
    pub fn return_id(self) -> EntityId {
        self.id
    }
}


pub struct EntityUpdater<'c> {
    id: EntityId,

"
        ));

        try!(writer.write_all(
b"    bodies    : &'c mut Components<Body>,
    broadcasts: &'c mut Components<Broadcast>,
    maneuvers : &'c mut Components<Maneuver>,
    planets   : &'c mut Components<Planet>,
    ships     : &'c mut Components<Ship>,"
        ));

        try!(writer.write_all(
b"
}

impl<'c> EntityUpdater<'c> {
"
        ));

        try!(writer.write_all(
b"    pub fn add_body(mut self, component: Body) -> EntityUpdater<'c> {
        self.bodies.insert(self.id, component);
        self
    }

    pub fn add_broadcast(mut self, component: Broadcast) -> EntityUpdater<'c> {
        self.broadcasts.insert(self.id, component);
        self
    }

    pub fn add_maneuver(mut self, component: Maneuver) -> EntityUpdater<'c> {
        self.maneuvers.insert(self.id, component);
        self
    }

    pub fn add_planet(mut self, component: Planet) -> EntityUpdater<'c> {
        self.planets.insert(self.id, component);
        self
    }

    pub fn add_ship(mut self, component: Ship) -> EntityUpdater<'c> {
        self.ships.insert(self.id, component);
        self
    }

    pub fn remove_body(mut self) -> EntityUpdater<'c> {
        self.bodies.remove(&self.id);
        self
    }

    pub fn remove_broadcast(mut self) -> EntityUpdater<'c> {
        self.broadcasts.remove(&self.id);
        self
    }

    pub fn remove_maneuver(mut self) -> EntityUpdater<'c> {
        self.maneuvers.remove(&self.id);
        self
    }

    pub fn remove_planet(mut self) -> EntityUpdater<'c> {
        self.planets.remove(&self.id);
        self
    }

    pub fn remove_ship(mut self) -> EntityUpdater<'c> {
        self.ships.remove(&self.id);
        self
    }"
        ));

        try!(writer.write_all(
b"
}"
        ));

        Ok(())
    }
}
