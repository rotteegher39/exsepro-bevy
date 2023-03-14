#![allow(unused)]
use bevy::asset::AssetLoader;
use bevy::{prelude::*, window::*};
use std::{process::Command, time::Duration};
use std::thread::sleep;

use vessel::{*, CraftTypes::*};
use crate::craft_models::{Cmods::Zabuton, Umods::Krishna, Omods::*};
mod vessel;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoVsync,
                mode: WindowMode::Windowed,
                title: "Extermination Search Protocol".to_string(),
                resize_constraints: WindowResizeConstraints { min_width: 300.0, min_height: 300.0, max_width: 500.0, max_height: 300.0 },
                resizable: true,
                decorations: false,
                transparent: true,
                focused: true,
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
        // .add_system(show)
        .run();
}
#[derive(Component)]
pub struct Player {}

const IMAGE: &str = "sprites/pp.png";
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
            Player {}
        )
    );
}


fn spawn_camera(
    mut cmd: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    ) {
    let window = window_query.get_single().unwrap();

    cmd.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.height() / 2.0, window.width() / 2.0, 0.0),
        ..Default::default()
    });


}

fn show(mut crafts_query: Query<&Craft>, asset_server: Res<AssetServer>) {
    for craft in crafts_query.iter() {
        println!("{craft:#?}");
    }
    sleep(Duration::from_millis(500));
    Command::new("clear").status().expect("auto clear command didn't work?");
    println!("{:?}", asset_server.get_load_state(IMAGE))
}

