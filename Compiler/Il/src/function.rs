use std::collections::HashMap;

use crate::operation::Operation;
use crate::r#type::TypeRef;
use crate::value::{Value, ValueRef};
use crate::variable::{Variable, VariableRef};
use serde::{Serialize, Deserialize};

pub type FunctionRef = u32;

#[derive(Clone, Serialize, Deserialize)]
pub struct Function {
    pub name: Option<String>,
    pub conv: FunctionConv,
    pub param_types: Vec<TypeRef>,
    pub dynamic_param_type: Option<TypeRef>,
    pub dynamic_keyword_param_type: Option<TypeRef>,
    pub return_types: Vec<TypeRef>,
    pub variables: HashMap<VariableRef, Variable>,
    pub values: HashMap<ValueRef, Value>,
    pub body: Vec<Operation>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum FunctionConv {
    Fast,
    C,
}
