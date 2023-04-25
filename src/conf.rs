use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::Read;
use serde::Serialize;
use serde::de::DeserializeOwned;
use ron::de::from_str;
use ron::ser::*;
use std::io::Error;

use bevy::ecs::system::*;
use bevy::prelude::*;

pub mod window_config;

// DEBUG:
    // When true -> serializes all files relative to binary's path
    // Example:
        // if binary executed from ~ and binary is at some/folder/binary_file
            // all files would be serialized inside some/folder/

    // When false -> serializes all files relative to execution path. (to pwd folder)
    // Example:
        // if binary executed from ~ and binary is at some/folder/binary_file
            // all files would be serialized at ~
// when using cargo run while IS_DEBUG_PATH = true it will serialize everything into 'target/debug/' or to 'target/release/' when cargo run --release

// just use use false when cargo run
// and use true when compiling with --release
pub const IS_DEBUG_PATH: bool = false;

// Serializes and Deserializes contained type 'Containant' -> into/from a RON file (which is an any configurable type inside a Wrapper that is implemented by this trait).
    // Example:
        // struct WindowConfig(Window) - which is a Wrapper for Window struct
        // struct Window - which is a Containant - configrable type that is saved/fetched into/from config RON file. Which is contained inside WindowConfig struct.
            // Example:
                    // pub struct WindowConfig(Window);
                    //
                    // impl Container for WindowConfig {
                    //     type Wrapper = WindowConfig;
                    //     type Containant = Window;
                    //     fn unwrap_containant_from(wrapper: WindowConfig) -> Window {
                    //         wrapper.0
                    //     }
                    // }
pub trait Container: Sized {
    // Wrapper that contains 'Containant'
    type Wrapper: Default;
    // Type that is returned from a fetch. (Type that contained inside Wrapper)
    type Containant: Default + Serialize + DeserializeOwned;
    // Specify how to get Containant from Wrapper
    fn unwrap_containant_from(wrapper: Self::Wrapper) -> Self::Containant;

    // Function to fetch configuration from a file and deserialize it
    fn fetch_containant(path: &str) -> Self::Containant {

        // Get mutable default containant constructor from default Wrapper
            // Uses default() values of Wrapper. Separete from Containant::default().
            // This way separete from Containant different default() values can be implemented for every impl Default for AnyDifferentWrapper(Containant)
                // This allowes different separete default() for Containant via different wrappers and their default() implementations.
                // Each wrapper creates a separete default() for itself and it translates onto Containant.
        // Example:
            // impl Default for FirstWindowConfig(Window) {...}
            // impl Default for SecondWindowConfig(Window) {...}
                // impl Container for FirstWindowConfig
                // impl Container for SecondWindowConfig
            // FirstWindowConfig::serialize_constructor('some/path/first_default_config.ron', FirstWindowConfig::default())
            // FirstWindowConfig::serialize_constructor('some/path/second_default_config.ron', SecondWindowConfig::default())
        // Different defaults for same Containant type.
        let mut containant_constructor = Self::unwrap_containant_from(Self::Wrapper::default());

        // Try to open the config File and parse it
        match File::open(
            Self::check_path(path, IS_DEBUG_PATH).to_str().unwrap()
        ) {
            Ok(opened_file) => {
                Self::deserialize_mut_constructor(opened_file, &mut containant_constructor);
                return containant_constructor
            },
            // Serializes Wrapper::default().0 in case of file error.
            Err(err) => {
                warn!("Failed to open/read file at '{}'. -> REPLACING WITH DEFAULTS. Error: {}", path, err);
                // Serialize Self::Wrapper::default().0 in this Err(err) arm
                Self::serialize_constructor(path, &containant_constructor);
                return containant_constructor
            }
        } 
    }

    // Serialize constructor to the file at 'path'. Wrapper::default().0 is used as constructor by default.
    // any other constructor can be passed in to serialize different from default() custom values.
    fn serialize_constructor(path: &str, containant_constructor: &Self::Containant) -> () {
        // Type name of Containant for debug messages
        let containant_name = std::any::type_name::<Self::Containant>();

        // Get check path
        let check = Self::check_path(path, IS_DEBUG_PATH);
        let path: &str = check.to_str().unwrap();
        // Write RON file at 'path'
        match std::fs::write(
            path,
            ron::ser::to_string_pretty(
                &containant_constructor,
                PrettyConfig::default(),
            )
                .unwrap_or_else(|err| panic!("Failed to serialize/write default() config to '{}' file. \n How the hell did that happen??? -> Error: {}", path, err)),
        ) {
            Ok(()) => info!("Serialization of {1} completed OK.\n ^^^^^^^ FILE: '{0}' written/replaced with {1}", path, containant_name),
            Err(err) => error!("Failed to write '{2}' to '{0}' file. Error: {1}", path, err, containant_name),
        }
    }
    // Deserializes the file and tries to mutate each field of passed in constructor and returns the result. Containant::default() is passed in by default.
        // For now only tries to deserialize the whole file. If failes just returns the error and default()
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

    fn check_path(path: &str, is_root_path: bool) -> PathBuf {
        // Get path buffer of the executable for packaging purposes
        let mut path_buf = PathBuf::new();
        // DEBUG:
        // Input false to is_root_path to keep directory structure when compiling debug with cargo build
        // Input true when cargo build --release to package exec binary
        // Get relative to executable path to work
        if is_root_path {
            path_buf.push(std::env::current_exe()
                .expect("Couldn't get path for current executable")
                .parent()
                .expect("Couldn't get parent for current executable path. HOW? IMPOSSIBLE! Shouldn't have happened!"));
            info!("Using relative to executable path. All ok...")
        } else {
            warn!("Using non relative to executable path!!! FOR DEBUG ONLY!!! Intended to be used with 'cargo run' when IS_DEBUG_PATH = false. ");
            path_buf.push(
                std::env::current_dir()
                    .expect("Couldn't get path for current working directory")
            );
        }
        // Push input relative path
        path_buf.push(path);

        let parent_dir = path_buf.parent().expect("Couldn't get parent of PathBuf");
        if !parent_dir.exists() {
            match std::fs::create_dir_all(parent_dir) {
                Ok(()) => {
                    warn!("Directory {:?} does not exist. Creating new ones RECURSIVELY...", parent_dir);
                }
                Err(err) => {
                    error!("Failed to create directory at path '{:?}'. Error: {err}", parent_dir);
                }
            };
        }
        info!("Checked path: {:?}", path_buf);
        return path_buf
    }
} 

