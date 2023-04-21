use std::any::TypeId;

use bevy::{prelude::*, window::*};
use bevy::math::vec3;
use bevy::reflect::*;

use craft_models::{
*,
CraftTypes::*,
};
use modules::*;
pub mod craft_models;
pub mod modules;

// Main Instance Vessel Information
#[derive(Component, Debug, Reflect)]
pub struct Craft {
    pub craft_typemodel: CraftTypes,
    // pub craft_modelinfo: CraftModel,
}

// Get Craft with easy interface of name and model.
// Fetches info about model from ...]TODO[!...
pub trait CraftInterface<ANYMODEL> {
    fn new_define(name: String, typ: ANYMODEL) -> (Name, Craft);
}
impl CraftInterface<Smodls> for Craft {
    fn new_define(name: String, typ: Smodls) -> (Name, Craft) {
        (
            Name::new(name),
            Craft {
                craft_typemodel: CraftTypes::Small(typ),
                // craft_modelinfo: Default::default()
            },
        )
    }

}
