use crate::r#type::TypeRef;
use serde::{Serialize, Deserialize};

pub type ValueRef = u32;

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct Value {
    pub r#type: TypeRef,
}
