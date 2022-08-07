use std::collections::HashMap;

use crate::function::{Function, FunctionRef};
use crate::global_data::{GlobalData, GlobalDataRef};
use crate::r#type::{Type, TypeRef};
use crate::static_char_array::{StaticCA, StaticCARef};

pub struct Project {
    pub functions: HashMap<FunctionRef, Function>,
    pub global_data_map: HashMap<GlobalDataRef, GlobalData>,
    pub static_char_arrays: HashMap<StaticCARef, StaticCA>,
    pub types: HashMap<TypeRef, Type>,
}
