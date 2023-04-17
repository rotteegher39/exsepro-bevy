use std::borrow::BorrowMut;
use std::fs::File;
use std::io::{BufReader, Read, BufRead};
use bevy::ecs::query::WorldQuery;
use serde::{Deserialize, Serialize};
use serde::de::{DeserializeOwned, Error};
use ron::de::*;
use ron::ser::*;
use ron::Value;
use bevy::ecs::system::*;

use bevy::app::PluginGroupBuilder;
use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy::window::*;
use bevy::log::LogPlugin;


// Setting the Default for WindowConfig (wrapper config struct for Window struct type)
#[derive(Resource)]
pub struct WindowConfig(Window);

impl Container for WindowConfig { type Containant = Window;}

impl Default for WindowConfig {
    fn default() -> Self {
        WindowConfig(
            Window {
                present_mode: PresentMode::AutoVsync,
                mode: WindowMode::Windowed,
                title: "Extermination Search Protocol".to_string(),
                resize_constraints: WindowResizeConstraints {
                    min_width: 300.0,
                    min_height: 300.0,
                    max_width: 500.0,
                    max_height: 300.0,
                },
                resizable: true,
                decorations: true,
                transparent: false,
                focused: true,
                window_level: WindowLevel::AlwaysOnTop,
                ..Default::default()
            }
        )
    }
}


// Saves and Fetches contained type 'Containant' -> into/from a file (which is an any configurable type inside a wrapper that is implemented by this trait).
    // Example:
    // struct WindowConfig(Window) - which is a Wrapper
    // struct Window - which is a contained configrable type (Containant) that is saved/fetched into/from config file.
pub trait Container: Sized {
    // Type that is returned from a fetch. (Contained value of Wrapper)
    type Containant: Default + Serialize + DeserializeOwned;

    // Function to fetch configuration from a file and deserialize it
    fn fetch_containant(path: &str) -> Self::Containant {
        // Get mutable default containant constructor
        let mut containant_constructor = Self::Containant::default();
        // Try to open the config File and parse it
        match File::open(path) {
            Ok(mut opened_file) => {
                info!("File {} opened successfully", path);
                let mut file_contents = String::new();
                opened_file.read_to_string(&mut file_contents);
                Self::deserialize_mut_containant(&file_contents, &mut containant_constructor);
                return containant_constructor
            },
            // Serializes Containant::default() in case of file error.
            Err(err) => {
                warn!("Failed to open/read '{}' file -> REPLACING WITH DEFAULTS. Error: {}", path, err);
                // Serialize Self::Containant::default() in this Err(err) arm
                Self::serialize_constructor(path, containant_constructor)
            }
        } 
    }
    // Serialize defaults to the file at 'path'
    fn serialize_constructor(path: &str, containant_constructor: Self::Containant) -> Self::Containant {
        // Type name of Containant
        let containant_name = std::any::type_name::<Self::Containant>();
        std::fs::write(
            path,
            ron::ser::to_string_pretty(
                &containant_constructor,
                PrettyConfig::default(),
            )
                .unwrap_or_else(|err| panic!("Failed to serialize/write default() config to '{}' file: {}", path, err)),
        )
            .unwrap_or_else(|err| panic!("Failed to write default() config to '{}' file: {}", path, err));
        info!("File {} replaced with defaults, returning {} expression", path, containant_name);
        return containant_constructor;

    }
    // Deserializes the file and tries to mutate each field of constructor and returns the result
    fn deserialize_mut_containant(ron_str: &str, containant_constructor: &mut Self::Containant) -> () {
        // Try to deserialize whole ron_str and mutate the containant_constructor to return it with changed values to those of ron_str
        *containant_constructor = match ron::de::from_str(ron_str){
            Ok(deserialized) => deserialized,
            Err(err) => {error!("Failed to deserialize whole file... Skipping. The Error: {}", {err}); return}
        };
    }
} 



// Path for window config file.
pub const WINCONF_PATH: &str = "window_config.ron";

// Adds DefaultPlugins and sets Window from fetch_winconfig().
// Should always work, even if config file did not load, it should load default()
pub fn init_window_and_defaultplugins() -> PluginGroupBuilder {
    DefaultPlugins.set(
        WindowPlugin {
            primary_window: Some(WindowConfig::fetch_containant(WINCONF_PATH)),
            exit_condition: ExitCondition::OnAllClosed,
            close_when_requested: true,
        }
        )
        .disable::<LogPlugin>()
}
