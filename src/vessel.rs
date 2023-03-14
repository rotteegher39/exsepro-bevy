use bevy::prelude::*;

use {
    modules::*,
};

mod modules;






// Main Vessel Information
#[derive(Component, Debug)]
pub struct Craft {
    pub param: CraftParams,
}
#[derive(Debug)]
pub struct CraftParams {
    pub craft_name: String,
    pub craft_model: String,
    pub craft_type: CraftTypes,
}

#[derive(Debug)]
pub enum CraftTypes {
    Defolt,
    Chiisai,
    Chuu,
    Ooki,
}

impl Default for Craft {
    fn default() -> Self {
        Self { 
            param: CraftParams {
                craft_name: "Testos".to_owned(), 
                craft_model: "ASX001".to_owned(), 
                craft_type: CraftTypes::Defolt }
        }
    }
}

use CraftTypes::*;

impl Craft {
    pub fn new(typ: &CraftTypes) -> Self {
        match typ {
            _ | Defolt => Self::default(),
            Chiisai => todo!(), // add craft model
            Chuu => todo!(),
            Ooki => todo!(),
        }
    }
}
