use std::process::Command;
use std::time::Duration;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::text::TextPlugin;

use super::vessel::*;
use super::conf::*;

pub struct DebugInfoPlugin;


impl Plugin for DebugInfoPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(WorldInspectorPlugin::new())
            .add_plugin(
                LogDiagnosticsPlugin {
                    debug: false,
                    wait_duration: Duration::from_millis(500),
                    filter: None,
                }
            )
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(info)
            ;
    }
}
pub struct Hello(String);
impl Default for Hello {
    fn default() -> Self {
        Hello("default".to_string())
    }
}

impl Container for Hello {
    type Wrapper = Hello;
    type Containant = String;
    fn unwrap_containant_from(wrapper: Hello) -> String {
        wrapper.0
    }
}
// Query info for All Crafts
fn info(
    craft_q: Query<&Craft>,
    camera_trnsf: Query<&mut Transform>,
    asset_server: Res<AssetServer>,
) {
    let some_string = "Hello World!".to_string();
    Hello::serialize_constructor("test1/test2/hello.ron", &some_string)
    // Command::new("clear")
    //     .status()
    //     .expect("constant auto clear command didn't work?");
    // for craft in craft_q.iter() {
    //     info!("{craft:#?}");
    // }
    // for camera in camera_trnsf.iter() {
    //     info!("{camera:#?}")
    // }
}
