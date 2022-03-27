use std::fmt::{Debug, Display};

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct NyahUtilAny;

impl NyahUtilAny {
    pub const fn new() -> Self {
        NyahUtilAny
    }
}

impl Display for NyahUtilAny {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Any")
    }
}

impl Debug for NyahUtilAny {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Any")
    }
}
