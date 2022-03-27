use crate::object::tag::NyahObjectTag;

pub struct NyahObjectHeader {
    pub tag: NyahObjectTag,
    pub ref_count: usize,
}
