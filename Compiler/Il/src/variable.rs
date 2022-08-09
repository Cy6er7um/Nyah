use crate::r#type::TypeRef;
use serde::{Serialize, Deserialize};

pub type VariableRef = u32;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct Variable {
    pub r#type: TypeRef,
}
