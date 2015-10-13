use shared::game::data::{
    Body,
    Broadcast,
    EntityId,
    Maneuver,
    Planet,
    Ship,
};


/// This modules includes auto-generated ECS code. As of this writing, the
/// complete generation code resides in `build.rs` in the package's root
/// directory.


include!(concat!(env!("OUT_DIR"), "/entities.rs"));
