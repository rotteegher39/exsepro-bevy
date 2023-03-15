use modparams::{Controlled, Engines, Moving, Room};
mod modparams;

pub struct CraftModules {
    moving: Vec<Moving>,
    controlled: Vec<Controlled>,
    engines: Vec<Engines>,
    room: Vec<Room>,
}
