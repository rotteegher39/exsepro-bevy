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
    Liubyt,
}

// Useless code, refactor later to something useful
pub trait CraftTypesModelInterface<ANYMODEL> {
    fn get_model(typ: ANYMODEL) -> CraftTypes;
}

impl CraftTypesModelInterface<Smodls> for Smodls {
    fn get_model(typ: Smodls) -> CraftTypes {
        CraftTypes::Small(typ)
    }
}
impl CraftTypesModelInterface<Mmodls> for Mmodls {
    fn get_model(typ: Mmodls) -> CraftTypes {
        CraftTypes::Medium(typ)
    }
}
impl CraftTypesModelInterface<Bmodls> for Bmodls {
    fn get_model(typ: Bmodls) -> CraftTypes {
        CraftTypes::Big(typ)
    }
}
