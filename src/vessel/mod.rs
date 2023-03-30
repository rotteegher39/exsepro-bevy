use bevy::{prelude::*, window::*};
use bevy::math::vec3;

use craft_models::{
*,
CraftTypes::*,
};
use modules::*;
pub mod craft_models;
pub mod modules;

#[derive(Component, Debug)]
// Main Instance Vessel Information
pub struct Craft {
    pub craft_name: String,
    pub craft_typemodel: CraftTypes,
    pub craft_modelinfo: CraftModel,
}

// Get Craft with easy interface of name and model.
// Fetches info about model from ...]TODO[!...
pub trait CraftInterface<ANYMODEL> {
    fn new_define(name: String, typ: ANYMODEL) -> Craft;
}
impl CraftInterface<Smodls> for Craft {
    fn new_define(name: String, typ: Smodls) -> Craft {
       Craft {
           craft_name: name,
           craft_typemodel: CraftTypes::Small(typ),
           craft_modelinfo: Default::default()
       } 
    }

}
