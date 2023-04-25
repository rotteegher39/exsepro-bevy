#![allow(unused)]

use bevy::{prelude::*, window::*};

use vessel::*;
use debug::*;
pub mod vessel;
pub mod debug;
pub mod conf;
pub mod assets;

// The Project builder. AKA "App"
fn main() {
    App::new()
        // Setup WindowPlugin with fetched settings from window_settings.ron
        .add_plugin(conf::window_config::set_windowplugin())
        // DefaultPlugins
        .add_plugins(DefaultPlugins.build()
            .disable::<WindowPlugin>()
        )



        // Diagnostics
        .add_plugin(DebugInfoPlugin)


    
        // camera
        .add_startup_system(spawn_camera)
        // Make between frames rendering color when nothing is rendered 
        .insert_resource(ClearColor(Color::NONE))

        // Crafts
        .add_plugin(CraftPlugin)
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
