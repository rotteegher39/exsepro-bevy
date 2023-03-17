#![allow(unused)]
use std::any::TypeId;
use bevy::app::PluginGroupBuilder;
use bevy::input::keyboard;
use bevy::math::vec3;
use bevy::{prelude::*, window::*};
use std::process::Command;

use crate::craft_models::{Cmods::*, Omods::*, Umods::*};
use vessel::{CraftTypes::*, *};
mod vessel;
mod winconf;

use winconf::get_window;
fn main() {

    App::new()
        // Parameters to setup WINDOW
        .add_plugins(winconf::get_window())
        // startup
        .add_startup_system(vessel::spawncrafts)
        .add_startup_system(spawn_camera)

        .add_system(get_input)
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

fn get_input(
    keyin: Res<Input<KeyCode>>,
    mut craft_trnsf: Query<&mut Transform, With<Craft>>,
    time: Res<Time>,
    ) {
    if let Ok(mut trnsf) = craft_trnsf.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyin.pressed(KeyCode::H) {
           direction += vec3(-1.0, 0.0, 0.0) 
        }
        if keyin.pressed(KeyCode::J) {
           direction += vec3(0.0, -1.0, 0.0) 
        }
        if keyin.pressed(KeyCode::K) {
           direction += vec3(0.0, 1.0, 0.0) 
        }
        if keyin.pressed(KeyCode::L) {
           direction += vec3(1.0, 0.0, 0.0) 
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        trnsf.translation += direction * SPEED * time.delta_seconds();
    } 
}

// Query info for All Crafts
fn info(craft_q: Query<&Craft, With<Craft>>, asset_server: Res<AssetServer>) {
    Command::new("clear")
        .status()
        .expect("constant auto clear command didn't work?");

    for craft in craft_q.iter() {
        println!("{craft:#?}");
    }
    println!("Sprite {:?}", asset_server.get_load_state(IMAGE))
}
