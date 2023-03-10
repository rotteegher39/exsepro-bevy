#![allow(unused)]

use crate::vessel::*;
use crate::vessel::CraftTypes::*;
mod vessel;

fn main() {
    let craft1 = Craft::new(Defolt);
    println!("{:#?}", craft1)
}


