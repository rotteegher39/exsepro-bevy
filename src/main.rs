#![allow(unused)]
use std::any::TypeId;
use bevy::app::PluginGroupBuilder;
use bevy::ecs::query::WorldQuery;
use bevy::input::keyboard;
use bevy::math::vec3;
use bevy::{prelude::*, window::*};
use bevy::log::LogPlugin;
use std::process::Command;

use vessel::craft_models::{*, Bmodls::*, Mmodls::*, Smodls::*};
use vessel::*;
pub mod vessel;
pub mod winconf;

use winconf::*;
fn main() {
    App::new()
        // Parameters to setup WINDOW
        .add_plugin(LogPlugin::default())
        .add_plugins(set_window())
        // startup
        .add_startup_system(spawn_camera)
        .add_startup_system(spawncrafts)
        // Print Debug info
        // .add_system(info)
        .insert_resource(ClearColor(Color::MIDNIGHT_BLUE))
        .run();
}
// Get Camera working
fn spawn_camera(mut cmd: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    cmd.spawn(Camera3dBundle {
        transform: Transform::from_xyz(window.height() / 2.0, window.width() / 2.0, 0.0),
        ..Default::default()
    });
}
// Spawn Crafts with easy interface of name and model.
// Craft::new_define() fetches info about model&name and returns Craft instance
// of Craft with specified model
pub fn spawncrafts(
    mut cmd: Commands,
) {
    cmd.spawn(
            Craft::new_define("CSX001".to_string(), Zabuton),
    );
    cmd.spawn(
            Craft::new_define("CSX002".to_string(), Pliashka),
    );
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
