use bevy::{prelude::*, window::*};
use crate::craft_models::{Cmods::*, Omods::*, Umods::*};
use bevy::math::vec3;
// Plan to add more modules
use modules::*;

pub mod craft_models;
mod modules;


#[derive(Component, Debug)]

// Main Vessel Information
pub struct Craft {
    pub craft_name: String,
    pub craft_typemodel: CraftTypes,
    // pub craft_modules: CraftModules,
}

pub use craft_models::*;

// Main Vessel Types
#[derive(Debug)]
pub enum CraftTypes {
    Chiisai(Cmods),
    Chuu(Umods),
    Ookii(Omods),
}

// Get default Craft
impl Default for Craft {
    fn default() -> Self {
        Self {
            craft_name: "Testos".to_owned(),
            craft_typemodel: CraftTypes::Chiisai(Cmods::Zabuton),
        }
    }
}

// Get Craft by specifying Model and Name
use CraftTypes::{Chiisai, Chuu, Ookii};
pub trait CraftGet {
    const SIZE: CraftTypes;

    fn new_def(name: String, typ: CraftTypes) -> Craft {
        match typ {
            CraftTypes::Chiisai(mods) => Craft {
                craft_name: name,
                craft_typemodel: CraftTypes::Chiisai(mods),
            },
            CraftTypes::Chuu(mods) => Craft {
                craft_name: name,
                craft_typemodel: CraftTypes::Chuu(mods),
            },
            CraftTypes::Ookii(mods) => Craft {
                craft_name: name,
                craft_typemodel: CraftTypes::Ookii(mods),
            },
            _ => Craft::default(),
        }
    }
}
// Make to work later
impl CraftGet for Craft {
    const SIZE: CraftTypes = CraftTypes::Chiisai(todo!());
}


// Player sprite
pub const IMAGE: &str = "sprites/craft.png";

pub fn spawncrafts(
    mut cmd: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let scale: f32 = 0.05;

    cmd.spawn((
        Craft::new_def("CSX001".to_string(), Chiisai(Zabuton)),
        SpriteBundle {
            transform: Transform {
                translation: vec3(0.0, 0.0, 0.0),
                rotation: Quat::from_xyzw(0.0, 0.0, 0.0, 0.0),
                scale: vec3(scale, scale, scale),
            },
            texture: asset_server.load(IMAGE),
            ..default()
        },
    ));
}
