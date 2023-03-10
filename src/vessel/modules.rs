use std::collections::{hash_map, HashMap};
use modparams::*;
mod modparams;

pub struct CraftModuleTypes {
    moving: HashMap<Moving, u8>,
    controlled: HashMap<Controlled, u8>,
    engines: HashMap<Engines, u8>,
    room: HashMap<Room, u8>,
}
