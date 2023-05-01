// #![allow(unused)]

use bevy::{prelude::*, window::*};
use bevy::log::LogPlugin;

use vessel::*;
use debug::*;
pub mod vessel;
pub mod debug;
pub mod conf;

// The Project builder. AKA "App"
fn main() {
    App::new()
        .add_plugin(LogPlugin::default())
        // Setup WindowPlugin with fetched settings from window_settings.ron
        .add_plugin(conf::window_config::set_windowplugin())
        // DefaultPlugins
        .add_plugins(DefaultPlugins.build()
            .disable::<WindowPlugin>()
            .disable::<LogPlugin>()
        )


        // Diagnostics
        .add_plugin(DebugInfoPlugin)


    
        // camera
        .add_startup_system(spawn_camera)
        // Make between frames rendering color when nothing is rendered 
        .insert_resource(ClearColor(Color::TEAL))

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
