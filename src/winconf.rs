use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy::window::*;

pub fn get_window() -> PluginGroupBuilder {
    DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
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
        }),
        exit_condition: ExitCondition::OnAllClosed,
        close_when_requested: true,
        ..Default::default()
    })
}
