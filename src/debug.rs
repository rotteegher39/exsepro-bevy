use std::process::Command;
use std::time::Duration;

use bevy::utils::Instant;
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin, Diagnostics},
    prelude::*,
};
use bevy::transform::commands;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use super::vessel::*;
use super::conf::*;

pub struct DebugInfoPlugin;


impl Plugin for DebugInfoPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(WorldInspectorPlugin::new())
            // .add_plugin(
            //     LogDiagnosticsPlugin {
            //         debug: false,
            //         wait_duration: Duration::from_millis(2500),
            //         filter: None,
            //     }
            // )
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            //
            // .add_startup_system(craft_info)
            .add_startup_system(setup_debug)
            .add_system(fps)
            ;
    }
}

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
struct FpsText;
pub const FONT: &str = "fonts/monocraft/Monocraft.ttf";
fn setup_debug(
    asset_server: Res<AssetServer>,
    mut cmd: Commands,
) {
    // UI camera
    cmd.spawn(Camera2dBundle::default());
    // Text with one section
    cmd.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_sections([
                TextSection::from_style(
                    TextStyle {
                        font: asset_server.load(FONT),
                        font_size: 15.0,
                        color: Color::GREEN,
                    },
                ),
                TextSection::from_style(
                    TextStyle {
                        font: asset_server.load(FONT),
                        font_size: 12.0,
                        color: Color::GOLD,
                    }
                ),
                TextSection::from_style(
                    TextStyle {
                        font: asset_server.load(FONT),
                        font_size: 12.0,
                        color: Color::WHITE,
                    }
                ),
        ])
        .with_background_color(Color::BLACK)
        // Set the alignment of the Text
        .with_text_alignment(TextAlignment::Right)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.0),
                right: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        FpsText,
    ));
}


fn fps(
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<FpsText>>,
    mut last_update_time: Local<Option<Instant>>,
) {
    for mut text in &mut query {
        let now = Instant::now();
        let elapsed = last_update_time.get_or_insert_with(|| now).elapsed();
        if elapsed >= Duration::from_secs_f32(0.5) {
            if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(value) = fps.smoothed() {
                    // Update the value of the first section
                    text.sections[0].value = format!("{:.0}fps\n", value);
                }
            }
            if let Some(frame_time) = diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_TIME) {
                if let Some(value) = frame_time.smoothed() {
                    // Update the value of the second section
                    text.sections[1].value = format!("{:.2}ms\n", value);
                }
            }
            if let Some(frame_count) = diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_COUNT) {
                if let Some(value) = frame_count.smoothed() {
                    // Update the value of the second section
                    text.sections[2].value = format!("{:.0}f\n", value);
                }
            }
            *last_update_time = Some(now);
        }
    }
}


// Query info for All Crafts
fn craft_info(
    craft_q: Query<&Craft>,
    camera_trnsf: Query<&mut Transform>,
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
