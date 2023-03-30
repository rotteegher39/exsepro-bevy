#[derive(Debug)] 
pub struct Module {
    integrity: u8,
}

#[derive(Debug)] 
pub struct Engine {
    throttle: f32,
    max_thrust: f32,
}

#[derive(Debug)] 
pub struct MainFrame {
}

#[derive(Debug)] 
pub struct ArmorFrame {
}

#[derive(Debug)] 
pub enum Room{
    Cuboid{x: f32, y: f32, z:f32},
    Cylinder{r: f32, h: f32},
    Sphere(f32),
}

#[derive(Debug)] 
pub enum Engines {
    Chemical(Module, Engine),
    Ion(Module, Engine),
}

#[derive(Debug)] 
pub enum Controlled {
    PilotSeat(Module),
    Terminal(Module),
}

#[derive(Debug)] 
pub enum Moving {
    LandingGear(Module),
    Door(Module),
    Flap(Module),
    WeaponRetractor(Module),
}




