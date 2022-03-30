use crate::class::reference::NyahClassReference;

pub struct NyahClassField {
    pub name: String,
    pub class: NyahClassReference,
    pub offset: usize,
}