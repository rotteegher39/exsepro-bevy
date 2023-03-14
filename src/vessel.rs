use bevy::prelude::*;

use {
    modules::*,
};

mod modules;
pub mod craft_models;






// Main Vessel Information
#[derive(Component, Debug)]
pub struct Craft {
    pub param: CraftInfo,
}
#[derive(Debug)]
pub struct CraftInfo {
    pub craft_name: String,
    pub craft_typemodel: CraftTypes,
}

pub use craft_models::*;

// Main Vessel Types
#[derive(Debug)]
pub enum CraftTypes
{
    Chiisai(Cmods),
    Chuu(Umods),
    Ookii(Omods),
}

// Get default Craft
impl Default for Craft {
    fn default() -> Self {
        Self { 
            param: CraftInfo {
                craft_name: "Testos".to_owned(), 
                craft_typemodel: CraftTypes::Chiisai(Cmods::Zabuton) }
        }
    }
}

// Get Craft by specifying Model and Name
use CraftTypes::{Chiisai, Chuu, Ookii};
pub trait CraftGet {
    fn new_def(name: String, typ: CraftTypes) -> Craft {
        match typ {
            CraftTypes::Chiisai(mods) => {
                Craft {param: CraftInfo {
                            craft_name: name,
                            craft_typemodel: CraftTypes::Chiisai(mods),
                        }}
            },
            CraftTypes::Chuu(mods) => {
                Craft {param: CraftInfo {
                            craft_name: name,
                            craft_typemodel: CraftTypes::Chuu(mods),
                        }}

            } 
            CraftTypes::Ookii(mods) => {
                Craft {param: CraftInfo {
                            craft_name: name,
                            craft_typemodel: CraftTypes::Ookii(mods),
                        }}

            } 
        }
    }
}
impl CraftGet for Craft {}
