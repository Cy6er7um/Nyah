use std::fmt::Display;

#[repr(C)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub struct Untyped;

impl Untyped {
    pub const fn new() -> Self {
        Untyped
    }
}

impl Display for Untyped {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Untyped")
    }
}
