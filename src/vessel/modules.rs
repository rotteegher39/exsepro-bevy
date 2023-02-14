use std::collections::{hash_map, HashMap};

use crate::vessel::modparams::*;



pub struct CraftModules {
    moving: Vec<MovingMods>,
    control: Vec<ControlMods>,
    engines: Vec<EngineTypes>,
    room: HashMap<RoomTypes, u8>,
}

enum RoomTypes{
    Cuboid{x: f32, y: f32, z:f32},
    Cylinder{r: f32, h: f32},
    Sphere(f32)
}

enum EngineTypes {
    Chemical(Module, Engine),
    Ion(Module, Engine),
}



struct Engine {
    throttle: f32,
    max_thrust: f32,
}



enum ControlMods {
    PilotSeat(Module),
    Terminal(Module),
}

enum MovingMods {
    LandingGear(Module),
    Door(Module),
    Flap(Module),
    WeaponRetractor(Module),
}









pub struct MainFrame {

}
pub struct ArmorFrame {
}
