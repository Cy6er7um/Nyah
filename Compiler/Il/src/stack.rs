use crate::r#type::TypeRef;
use serde::{Serialize, Deserialize};

pub type StackRef = u32;

#[derive(Clone, Serialize, Deserialize)]
pub struct Stack {
    pub r#type: TypeRef,
}
