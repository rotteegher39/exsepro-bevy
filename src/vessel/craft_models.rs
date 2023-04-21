use super::modules::*;
use bevy::{reflect::*, prelude::ReflectDefault};

#[derive(Debug)]
pub struct CraftModel{
    pub modules: CraftModules
}

#[derive(Debug, Reflect, FromReflect)]
#[reflect(with = "bevy::reflect::ReflectDefault")]
pub enum CraftTypes {
    Small(Smodls),
    Medium(Mmodls),
    Big(Bmodls),
}


#[derive(Default, Debug, Reflect, FromReflect)]
#[reflect(Default, with = "bevy::reflect::ReflectDefault")]
pub enum Smodls {
    #[default]
    Zabuton,
    Kiff,
    Fillet,
    Typhon,
    Pliashka,
}

#[derive(Default, Debug, Reflect, FromReflect)]
#[reflect(Default, with = "bevy::reflect::ReflectDefault")]
pub enum Mmodls {
    #[default]
    Suki,
    Krishna,
    Orion,
    Pegasus,
    Echidna,
    Monad,
}

#[derive(Default, Debug, Reflect, FromReflect)]
#[reflect(Default, with = "bevy::reflect::ReflectDefault")]
pub enum Bmodls {
    #[default]
    Liubyt,
    Stati,
    Stratus,
    Aether,
}

pub trait CraftypesModelInterface<ANYMODEL> {
    fn fetch_modelinfo(typ: ANYMODEL) -> CraftModel;
}
