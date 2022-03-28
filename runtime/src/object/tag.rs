#[repr(usize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
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

impl NyahObjectTag{
    pub fn is_unknown(&self) -> bool {
        self == &NyahObjectTag::Unknown
    }

    pub fn is_unmarked(&self) -> bool {
        self == &NyahObjectTag::Unmarked
    }

    pub fn is_marked(&self) -> bool {
        self == &NyahObjectTag::Marked
    }

    pub fn is_static(&self) -> bool {
        self == &NyahObjectTag::Static
    }

    pub fn is_ref_count(&self) -> bool {
        self == &NyahObjectTag::RefCount
    }
}

impl Default for NyahObjectTag {
    fn default() -> Self {
        Self::Unknown
    }
}
