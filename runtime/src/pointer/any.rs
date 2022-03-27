use std::fmt::{Debug, Display};

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct NyahAny;

impl NyahAny {
    pub const fn new() -> Self {
        NyahAny
    }
}

impl Display for NyahAny {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Any")
    }
}

impl Debug for NyahAny {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Any")
    }
}
