use std::fs::File;
use std::io::BufReader;
use serde::{Deserialize, Serialize};
use ron::de::*;
use ron::ser::*;
use bevy::ecs::system::*;
use tracing::warn;


use bevy::app::PluginGroupBuilder;
use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy::window::*;

pub fn load_window_settings(winconf_path: &str) -> Window {
    let file = File::open(winconf_path);
    let hardcoded_window_defaults: Window = 
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
            decorations: false,
            transparent: false,
            focused: true,
            window_level: WindowLevel::AlwaysOnTop,
            ..Default::default()
    };
        match file {
            Ok(file) => {
                let reader = BufReader::new(file);
                info!("{} file opened", winconf_path);
                match ron::de::from_reader(reader) {
                    Ok(window) => {
                        info!("Window settings from window_config.ron deserialized successfully");
                        window 
                    },
                    Err(_) => { 
                        warn(In(Err("Failed to deserialize Window struct from window_config.ron to load window settings. \nUSING DEFAULTS")));
                        hardcoded_window_defaults
                    },
                }
            },
            Err(_) => {
                warn(In(Err("Failed to open/read file -> 'window_config.ron'. Does File exist with correct permissions? \nReplacing/Serializing with defaults...")));
                let serialized_config = ron::ser::to_string_pretty(&hardcoded_window_defaults, PrettyConfig::default())
                    .expect("How did the serialization of 'hardcoded_window_defaults' failed?");
                std::fs::write(winconf_path, serialized_config)
                    .expect("Failed to write Window::default() into window_config.ron file.");
                hardcoded_window_defaults
            },
    }
}

pub fn get_window(window_config: Window) -> PluginGroupBuilder {

    DefaultPlugins.set(WindowPlugin {
        primary_window: Some(window_config),
        exit_condition: ExitCondition::OnAllClosed,
        close_when_requested: true,
        ..Default::default()
    })
}
