use std::borrow::BorrowMut;
use std::fs::File;
use std::path::Path;
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

impl Container for WindowConfig {
    type Wrapper = WindowConfig;
    type Containant = Window;
    fn get_containant_from(wrapper: WindowConfig) -> Window {
        wrapper.0
    }
}

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

pub mod window_config;

// Saves and Fetches contained type 'Containant' -> into/from a file (which is an any configurable type inside a wrapper that is implemented by this trait).
    // Example:
    // struct WindowConfig(Window) - which is a Wrapper
    // struct Window - which is a contained configrable type (Containant) that is saved/fetched into/from config file.
pub trait Container: Sized {
    // wrapper
    type Wrapper: Default;
    // Type that is returned from a fetch. (Contained value of Wrapper)
    type Containant: Default + Serialize + DeserializeOwned;
    // Specify how to get Containant from Wrapper
    fn get_containant_from(wrapper: Self::Wrapper) -> Self::Containant;

    // Function to fetch configuration from a file and deserialize it
    fn fetch_containant(path: &str) -> Self::Containant {
        // Get mutable default containant constructor from default Wrapper
        let mut containant_constructor = Self::get_containant_from(Self::Wrapper::default());
        // Create folder at 'path' if doesn't exist.
        if let Some(parent_dir) = Path::new(path).parent() {
            if !parent_dir.exists() {
                match std::fs::create_dir_all(parent_dir) {
                    Ok(()) => { warn!("Directory at path {:?} does not exist. Creating new ones RECURSIVELY...", parent_dir); },
                    Err(err) => { panic!("Failed to create directory at path '{:?}'. Error: {err}", parent_dir);},
                }
            }
        }
        // Try to open the config File and parse it
        match File::open(path) {
            Ok(opened_file) => {
                Self::deserialize_mut_constructor(opened_file, &mut containant_constructor);
                return containant_constructor
            },
            // Serializes Wrapper::default().0 in case of file error.
            Err(err) => {
                warn!("Failed to open/read '{}' file -> REPLACING WITH DEFAULTS. Error: {}", path, err);
                // Serialize Self::Wrapper::default().0 in this Err(err) arm
                Self::serialize_constructor(path, containant_constructor)
            }
        } 
    }
    // Serialize defaults to the file at 'path'
    fn serialize_constructor(path: &str, containant_constructor: Self::Containant) -> Self::Containant {
        // Type name of Containant
        let containant_name = std::any::type_name::<Self::Containant>();
        // Create folder at 'path' if doesn't exist.
        if let Some(parent_dir) = Path::new(path).parent() {
            if !parent_dir.exists() {
                match std::fs::create_dir_all(parent_dir) {
                    Ok(()) => { warn!("Directory at path {:?} does not exist. Creating new ones RECURSIVELY...", parent_dir); },
                    Err(err) => { error!("Failed to create directory at path '{:?}'. Error: {err}", parent_dir);},
                }
            }
        }
        // Write RON file at 'path'
        std::fs::write(
            path,
            ron::ser::to_string_pretty(
                &containant_constructor,
                PrettyConfig::default(),
            )
                .unwrap_or_else(|err| panic!("Failed to serialize/write default() config to '{}' file. \n How the hell did that happen??? -> Error: {}", path, err)),
        )
            .unwrap_or_else(|err| panic!("Failed to write default() config to '{}' file: {}", path, err));
        warn!("File '{}' written/replaced with defaults, returning initial {} expression", path, containant_name);
        return containant_constructor;

    }
    // Deserializes the file and tries to mutate each field of constructor and returns the result
    fn deserialize_mut_constructor(mut opened_file: File, containant_constructor: &mut Self::Containant) -> () {
        let constructor_name = std::any::type_name::<Self::Containant>();

        let mut file_contents = String::new();
        opened_file.read_to_string(&mut file_contents);
        // Try to deserialize whole file and mutate the containant_constructor to return it with changed values to those of ron_str
        *containant_constructor = match ron::de::from_str(&file_contents){
            Ok(deserialized) => { 
                info!("{} opened successfully", constructor_name);
                deserialized 
            },
            Err(err) => {
                error!("Failed to deserialize whole file... Skipping. The Error: {}", {&err});
                return
            }
        };
    }
} 

