use super::*;
use bevy::log::LogPlugin;
use bevy::app::PluginGroupBuilder;
use bevy::window::*;


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
        WindowConfig(Window {
            present_mode: PresentMode::AutoVsync,
            mode: WindowMode::Windowed,
            title: "External Search Protocol".to_string(),
            resize_constraints: WindowResizeConstraints {
                min_width: 300.0,
                min_height: 300.0,
                max_width: 500.0,
                max_height: 300.0,
            },
            resolution: WindowResolution::default().with_scale_factor_override(0.85),
            resizable: true,
            decorations: true,
            transparent: false,
            focused: true,
            window_level: WindowLevel::AlwaysOnTop,
            cursor: Cursor::default(),
            position: Default::default(),
            composite_alpha_mode: CompositeAlphaMode::Auto,
            canvas: Default::default(),
            fit_canvas_to_parent: Default::default(),
            prevent_default_event_handling: false,
            internal: InternalWindowState::default(),
            ime_enabled: false,
            ime_position: Vec2::default(),
        })
    }
}

// Relative to executable path to window config file.
pub const WINCONF_PATH: &str = "conf-ron/window_config.ron";

// Adds DefaultPlugins and sets Window from fetch_winconfig().
// Should always work, even if config file did not load, it should load default()
pub fn set_window_and_defaultplugins() -> PluginGroupBuilder {

    DefaultPlugins
        .set(WindowPlugin { // put true when compiling with --relase
            primary_window: Some(WindowConfig::fetch_containant(WINCONF_PATH, IS_ROOT_PATH)), // put false when building debug
            exit_condition: ExitCondition::OnAllClosed,
            close_when_requested: true,
        })
        .disable::<LogPlugin>()
}
