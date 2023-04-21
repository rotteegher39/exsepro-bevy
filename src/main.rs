#![allow(unused)]
use std::process::Command;

use bevy::log::LogPlugin;
use bevy::{prelude::*, window::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use vessel::craft_models::{Bmodls::*, Mmodls::*, Smodls::*, *};
use vessel::*;
pub mod conf;
pub mod vessel;

// The Project builder. AKA "App"
fn main() {
    App::new()
        // init logging system cause it's disabled in init_window_and_defaultplugins()
        // MUST be first in the APP to work
        .add_plugin(LogPlugin::default())
        // Parameters to setup DefaultPlugins with set Window from window_config.ron
        .add_plugins(conf::window_config::set_window_and_defaultplugins())
        .add_plugin(WorldInspectorPlugin::new())
        // camera
        .add_startup_system(spawn_camera)
        // startup testings
        .add_startup_system(spawncrafts)
        .register_type::<Craft>()
        .register_type::<Smodls>()
        .register_type::<Mmodls>()
        .register_type::<Bmodls>()
        // Print Debug info
        // .add_system(info)
        .insert_resource(ClearColor(Color::MIDNIGHT_BLUE))
        .run();
}

// Get Camera working. No camera - no display.
fn spawn_camera(mut cmd: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    window_query.get_single().unwrap();
    cmd.spawn(Camera3dBundle {
        transform: Transform::IDENTITY, // at position 0
        ..Default::default()
    });
}

// Spawn Crafts with easy interface of name and model.
// Craft::new_define() fetches info about model&name and returns Craft component instance
// of Craft with specified model
pub fn spawncrafts(mut cmd: Commands) {
    cmd.spawn(Craft::new_define("CSX001".to_string(), Zabuton));
    cmd.spawn(Craft::new_define("CSX002".to_string(), Pliashka));
}

// Query info for All Crafts
fn info(
    craft_q: Query<&Craft>,
    camera_trnsf: Query<&mut Transform>,
    asset_server: Res<AssetServer>,
) {
    Command::new("clear")
        .status()
        .expect("constant auto clear command didn't work?");
    for craft in craft_q.iter() {
        info!("{craft:#?}");
    }
    for camera in camera_trnsf.iter() {
        info!("{camera:#?}")
    }
}
