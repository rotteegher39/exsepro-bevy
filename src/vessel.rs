use std::f32::consts::PI;

use {
    modules::*,
};

mod modules;


// Main Vessel Information
#[derive(Debug)]
pub struct Craft {
    pub global_craft_params: CraftParams,
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
            global_craft_params: CraftParams {
                craft_name: "Testos".to_owned(), 
                craft_model: "ASX001".to_owned(), 
                craft_type: CraftTypes::Defolt }
        }
    }
}

use crate::vessel::CraftTypes::*;

impl Craft {
    pub fn new(typ: CraftTypes) -> Craft {
        match typ {
            Defolt => Craft::default(),
            _ => Craft::default(),

            Chiisai => todo!(),
            Chuu => todo!(),
            Ooki => todo!(),
        }
    }
}
