use modparams::*;
use super::*;
mod modparams;


#[derive(Debug, Default)] 
pub struct CraftModules {
    pub moving: Vec<Moving>,
    pub controlled: Vec<Controlled>,
    pub engines: Vec<Engines>,
    pub room: Vec<Room>,
}
impl Default for CraftModel {
    fn default() -> Self {
        Self { modules: CraftModules{
            moving: vec![],
            controlled: vec![],
            engines: vec![],
            room: vec![],
        } }
    }
}
