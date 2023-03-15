#![allow(unused)]
use bevy::math::vec3;
use bevy::{prelude::*, window::*};
use std::{process::Command, time::Duration};
use std::thread::sleep;

use vessel::{*, CraftTypes::*};
use crate::craft_models::{Cmods::*, Umods::*, Omods::*};
mod vessel;

fn main() {
    App::new()
        // Window Parameters
        .add_plugins(DefaultPlugins.set(WindowPlugin {

            primary_window: Some(Window {
                
                present_mode: PresentMode::AutoVsync,
                mode: WindowMode::Windowed,
                title: "Extermination Search Protocol".to_string(),
                resize_constraints: WindowResizeConstraints { min_width: 300.0, min_height: 300.0, max_width: 500.0, max_height: 300.0 },
                resizable: true,
                decorations: false,
                transparent: false,
                focused: false,
                window_level: WindowLevel::AlwaysOnTop,
                ..Default::default()
            }),

            exit_condition: ExitCondition::OnAllClosed,
            close_when_requested: true,
            ..Default::default()
        }))
        // .add_plugins(DefaultPlugins)
        .add_startup_system(onset)
        .add_startup_system(spawn_camera)
        .add_system(info)
        .run();
}

// Player sprite
const IMAGE: &str = "sprites/craft.png";

fn onset(
    mut cmd: Commands,
        window_query: Query<&Window, With<PrimaryWindow>>,
        asset_server: Res<AssetServer>,
    ) {
    let window = window_query.get_single().unwrap();


    cmd.spawn( 
        (
            SpriteBundle {
                transform: Transform::from_xyz(window.height() / 2.0, window.width() / 2.0, 0.0),
                texture: asset_server.load(IMAGE),
                ..default()
            },
            Craft::new_def("CSX001".to_string(), Chiisai(Zabuton)),
        )
    );
}

fn spawn_camera(
    mut cmd: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    ) {
    let window = window_query.get_single().unwrap();
    let scale: f32 = 20 as f32;

    cmd.spawn(Camera2dBundle {
        transform: Transform {
            translation: vec3(window.height() / 2.0, window.width() / 2.0, 0.0),
            rotation: Quat::from_xyzw(0.0, 0.0, 0.0, 0.0),
            scale: vec3(scale, scale, scale),
        },
        ..Default::default()
    });


}

fn info(
        craft_q: Query<&Craft>,
        asset_server: Res<AssetServer>
    )
{
    Command::new("clear").status().expect("constant auto clear command didn't work?");
    for craft in craft_q.iter() {
        println!("{craft:#?}");
    }
    println!("{:?}", asset_server.get_load_state(IMAGE))
}

