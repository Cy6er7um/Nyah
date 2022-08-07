pub type TypeRef = u32;

pub struct Type {
    pub element_size: u32,
    pub element_count: u32,
}

#[repr(u32)]
pub enum TypeBuiltin {
    I8 = 0,
    U8 = 1,
    I16 = 2,
    U16 = 3,
    I32 = 4,
    U32 = 5,
    I64 = 6,
    U66 = 7,
    F32 = 8,
    F64 = 9,
    C8 = 10,
    C32 = 11,
}
