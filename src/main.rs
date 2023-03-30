#![allow(unused)]
use std::any::TypeId;
use bevy::app::PluginGroupBuilder;
use bevy::input::keyboard;
use bevy::math::vec3;
use bevy::{prelude::*, window::*};
use std::process::Command;

use crate::craft_models::{Cmods::*, Omods::*, Umods::*};
use vessel::{CraftTypes::*, *};
pub mod vessel;
pub mod winconf;

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
pub const ANGLE_SPEED: f32 = 0.01;


fn get_input(
    keyin: Res<Input<KeyCode>>,
    mut craft_trnsf: Query<&mut Transform, With<Craft>>,
    time: Res<Time>,
    ) {
    if let Ok(mut trnsf) = craft_trnsf.get_single_mut() {
        let mut p_up: bool = keyin.pressed(KeyCode::Up);
        let mut p_left: bool = keyin.pressed(KeyCode::Left);
        let mut p_down: bool = keyin.pressed(KeyCode::Down);
        let mut p_right: bool = keyin.pressed(KeyCode::Right);

        let mut p_x: bool = keyin.pressed(KeyCode::X);
        let mut p_y: bool = keyin.pressed(KeyCode::Y);
        let mut p_z: bool = keyin.pressed(KeyCode::Z);
        let mut p_w: bool = keyin.pressed(KeyCode::W);

        let mut p_xs: bool = keyin.pressed(KeyCode::Space) && keyin.pressed(KeyCode::X);
        let mut p_ys: bool = keyin.pressed(KeyCode::Space) && keyin.pressed(KeyCode::Y);
        let mut p_zs: bool = keyin.pressed(KeyCode::Space) && keyin.pressed(KeyCode::Z);
        let mut p_ws: bool = keyin.pressed(KeyCode::Space) && keyin.pressed(KeyCode::W);
        
        let mut reset: bool = keyin.pressed(KeyCode::Space) && keyin.pressed(KeyCode::R);

        let mut direction = Vec3::ZERO;



        // WASD
            if p_up {
               direction += vec3(0.0, 1.0, 0.0) 
            }
            if p_left {
               direction += vec3(-1.0, 0.0, 0.0) 
            }
            if p_down {
               direction += vec3(0.0, -1.0, 0.0) 
            }
            if p_right {
               direction += vec3(1.0, 0.0, 0.0) 
            }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        // XYZW
        if p_xs {
            trnsf.rotate_x(-ANGLE_SPEED);
        } else {
            if p_x {
                trnsf.rotate_x(ANGLE_SPEED);
            }
        }
        if p_ys {
            trnsf.rotate_y(-ANGLE_SPEED);
        } else {
            if p_y {
                trnsf.rotate_y(ANGLE_SPEED);
            }
        }
        if p_zs {
            trnsf.rotate_z(-ANGLE_SPEED);
        } else {
            if p_z {
                trnsf.rotate_z(ANGLE_SPEED);
            }
        }
        if p_ws {
            trnsf.rotation.w += -ANGLE_SPEED;
        } else {
            if p_w {
                trnsf.rotation.w += ANGLE_SPEED;
            }
        }

        trnsf.translation += direction * SPEED * time.delta_seconds();
    } 
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
    for craft in craft_trnsf.iter() {
        println!("{craft:#?}");
    }
    println!("Sprite {:?}", asset_server.get_load_state(IMAGE))
}
