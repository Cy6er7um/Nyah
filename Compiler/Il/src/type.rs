use serde::{Serialize, Deserialize};

pub type TypeRef = u32;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Type {
    // TODO: ??
    pub element_type: TypeBuiltin,
    pub element_count: u32,
}

#[repr(u32)]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum TypeBuiltin {
    I8 = 0,
    U8 = 1,
    I16 = 2,
    U16 = 3,
    I32 = 4,
    U32 = 5,
    I64 = 6,
    U64 = 7,
    F32 = 8,
    F64 = 9,
    C8 = 10,
    C32 = 11,
    B8= 12,
    Pointer = 13,
}
