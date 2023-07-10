use super::modules::*;
use bevy::{reflect::*, prelude::ReflectDefault};

#[derive(Debug)]
pub struct CraftModel{
    pub modules: CraftModules
}

#[derive(Debug, Reflect)]
pub enum CraftTypes {
    Small(Smodls),
    Medium(Mmodls),
    Big(Bmodls),
}


#[derive(Default, Debug, Reflect)]
#[reflect(Default)]
pub enum Smodls {
    #[default]
    Zabuton,
    Kiff,
    Fillet,
    Typhon,
    Pliashka,
}

#[derive(Default, Debug, Reflect)]
#[reflect(Default)]
pub enum Mmodls {
    #[default]
    Suki,
    Krishna,
    Orion,
    Pegasus,
    Echidna,
    Monad,
}

#[derive(Default, Debug, Reflect)]
#[reflect(Default)]
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
