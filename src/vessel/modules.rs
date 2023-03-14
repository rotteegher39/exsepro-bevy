use modparams::*;
use std::collections::{hash_map, HashMap};
mod modparams;

pub struct CraftModules {
    moving: HashMap<Moving, u8>,
    controlled: HashMap<Controlled, u8>,
    engines: HashMap<Engines, u8>,
    room: HashMap<Room, u8>,
}
