#![allow(unused)]
use bevy::{prelude::*, window::{ExitCondition, PresentMode, WindowLevel, WindowMode}};
use std::{process::Command, time::Duration};
use std::thread::sleep;

use vessel::{Craft, CraftGet, craft_models, CraftTypes::{Chiisai, Chuu}};
use crate::craft_models::{Cmods::Zabuton, Umods::Krishna, Omods::*};
mod vessel;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoVsync,
                mode: WindowMode::Windowed,
                title: "Extermination Search Protocol".to_string(),
                resize_constraints: WindowResizeConstraints { min_width: 200.0, min_height: 200.0, max_width: 500.0, max_height: 300.0 },
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
        .add_startup_system(setup)
        .add_system(show)
        .run();
}

fn setup(mut cmd: Commands) {
    cmd.spawn(Craft::new_def("ASX-Tester".to_string(), Chiisai(Zabuton)));
    cmd.spawn(Craft::new_def("USX-Tester".to_string(), Chuu(Krishna)));
}

fn show(mut crafts_query: Query<&Craft>) {
    for craft in crafts_query.iter() {
        println!("{craft:#?}");
    }
    sleep(Duration::from_millis(500));
    Command::new("clear").status().unwrap();
}

