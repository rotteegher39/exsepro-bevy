#![allow(unused)]
use bevy::prelude::*;

use vessel::*;
mod vessel;

fn main() {
    App::new()
        .add_startup_system(setup)
        .run()
}

fn setup() {
    println!("Hello World")
}


