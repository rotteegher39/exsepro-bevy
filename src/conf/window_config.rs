
use bevy::{prelude::*, window::*};
use bevy::log::LogPlugin;
use super::*;

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

// Path for window config file.
pub const WINCONF_PATH: &str = "conf-ron/window_config.ron";

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
