pub struct Module {
    integrity: u8,
}

pub struct Engine {
    throttle: f32,
    max_thrust: f32,
}

pub struct MainFrame {
        
}
pub struct ArmorFrame {
}

pub enum Room{
    Cuboid{x: f32, y: f32, z:f32},
    Cylinder{r: f32, h: f32},
    Sphere(f32)
}

pub enum Engines {
    Chemical(Module, Engine),
    Ion(Module, Engine),
}

pub enum Controlled {
    PilotSeat(Module),
    Terminal(Module),
}

pub enum Moving {
    LandingGear(Module),
    Door(Module),
    Flap(Module),
    WeaponRetractor(Module),
}




