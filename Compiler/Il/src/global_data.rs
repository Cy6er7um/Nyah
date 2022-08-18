use crate::function::FunctionRef;
use crate::r#type::TypeRef;

pub type GlobalDataRef = u32;

pub struct GlobalData {
    pub name: Option<String>,
    pub r#type: TypeRef,
    pub data: GlobalDataInner,
    pub fill_global_data_address: Vec<(u32, GlobalDataRef)>,
    pub fill_function_address: Vec<(u32, FunctionRef)>,
}

pub enum GlobalDataInner {
    U8Box(Box<[u8]>),
    U8Vector(Vec<u8>),
    Empty(u32),
}

impl GlobalDataInner {
    pub fn size(&self) -> u32 {
        match &self {
            GlobalDataInner::U8Box(inner) => inner.len() as u32,
            GlobalDataInner::U8Vector(inner) => inner.len() as u32,
            GlobalDataInner::Empty(size) => *size,
        }
    }
}
