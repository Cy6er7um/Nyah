use crate::object::tag::NyahObjectTag;

pub struct NyahObjectHeader {
    // TODO: pub class: NyahClassReference,
    pub tag: NyahObjectTag,
    pub ref_count: usize,
}
