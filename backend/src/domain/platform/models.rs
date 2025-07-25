use strum_macros::{EnumString, IntoStaticStr, VariantNames};

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumString, IntoStaticStr, VariantNames)]
pub enum PlatformName {
    ChessCom,
}

pub struct Platform {
    name: PlatformName,
}

impl Platform {
    pub fn new(name: PlatformName) -> Self {
        Self { name: name }
    }
}
