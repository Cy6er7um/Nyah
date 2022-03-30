use crate::class::reference::NyahClassReference;
use crate::object::tag::NyahObjectTag;

pub struct NyahObjectHeader {
    pub class: NyahClassReference,
    pub tag: NyahObjectTag,
    pub ref_count: usize,
}
