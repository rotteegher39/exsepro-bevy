#![allow(unused)]
use bevy::{prelude::*, ecs::system::EntityCommands};

use vessel::{*, CraftTypes::*};
use crate::craft_models::{Cmods::*, Umods::*, Omods::*};
mod vessel;

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_system(show)
        .run();
}

fn setup(mut cmd: Commands) {
    cmd.spawn(Craft::new_def("ASX-Tester".to_string(), Chiisai(Zabuton)));
}

fn show(mut crafts_query: Query<&Craft>) {
    for craft in crafts_query.iter() {
        println!("{:#?}", craft);
    }
}

