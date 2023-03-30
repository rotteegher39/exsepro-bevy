#![allow(unused)]
use std::any::TypeId;
use bevy::app::PluginGroupBuilder;
use bevy::input::keyboard;
use bevy::math::vec3;
use bevy::{prelude::*, window::*};
use std::process::Command;

use vessel::craft_models::{*, Bmodls::*, Mmodls::*, Smodls::*};
use vessel::*;
pub mod vessel;
pub mod winconf;

use winconf::get_window;
fn main() {
    App::new()
        // Parameters to setup WINDOW
        .add_plugins(winconf::get_window())
        // startup
        // .add_startup_system(vessel::spawncrafts)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawncrafts)
        .add_system(info)
        .run();
}

fn spawn_camera(mut cmd: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    cmd.spawn(Camera2dBundle {
        // transform: Transform::from_xyz(window.height() / 2.0, window.width() / 2.0, 0.0),
        ..Default::default()
    });
}

pub const SPEED: f32 = 250.0;
pub const ANGLE_SPEED: f32 = 0.01;

pub fn spawncrafts(
    mut cmd: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let scale: f32 = 0.15;

    cmd.spawn(
        (
            Craft::new_define("CSX001".to_string(), Zabuton),
        )
    );
}

// Query info for All Crafts
fn info(
    craft_q: Query<&Craft, With<Craft>>,
    craft_trnsf: Query<&Transform, With<Craft>>,
    asset_server: Res<AssetServer>
    ) {
    Command::new("clear")
        .status()
        .expect("constant auto clear command didn't work?");

    for craft in craft_q.iter() {
        println!("{craft:#?}");
    }
}
