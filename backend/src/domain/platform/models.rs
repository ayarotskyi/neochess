use strum_macros::{EnumString, IntoStaticStr};

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumString, IntoStaticStr)]
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
