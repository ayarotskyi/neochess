#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
