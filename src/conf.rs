use std::fs::File;
use std::path::PathBuf;
use std::io::Read;
use serde::Serialize;
use serde::de::DeserializeOwned;
use ron::ser::*;
use bevy::log::*;

use bevy::ecs::system::*;
use bevy::prelude::*;

pub mod window_config;


/// Debug force de/serializing path to be package root dir when working with cargo.
#[cfg(debug_assertions)]
pub const NO_DEBUG_PATH: bool = false;

/// Debug force de/ser path to be relative to executable file root dir when running binary itself.
#[cfg(not(debug_assertions))]
pub const NO_DEBUG_PATH: bool = true;

/// Serializes and deserializes a contained type, `Containant`, into/from a RON file.
///
/// # Example:
/// - `ExampleWindowConfig`: A wrapper type for the `Window` struct.
/// - `Window`: The configurable type that is saved/fetched into/from a RON file at a specific path,
///   which is contained inside the example `WindowConfig` struct.
/// ```rust,no_run
/// pub struct WindowConfig(Window);
///
/// impl Container for WindowConfig {
///     type Wrapper = WindowConfig;
///     type Containant = Window;
///
///     // Unwraps the `Containant` from the `Wrapper`.
///     fn unwrap_containant_from(wrapper: WindowConfig) -> Window {
///         wrapper.0
///     }
/// }
///
/// // Provides unique defaults for `Containant` for each different type that implements the `Container` trait.
/// impl Default for WindowConfig {
///     fn default() -> Self {
///         WindowConfig(Window { /* unique defaults */ })
///     }
/// }
///
/// impl Default for OtherWindowConfig {
///     fn default() -> Self {
///         OtherWindowConfig(Window { /* unique defaults */ })
///     }
/// }
///
/// // Loads the `Window` struct from the RON file at the specified path.
/// // If the file cannot be loaded, `default()` will be used and serialized instead.
/// pub fn load_window() -> Window {
///     WindowConfig::fetch_containant("path/to/windowconfig.ron")
/// }
///
/// // Fetches exampleconfig.ron from parsed `path`
/// fetch_containant(path: &str)
/// - `path` is relative to executable when compiling with --release
/// - cargo run by default makes `path` relative to the project's top directory.
/// - otherwise `path` is relative to current execution `pwd` directory. 
/// ```
pub trait Container: Sized {
    /// Wrapper that contains 'Containant'
    type Wrapper: Default;
    /// Type that is returned from a fetch. (Type that contained inside Wrapper)
    type Containant: Default + Serialize + DeserializeOwned;
    /// Specify how to get Containant from WrSpper
    fn unwrap_containant_from(wrapper: Self::Wrapper) -> Self::Containant;

    /// Function to fetch configuration from a file and deserialize Wrapper::default() if does not exist
    fn fetch_containant(path: &str) -> Self::Containant {

        // Get mutable default containant constructor from default Wrapper values.
        // Uses default() values of Wrapper. Separete from Containant::default() itself.
            // Not to be confused with Containant::default().
        let mut containant_constructor = Self::unwrap_containant_from(Self::Wrapper::default());

        // Try to open the config File and parse it
        match File::open(
            // funciton to check if path is available.
            Self::check_path(path, NO_DEBUG_PATH).to_str().unwrap()
        ) {
            // Try to deserialize opened file into RON.
            Ok(opened_file) => {
                Self::deserialize_mut_constructor(opened_file, &mut containant_constructor);
                return containant_constructor
            },
            // Gets Containant from Wrapper::default() and Serializes them at path,
            Err(err) => {
                warn!("Failed to open/read file at '{}' REPLACING IT WITH DEFAULTS. Error: {}", path, err);
                // Serialize Self::Wrapper::default().0 in this Err(err) arm
                Self::serialize_constructor(path, &containant_constructor, PrettyConfig::default());
                return containant_constructor
            }
        } 
    }

    /// Serialize constructor to the RON file at 'path'. Wrapper::default() is usually used by default.
    /// any other constructor can be passed in to serialize different from default() custom values.
    fn serialize_constructor(path: &str, containant_constructor: &Self::Containant, pretty_config: PrettyConfig) {
        // type_name of containant for debug messages
        let containant_name = std::any::type_name::<Self::Containant>();

        // Get to check path
        let check = Self::check_path(path, NO_DEBUG_PATH);
        let path: &str = check.to_str().unwrap();
        // Write RON file at checked 'path'
        match std::fs::write(
            path,
            ron::ser::to_string_pretty(
                &containant_constructor,
                pretty_config,
            )
                .unwrap_or_else(|err| panic!("Failed to serialize/write default() config to '{}' file. \n How the hell did that happen??? -> Error: {}", path, err)),
        ) {
            Ok(()) => info!("Serialization of {1} completed OK.\n ^^^^^^^ FILE: '{0}' written/replaced with {1}", path, containant_name),
            Err(err) => error!("Failed to write '{2}' to '{0}' file. Error: {1}", path, err, containant_name),
        }
    }
    /// Tries to Deserialize already opened File from RON format then
    /// Mutates each field of passed in constructor_containant accordingly to file_contents.
        /// If could not be deserialized, then leaves returns passed in containant_constructor unchanged.
    /// For now only tries to deserialize the whole file. If failes just returns the error and default()
    fn deserialize_mut_constructor(mut opened_file: File, containant_constructor: &mut Self::Containant) {
        // type_name of constructor for debug messages
        let constructor_name = std::any::type_name::<Self::Containant>();

        let mut file_contents = String::new();
        if let Err(err) = opened_file.read_to_string(&mut file_contents) {
            error!("Could not read 'opened_file' of type File: '{:#?}', Error: {}", opened_file, err);
            error!("File: '{:#?}', could not be deserialized, Error: {}", opened_file, err);
            warn!("Containant constructor: '{}' is unchanged, returning...", constructor_name);
            return
        };
        // Try to deserialize whole file at once and mutate the containant_constructor based on file_contents
        *containant_constructor = match ron::de::from_str(&file_contents){
            Ok(deserialized) => { 
                info!("Deserialized '{}' loaded successfully", constructor_name);
                deserialized 
            },
            Err(err) => {
                error!("Failed to deserialize '{}' ... Skipping. The Error: {} ", constructor_name, err);
                return
            }
        };
    }

    /// Checks if path is available, if not creates folders.
    /// NO_DEBUG_PATH=false used for debug when using cargo run
    /// NO_DEBUG_PATH=true used when cargo run --release
    fn check_path(path: &str, debug_path: bool) -> PathBuf {
        // Get path buffer of the executable for packaging purposes
        let mut path_buf = PathBuf::new();
        // DEBUG:
        // Input false to is_not_debug to keep dir structure at package root when running cargo run
        // Input true when cargo run --release to keep dir structure relative to exec binary
        // Get or not get relative to executable path.
        if debug_path {
            path_buf.push(std::env::current_exe()
                .expect("Couldn't get path for current executable")
                .parent()
                .expect("Couldn't get parent for current executable path. HOW? IMPOSSIBLE! Shouldn't have happened!"));
            info!("Using relative to executable path. All ok...")
        } else {
            warn!("Using non relative to executable path!!! FOR DEBUG ONLY!!! Intended to be used with 'cargo run' when NO_DEBUG_PATH = false. ");
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

