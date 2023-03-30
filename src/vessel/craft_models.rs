use super::modules::*;

#[derive(Debug)]
pub struct CraftModel{
    pub modules: CraftModules
}

#[derive(Debug)]
pub enum CraftTypes {
    Small(Smodls),
    Medium(Mmodls),
    Big(Bmodls),
}
#[derive(Debug)]
pub enum Smodls {
    Zabuton,
    Kiff,
    Fillet,
    Typhon,
    Pliashka,
}

#[derive(Debug)]
pub enum Mmodls {
    Krishna,
    Orion,
    Pegasus,
    Echidna,
    Suki,
    Monad,
}

#[derive(Debug)]
pub enum Bmodls {
    Stati,
    Stratus,
    Aether,
    Liubytk,
}

pub trait CraftypesModelInterface<ANYMODEL> {
    fn fetch_modelinfo(typ: ANYMODEL) -> CraftModel;
}
