pub type TypeRef = u32;

pub struct Type {
    pub size: u32,
}

#[repr(u32)]
pub enum TypeBuiltin {
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    F32,
    F64,
    C8,
    C32,
    Array(Box<TypeRef>),
}
