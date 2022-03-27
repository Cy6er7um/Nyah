#[repr(usize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum NyahObjectTag {
    /// The garbage collector will skip `Unknown` objects.
    ///
    /// Usually unbound objects belong to Unknown objects.
    Unknown = 0,

    Unmarked = 1,
    Marked = 2,

    Static = 3,

    RefCount = 4,
}

impl Default for NyahObjectTag {
    fn default() -> Self {
        Self::Unknown
    }
}
