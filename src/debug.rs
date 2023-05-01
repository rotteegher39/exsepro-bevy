use std::process::Command;
use std::time::Duration;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

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
// Query info for All Crafts
fn info(
    craft_q: Query<&Craft>,
    camera_trnsf: Query<&mut Transform>,
    asset_server: Res<AssetServer>,
) {
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
