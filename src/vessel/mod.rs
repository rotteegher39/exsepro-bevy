use bevy::prelude::*;

use craft_models::{
    *,
    CraftTypes::*,
    Bmodls::*, Mmodls::*, Smodls::*,
};
pub mod craft_models;
pub mod modules;


pub struct CraftPlugin;
impl Plugin for CraftPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, spawncrafts)
            .register_type::<Craft>()
            .register_type::<Smodls>()
            .register_type::<Mmodls>()
            .register_type::<Bmodls>()
            ;
    }
}

// Spawn Crafts with easy interface of name and model.
// Craft::new_define() fetches info about model&name and returns Craft component instance
// of Craft with specified model
pub fn spawncrafts(mut cmd: Commands) {
    cmd.spawn(Craft::new_define("CSX001".to_string(), Zabuton));
    cmd.spawn(Craft::new_define("CSX002".to_string(), Suki));
}

// Main Instance Vessel Information
#[derive(Component, Debug, Reflect)]
pub struct Craft {
    pub craft_typemodel: CraftTypes,
    // pub craft_modelinfo: CraftModel,
}

// Get Craft with easy interface of any name and model.
// Fetches info about model from !!! ...] TODO [... !!! Probably from fetch_containant() deserialization...
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
impl CraftInterface<Mmodls> for Craft {
    fn new_define(name: String, typ: Mmodls) -> (Name, Craft) {
        (
            Name::new(name),
            Craft {
                craft_typemodel: CraftTypes::Medium(typ),
                // craft_modelinfo: Default::default()
            },
        )
    }
}
impl CraftInterface<Bmodls> for Craft {
    fn new_define(name: String, typ: Bmodls) -> (Name, Craft) {
        (
            Name::new(name),
            Craft {
                craft_typemodel: CraftTypes::Big(typ),
                // craft_modelinfo: Default::default()
            },
        )
    }
}
