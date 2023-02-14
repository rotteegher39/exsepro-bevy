use std::f32::consts::PI;

use {modparams::*, modules::*};
mod modules;
mod modparams;

pub struct Craft {
    pub global_craft_params: CraftParams,
    pub mainframe: MainFrame,
    pub armorframe: ArmorFrame,
    
}
pub struct CraftParams {
    pub craftname: String,
    pub craftmodel: String,
    pub crafttype: String,
    pub totalmass: f32,
}
