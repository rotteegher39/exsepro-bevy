use std::borrow::BorrowMut;
use std::fs::File;
use std::io::BufReader;
use bevy::ecs::query::WorldQuery;
use serde::{Deserialize, Serialize};
use ron::de::*;
use ron::ser::*;
use bevy::ecs::system::*;

use bevy::app::PluginGroupBuilder;
use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy::window::*;
use bevy::log::LogPlugin;

#[derive(Resource)]
pub struct WindowConfig(Window);
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
                decorations: false,
                transparent: false,
                focused: true,
                window_level: WindowLevel::AlwaysOnTop,
                ..Default::default()
            }
        )
    }
}

pub const WINCONF_PATH: &str = "window_config.ron";
pub fn fetch_winconfig() -> Window {
        let window_config: Window = match File::open(WINCONF_PATH) {
            Ok(file) => { let reader = BufReader::new(file);
                info!("{} file opened", WINCONF_PATH);
                match ron::de::from_reader(reader) {
                    Ok(window) => {
                        info!("Window settings from window_config.ron deserialized successfully");
                        window
                    },
                    Err(_) => { 
                        warn!("Failed to deserialize Window struct from window_config.ron to load window settings. \nUSING DEFAULTS");
                        WindowConfig::default().0
                    },
                }
            },
            Err(_) => {
                warn!("Failed to open/read file -> 'window_config.ron'. Does File exist with correct permissions? \nReplacing/Serializing with defaults...");
                let serialized_config = ron::ser::to_string_pretty(&WindowConfig::default().0, PrettyConfig::default())
                    .expect("How did the serialization of 'WindowConfig::default().0' failed?");
                std::fs::write(WINCONF_PATH, serialized_config)
                    .expect("Failed to write Window::default() into window_config.ron file.");
                WindowConfig::default().0
            },
        };
        return window_config;
}

pub fn set_window() -> PluginGroupBuilder {
    DefaultPlugins.set(
        WindowPlugin {
            primary_window: Some(fetch_winconfig()),
            exit_condition: ExitCondition::OnAllClosed,
            close_when_requested: true,
        }
        )
        .disable::<LogPlugin>()
}
