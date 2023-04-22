use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::Read;
use serde::Serialize;
use serde::de::DeserializeOwned;
use ron::de::from_str;
use ron::ser::*;

use bevy::ecs::system::*;
use bevy::prelude::*;

pub mod window_config;

// DEBUG:
    // Input false to is_root_path to keep directory structure when cargo build
    // Input true when cargo build --release to package exec binary
pub const IS_ROOT_PATH: bool = false;

// Serializes and Deserializes contained type 'Containant' -> into/from a RON file (which is an any configurable type inside a Wrapper that is implemented by this trait).
    // Example:
        // struct WindowConfig(Window) - which is a Wrapper for Window struct
        // struct Window - which is a Containant - configrable type that is saved/fetched into/from config RON file. Which is contained inside WindowConfig struct.
pub trait Container: Sized {
    // Wrapper that contains 'Containant'
    type Wrapper: Default;
    // Type that is returned from a fetch. (Type that contained inside Wrapper)
    type Containant: Default + Serialize + DeserializeOwned;
    // Specify how to get Containant from Wrapper
    fn get_containant_from(wrapper: Self::Wrapper) -> Self::Containant;

    // Function to fetch configuration from a file and deserialize it
    fn fetch_containant(path: &str, is_root_path: bool) -> Self::Containant {
        // Get path buffer of the executable for packaging purposes
        let mut path_buf = PathBuf::new();
        // DEBUG:
            // Input false to is_root_path to keep directory structure when compiling debug with cargo build
            // Input true when cargo build --release to package exec binary
        // Get relative to executable path to work 
        if is_root_path {
            path_buf.push(std::env::current_exe().unwrap().parent().unwrap());
        } else { warn!("Using non relative to executable path!!! FOR DEBUG ONLY! Intended to be used with 'cargo run' ");}
        // Push input relative path
        path_buf.push(path);

        // Shadow path with Unwraped to &str PathBuf
        let path: &str = path_buf.to_str().unwrap();

        // Get mutable default containant constructor from default Wrapper
        let mut containant_constructor = Self::get_containant_from(Self::Wrapper::default());
        // Create folder at 'path' if doesn't exist.
        if let Some(parent_dir) = Path::new(path).parent() {
            if !parent_dir.exists() {
                match std::fs::create_dir_all(parent_dir) {
                    Ok(()) => { warn!("Directory at path {:?} does not exist. Creating new ones RECURSIVELY...", parent_dir); },
                    Err(err) => { error!("Failed to create directory at path '{:?}'. Error: {err}", parent_dir);},
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
                Self::serialize_constructor(path, &containant_constructor);
                return containant_constructor
            }
        } 
    }

    // Serialize constructor to the file at 'path'. Wrapper::default().0 is used as constructor by default.
    fn serialize_constructor(path: &str, containant_constructor: &Self::Containant) -> () {
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
    }
    // Deserializes the file and tries to mutate each field of constructor and returns the result
    fn deserialize_mut_constructor(mut opened_file: File, containant_constructor: &mut Self::Containant) -> () {
        let constructor_name = std::any::type_name::<Self::Containant>();

        let mut file_contents = String::new();
        opened_file.read_to_string(&mut file_contents); 
        // Try to deserialize whole file and mutate the containant_constructor to return it with changed values to those of ron_str
        *containant_constructor = match ron::de::from_str(&file_contents){
            Ok(deserialized) => { 
                info!("Deserialized {} loaded successfully", constructor_name);
                deserialized 
            },
            Err(err) => {
                error!("Failed to deserialize whole file... Skipping. The Error: {}", {&err});
                return
            }
        };
    }
} 

